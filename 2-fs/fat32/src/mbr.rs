use std::{fmt, io};
use std::mem::transmute_copy;

use traits::BlockDevice;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CHS {
    // FIXME: Fill me in.
    starting_head: u8,
    starting_sector_cylinder: u8,
    starting_cylinder: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PartitionEntry {
    // FIXME: Fill me in.
    boot_indicator: u8,
    starting_CHS: CHS,
    partition_type: u8,
    ending_CHS: CHS,
    relative_sector: u32,
    total_sectors: u32,
}

/// The master boot record (MBR).
#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct MasterBootRecord {
    // FIXME: Fill me in.
    MBR_bootstrap: [u8; 436],
    unique_disk_ID: [u8; 10],
    partitions: [PartitionEntry; 4],
    magic: u16,
}

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut buf = [0u8; 512];
        let size = device.read_sector(0, &mut buf)?;
        if size != 512 {
            return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "unable to cast this sector as MBR",
                    ).into());
        } else {
            let this_mbr: MasterBootRecord = unsafe {
                transmute_copy(&buf)
            };
            if this_mbr.magic != 0xaa55 {
                return Err(Error::BadSignature);
            }
            for i in 0..4 {
                if this_mbr.partitions[i].boot_indicator != 0x00
                    && this_mbr.partitions[i].boot_indicator != 0x80 {
                        return Err(Error::UnknownBootIndicator(i as u8));
                    }
            }
            return Ok(this_mbr);
        }
    }

    pub fn first_vfat_partition(&self) -> Option<u32> {
        for i in 0..4 {
            if self.partitions[i].partition_type == 0xb
                || self.partitions[i].partition_type == 0x0c {
                    return Some(self.partitions[i].relative_sector);
                }
        }
        None
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
            .field("partitions", &self.partitions)
            .finish()
    }
}


impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS")
            .field("starting_head", &self.starting_head)
            .field("starting_sector_cylinder", &self.starting_sector_cylinder)
            .field("starting_cylinder", &self.starting_cylinder)
            .finish()
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
