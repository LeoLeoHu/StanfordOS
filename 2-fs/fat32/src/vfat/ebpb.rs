use std::{fmt, io};
use std::mem::transmute_copy;

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct BiosParameterBlock {
    // FIXME: Fill me in.
    EB_XX_90: [u8; 3],
    OEM: [u8; 8],

    // DOS 2.0 BPB
    pub(super) bytes_per_logical_sector: u16,
    pub(super) logical_sectors_per_cluster: u8,
    pub(super) reserved_logical_sectors: u16,
    pub(super) num_of_FATs: u8,
    root_directory_entries: u16,
    total_logical_sectors: u16,
    media_descriptor: u8,
    logical_sectors_per_FAT_0: u16,

    // DOS 2.0 BPB + DOS 3.31 BPB
    physical_sectors_per_track: u16,
    num_of_heads: u16,
    hidden_sectors: u32,
    large_total_logical_sectors: u32,

    // DOS 2.0 BPB + DOS 3.31 BPB + DOS 7.1 EBPB
    logical_sectors_per_FAT_1: u32,
    mirroring_flag: u16,
    version: u16,
    pub(super) root_directory_cluster: u32,
    FSInfo: u16,
    backup_boot_sector: u16,
    _reserved: [u8; 12],
    physical_drive_number: u8,
    flags: u8,
    extended_boot_signature: u8,
    volume_serial_number: u32,
    volume_label: [u8; 11],
    FSType: [u8; 8],

    boot_code: [u8; 420],
    magic: u16,
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; 512];
        let size = device.read_sector(sector, &mut buf)?;
        if size != 512 {
            return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "unable to cast this sector as EBPB",
                    ).into());
        } else {
            let this_bpb: BiosParameterBlock = unsafe {
                transmute_copy(&buf)
            };
            if this_bpb.magic != 0xaa55 {
                return Err(Error::BadSignature);
            }
            return Ok(this_bpb);
        }
    }

    #[allow(non_snake_case)]
    pub fn logical_sectors_per_FAT(&self) -> u32 {
        if self.logical_sectors_per_FAT_0 > 0 {
            self.logical_sectors_per_FAT_0 as u32
        } else {
            self.logical_sectors_per_FAT_1
        }
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock").finish()
    }
}
