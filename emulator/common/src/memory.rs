pub type MemoryResult<T> = Result<T, MemoryError>;
#[derive(Copy, Clone, Debug)]
pub enum MemoryError {
    NoMatchingRegion,
    CrossRegionAccess,
}

pub struct Memory {
    regions: Vec<MemoryRegion>,
}
impl Memory {
    /// The memory regions must ensure the following:
    /// - There are no overlaps
    /// - The entire memory space is mapped
    pub fn new(regions: Vec<MemoryRegion>) -> Memory {
        Memory { regions }
    }

    pub fn read(&self, address: usize, buffer: &mut [u8]) -> MemoryResult<()> {
        let region = self.find_region(address)?;
        region.read(address, buffer)
    }
    pub fn write(&mut self, address: usize, buffer: &[u8]) -> MemoryResult<()> {
        let region = self.find_region_mut(address)?;
        region.write(address, buffer)
    }
}
impl Memory {
    fn find_region(&self, address: usize) -> MemoryResult<&MemoryRegion> {
        for region in &self.regions {
            if region.start_address <= address && address <= region.end_address() {
                return Ok(region);
            }
        }

        Err(MemoryError::NoMatchingRegion)
    }
    fn find_region_mut(&mut self, address: usize) -> MemoryResult<&mut MemoryRegion> {
        for region in &mut self.regions {
            if region.start_address <= address && address <= region.end_address() {
                return Ok(region);
            }
        }

        Err(MemoryError::NoMatchingRegion)
    }
}

// TODO Mirrored regions
#[derive(Eq, PartialEq)]
pub struct MemoryRegion {
    start_address: usize,
    bytes: Vec<u8>,
}
impl MemoryRegion {
    pub fn new(start_address: usize, bytes: Vec<u8>) -> MemoryRegion {
        MemoryRegion { start_address, bytes }
    }
}
impl MemoryRegion {
    #[inline]
    fn end_address(&self) -> usize { self.start_address + self.bytes.len() }

    /// Can only be called if the address is guaranteed to exist in here
    fn read(&self, address: usize, buffer: &mut [u8]) -> MemoryResult<()> {
        let start_index = address - self.start_address;
        let end_length = start_index + buffer.len();
        if end_length <= self.bytes.len() {
            buffer.copy_from_slice(&self.bytes[start_index..end_length]);
            Ok(())
        } else {
            Err(MemoryError::CrossRegionAccess)
        }
    }
    /// Can only be called if the address is guaranteed to exist in here
    fn write(&mut self, address: usize, buffer: &[u8]) -> MemoryResult<()> {
        let start_index = address - self.start_address;
        let end_length = start_index + buffer.len();
        if end_length <= self.bytes.len() {
            self.bytes[start_index..end_length].copy_from_slice(buffer);
            Ok(())
        } else {
            Err(MemoryError::CrossRegionAccess)
        }
    }
}
