use std::{
    collections::{BTreeMap},
};

pub type MemoryResult<T> = Result<T, MemoryError>;
#[derive(Copy, Clone, Debug)]
pub enum MemoryError {
    InvalidRead,
    InvalidWrite,

    NoMatchingRegion,
    CrossRegionAccess,
}

pub struct Memory<A: Copy + Ord> {
    regions: Vec< MemoryRegion<A> >,
    current_access: A,
}
impl <A: Copy + Ord> Memory<A> {
    /// The memory regions must ensure the following:
    /// - There are no overlaps
    /// - The entire memory space is mapped
    pub fn new(regions: Vec< MemoryRegion<A> >, current_access: A) -> Memory<A> {
        Memory { regions, current_access }
    }

    pub fn set_access(&mut self, access: A) { self.current_access = access; }

    pub fn read(&self, address: usize, buffer: &mut [u8]) -> MemoryResult<()> {
        let buffer_len = buffer.len();
        let region = self.find_region(address)?;
        if region == self.find_region(address + buffer_len)? {
            if region.can_read(self.current_access) {
                buffer.copy_from_slice(&region.bytes[address..(address + buffer_len)]);
                Ok(())
            } else {
                Err(MemoryError::InvalidRead)
            }
        } else {
            Err(MemoryError::CrossRegionAccess)
        }
    }
    pub fn write(&mut self, address: usize, buffer: &[u8]) -> MemoryResult<()> {
        let buffer_len = buffer.len();
        if self.find_region(address)? == self.find_region(address + buffer_len)? {
            let current_access = self.current_access;
            let region = self.find_region_mut(address)?;
            if region.can_write(current_access) {
                region.bytes[address..(address + buffer_len)].copy_from_slice(buffer);
                Ok(())
            } else {
                Err(MemoryError::InvalidWrite)
            }
        } else {
            Err(MemoryError::CrossRegionAccess)
        }
    }
}
impl <A: Copy + Ord> Memory<A> {
    fn find_region(&self, address: usize) -> MemoryResult<&MemoryRegion<A>> {
        for region in &self.regions {
            if region.start_index <= address && address <= region.end_index() {
                return Ok(region);
            }
        }

        Err(MemoryError::NoMatchingRegion)
    }
    fn find_region_mut(&mut self, address: usize) -> MemoryResult<&mut MemoryRegion<A>> {
        for region in &mut self.regions {
            if region.start_index <= address && address <= region.end_index() {
                return Ok(region);
            }
        }

        Err(MemoryError::NoMatchingRegion)
    }
}

#[derive(Eq, PartialEq)]
pub struct MemoryRegion<A: Copy + Ord> {
    start_index: usize,
    bytes: Vec<u8>,
    permissions: BTreeMap<A, MemoryPermission>,
}
impl <A: Copy + Ord> MemoryRegion<A> {
    pub fn new(start_index: usize, bytes: Vec<u8>, permissions: &[(A, MemoryPermission)])
    -> MemoryRegion<A> {
        let permissions = permissions.iter()
            .map(|(access, permission)| (*access, *permission))
            .collect();
        MemoryRegion {
            start_index,
            bytes,
            permissions,
        }
    }
}
impl <A: Copy + Ord> MemoryRegion<A> {
    fn end_index(&self) -> usize { self.start_index + self.bytes.len() }

    fn can_read(&self, access: A) -> bool {
        self.permissions.get(&access)
            .map_or(false, |permission| permission.can_read())
    }
    fn can_write(&self, access: A) -> bool {
        self.permissions.get(&access)
            .map_or(false, |permission| permission.can_write())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MemoryPermission {
    CannotAccess,
    ReadOnly,
    ReadWrite,
}
impl MemoryPermission {
    fn can_read(&self) -> bool {
        match self {
            Self::ReadOnly | Self::ReadWrite => true,
            Self::CannotAccess => false,
        }
    }
    fn can_write(&self) -> bool {
        match self {
            Self::ReadWrite => true,
            Self::ReadOnly | Self::CannotAccess => false,
        }
    }
}
