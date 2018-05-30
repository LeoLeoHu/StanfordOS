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
        let sectors_per_fat = ebpb.logical_sectors_per_FAT();
        let cached_device = CachedDevice::new(
            device,
            Partition{
                start: fat_start_sector,
                sector_size: ebpb.bytes_per_logical_sector as u64,
            },
            );

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

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    //    fn read_cluster(
    //        &mut self,
    //        cluster: Cluster,
    //        offset: usize,
    //        buf: &mut [u8]
    //    ) -> io::Result<usize>;
    //
    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    //    fn read_chain(
    //        &mut self,
    //        start: Cluster,
    //        buf: &mut Vec<u8>
    //    ) -> io::Result<usize>;
    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    //    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry>;
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
