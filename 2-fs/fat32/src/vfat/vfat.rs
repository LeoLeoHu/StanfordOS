use std::io;
use std::path::Path;
use std::mem::size_of;
use std::cmp::min;

use util::SliceExt;
use mbr::MasterBootRecord;
use vfat::{Shared, Cluster, File, Dir, Entry, FatEntry, Error, Status};
use vfat::{BiosParameterBlock, CachedDevice, Partition};
use traits::{FileSystem, BlockDevice};

#[derive(Debug)]
pub struct VFat {
    device: CachedDevice,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
        where T: BlockDevice + 'static
    {
        let mbr = MasterBootRecord::from(&mut device)?;
        let fat_start_sector = match mbr.fat_start_sector() {
            Some(sector) => sector as u64,
            None => {
                return Err(Error::Io(io::Error::new(
                            io::ErrorKind::NotFound,
                            "cannot find first vfat partition",
                            )));
            },
        };
        let ebpb = BiosParameterBlock::from(&mut device, fat_start_sector)?;
        if ebpb.bytes_per_logical_sector == 0 || ebpb.bytes_per_logical_sector % device.sector_size() as u16 != 0 {
            return Err(Error::Io(io::Error::new(
                        io::ErrorKind::Other,
                        "unsupported logical sector size",
                        )));
        }
        let cached_device = CachedDevice::new(
            device,
            Partition{
                start: fat_start_sector,
                sector_size: ebpb.bytes_per_logical_sector as u64,
            },
            );
        let sectors_per_fat = ebpb.logical_sectors_per_FAT();

        Ok(Shared::new(VFat{
            device: cached_device,
            bytes_per_sector: ebpb.bytes_per_logical_sector,
            sectors_per_cluster: ebpb.logical_sectors_per_cluster,
            sectors_per_fat: sectors_per_fat,
            fat_start_sector: fat_start_sector + ebpb.reserved_logical_sectors as u64,
            data_start_sector: fat_start_sector + ebpb.reserved_logical_sectors as u64
                + sectors_per_fat as u64 * ebpb.num_of_FATs as u64,
            root_dir_cluster: Cluster::from(ebpb.root_directory_cluster),
        }))

    }

    //  * A method to read from an offset of a cluster into a buffer.
    fn read_cluster(&mut self, cluster: Cluster, offset: usize, buf: &mut [u8]) -> io::Result<usize> {
        if !cluster.is_valid() {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid cluster number",
                    ).into());
        }
        let sector_size = self.device.sector_size() as usize;
        let bytes_need_to_read = min(
            self.device.sector_size() as usize * self.sectors_per_cluster as usize - offset,
            buf.len());

        let mut bytes_offset_remain = offset % self.device.sector_size() as usize;
        let mut sector_to_read = self.data_start_sector + cluster.cluster_index() as u64 * self.sectors_per_cluster as u64
            - offset as u64 % self.device.sector_size() as u64;
        let mut bytes_read = 0;

        while bytes_read < bytes_need_to_read {
            let this_copy = self.device.get(sector_to_read)?;

            // bytes_need_to_read can be less than device.sector_size()
            // there is no offset after first loop
            // but still need to decide whether to copy the entire sector
            let bytes_this_copy = min(bytes_need_to_read - bytes_read, sector_size - bytes_offset_remain);
            
            buf[bytes_read..bytes_read + bytes_this_copy].copy_from_slice(
                &this_copy[bytes_offset_remain..bytes_offset_remain + bytes_this_copy]);

            bytes_offset_remain = 0;
            bytes_read += bytes_this_copy;
            sector_to_read += 1;
        }
        Ok(bytes_read)
    }

    fn next_cluster(&mut self, cluster: Cluster) -> io::Result<Option<Cluster>> {
        if !cluster.is_valid() {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid cluster number",
                    ).into());
        }

        match self.fat_entry(cluster)?.status() {
            Status::Eoc(_) => Ok(None),
            Status::Data(next_cluster) => {
                if !next_cluster.is_valid() {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Invalid next cluster number",
                            ).into());
                } else{ 
                    Ok(Some(next_cluster))
                }
            }
            _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid next cluster number",
                    )),
        }
    }

    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    fn read_chain(&mut self, start: Cluster, buf: &mut Vec<u8>) -> io::Result<usize> {
        unimplemented!();
    }

    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    pub(super) fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        // cluster corresponds to its FatEntry successively
        // still, cluster does not corresponds to the value of its FatEntry
        let sector_this_fatentry: u64 = cluster.cluster_num() as u64 * size_of::<FatEntry>() as u64
            / self.bytes_per_sector as u64;
        let bytes_remain_this_fatentry: usize = cluster.cluster_num() as usize * size_of::<FatEntry>()
            % self.bytes_per_sector as usize;
        let content = self.device.get(self.fat_start_sector + sector_this_fatentry)?;
        let entries: &[FatEntry] = unsafe {
            content.cast()
        };
        // several FatEntries in this content sector
        Ok(&entries[bytes_remain_this_fatentry / size_of::<FatEntry>()])
    }
        
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = ::traits::Dummy;
    type Dir = ::traits::Dummy;
    type Entry = ::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        unimplemented!("FileSystem::open()")
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
        where P: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
        where P: AsRef<Path>, Q: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
