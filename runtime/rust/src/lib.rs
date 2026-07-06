#![no_std]

use core::convert::{AsRef, AsMut};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
}

// Typestate Markers
pub struct UncheckedState;
pub struct MinimallyCompleteState; // New state!
pub struct CompleteState;
pub struct OkState;
pub struct AlwaysCompleteState;
pub struct OkAndAlwaysCompleteState;

pub trait State {
    type OnLayoutMutation: State;
    type OkVersion: State;
}

impl State for UncheckedState {
    type OnLayoutMutation = UncheckedState;
    type OkVersion = OkState;
}

impl State for MinimallyCompleteState {
    type OnLayoutMutation = MinimallyCompleteState;
    type OkVersion = OkState;
}

impl State for CompleteState {
    type OnLayoutMutation = MinimallyCompleteState;
    type OkVersion = OkState;
}

impl State for OkState {
    type OnLayoutMutation = MinimallyCompleteState;
    type OkVersion = OkState;
}

impl State for AlwaysCompleteState {
    type OnLayoutMutation = AlwaysCompleteState;
    type OkVersion = OkAndAlwaysCompleteState;
}

impl State for OkAndAlwaysCompleteState {
    type OnLayoutMutation = AlwaysCompleteState;
    type OkVersion = OkAndAlwaysCompleteState;
}

pub trait IsMinimallyComplete: State {}
impl IsMinimallyComplete for MinimallyCompleteState {}
impl IsMinimallyComplete for CompleteState {}
impl IsMinimallyComplete for OkState {}
impl IsMinimallyComplete for AlwaysCompleteState {}
impl IsMinimallyComplete for OkAndAlwaysCompleteState {}

pub trait IsComplete: IsMinimallyComplete {}
impl IsComplete for CompleteState {}
impl IsComplete for OkState {}
impl IsComplete for AlwaysCompleteState {}
impl IsComplete for OkAndAlwaysCompleteState {}

pub trait IsOk: IsComplete {}
impl IsOk for OkState {}
impl IsOk for OkAndAlwaysCompleteState {}

pub trait EmbossView {
    type Storage: Storage;
    type State: State;
    fn emboss_reserved_get_storage(&self) -> &Self::Storage;
    fn size_in_bytes(&self) -> Result<usize, Error>;
    fn min_size_in_bytes() -> usize;
    fn max_size_in_bytes() -> usize;
}

pub trait EmbossMutView: EmbossView {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut Self::Storage;
}

impl<'a, T: EmbossView> EmbossView for &'a T {
    type Storage = T::Storage;
    type State = T::State;
    fn emboss_reserved_get_storage(&self) -> &Self::Storage { (**self).emboss_reserved_get_storage() }
    fn size_in_bytes(&self) -> Result<usize, Error> { (**self).size_in_bytes() }
    fn min_size_in_bytes() -> usize { T::min_size_in_bytes() }
    fn max_size_in_bytes() -> usize { T::max_size_in_bytes() }
}

impl<'a, T: EmbossView> EmbossView for &'a mut T {
    type Storage = T::Storage;
    type State = T::State;
    fn emboss_reserved_get_storage(&self) -> &Self::Storage { (**self).emboss_reserved_get_storage() }
    fn size_in_bytes(&self) -> Result<usize, Error> { (**self).size_in_bytes() }
    fn min_size_in_bytes() -> usize { T::min_size_in_bytes() }
    fn max_size_in_bytes() -> usize { T::max_size_in_bytes() }
}

impl<'a, T: EmbossMutView> EmbossMutView for &'a mut T {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut Self::Storage { (**self).emboss_reserved_get_storage_mut() }
}

pub trait Storage {
    type Slice<'a>: Storage where Self: 'a;
    fn slice(&self, range: core::ops::Range<usize>) -> Self::Slice<'_>;

    fn read_le_u8(&self, offset: usize) -> Result<u8, Error>;
    fn read_le_u16(&self, offset: usize) -> Result<u16, Error>;
    fn read_le_u32(&self, offset: usize) -> Result<u32, Error>;
    fn read_le_u64(&self, offset: usize) -> Result<u64, Error>;

    unsafe fn read_le_u8_unchecked(&self, offset: usize) -> u8;
    unsafe fn read_le_u16_unchecked(&self, offset: usize) -> u16;
    unsafe fn read_le_u32_unchecked(&self, offset: usize) -> u32;
    unsafe fn read_le_u64_unchecked(&self, offset: usize) -> u64;
    
    fn len(&self) -> usize;
}

pub struct BorrowedStorage<'a, S: ?Sized>(pub &'a S);

impl<'a, S: Storage + ?Sized> Storage for BorrowedStorage<'a, S> {
    type Slice<'b> = S::Slice<'b> where Self: 'b;
    fn slice(&self, range: core::ops::Range<usize>) -> Self::Slice<'_> {
        self.0.slice(range)
    }
    fn read_le_u8(&self, offset: usize) -> Result<u8, Error> {
        self.0.read_le_u8(offset)
    }
    fn read_le_u16(&self, offset: usize) -> Result<u16, Error> {
        self.0.read_le_u16(offset)
    }
    fn read_le_u32(&self, offset: usize) -> Result<u32, Error> {
        self.0.read_le_u32(offset)
    }
    fn read_le_u64(&self, offset: usize) -> Result<u64, Error> {
        self.0.read_le_u64(offset)
    }
    unsafe fn read_le_u8_unchecked(&self, offset: usize) -> u8 {
        self.0.read_le_u8_unchecked(offset)
    }
    unsafe fn read_le_u16_unchecked(&self, offset: usize) -> u16 {
        self.0.read_le_u16_unchecked(offset)
    }
    unsafe fn read_le_u32_unchecked(&self, offset: usize) -> u32 {
        self.0.read_le_u32_unchecked(offset)
    }
    unsafe fn read_le_u64_unchecked(&self, offset: usize) -> u64 {
        self.0.read_le_u64_unchecked(offset)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: AsRef<[u8]>> Storage for T {
    type Slice<'a> = &'a [u8] where Self: 'a;
    fn slice(&self, range: core::ops::Range<usize>) -> Self::Slice<'_> {
        &self.as_ref()[range]
    }

    fn read_le_u8(&self, offset: usize) -> Result<u8, Error> {
        let slice = self.as_ref();
        slice.get(offset).copied().ok_or(Error::OutOfBounds)
    }

    fn read_le_u16(&self, offset: usize) -> Result<u16, Error> {
        let slice = self.as_ref();
        let src = slice.get(offset..offset + 2).ok_or(Error::OutOfBounds)?;
        Ok(u16::from_le_bytes(src.try_into().unwrap()))
    }

    fn read_le_u32(&self, offset: usize) -> Result<u32, Error> {
        let slice = self.as_ref();
        let src = slice.get(offset..offset + 4).ok_or(Error::OutOfBounds)?;
        Ok(u32::from_le_bytes(src.try_into().unwrap()))
    }

    fn read_le_u64(&self, offset: usize) -> Result<u64, Error> {
        let slice = self.as_ref();
        let src = slice.get(offset..offset + 8).ok_or(Error::OutOfBounds)?;
        Ok(u64::from_le_bytes(src.try_into().unwrap()))
    }

    unsafe fn read_le_u8_unchecked(&self, offset: usize) -> u8 {
        *self.as_ref().get_unchecked(offset)
    }

    unsafe fn read_le_u16_unchecked(&self, offset: usize) -> u16 {
        let src = self.as_ref().get_unchecked(offset..offset + 2);
        u16::from_le_bytes(src.try_into().unwrap())
    }

    unsafe fn read_le_u32_unchecked(&self, offset: usize) -> u32 {
        let src = self.as_ref().get_unchecked(offset..offset + 4);
        u32::from_le_bytes(src.try_into().unwrap())
    }

    unsafe fn read_le_u64_unchecked(&self, offset: usize) -> u64 {
        let src = self.as_ref().get_unchecked(offset..offset + 8);
        u64::from_le_bytes(src.try_into().unwrap())
    }
    
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

pub trait MutStorage: Storage {
    type MutSlice<'a>: MutStorage where Self: 'a;
    fn slice_mut(&mut self, range: core::ops::Range<usize>) -> Self::MutSlice<'_>;


    fn write_le_u8(&mut self, offset: usize, value: u8) -> Result<(), Error>;
    fn write_le_u16(&mut self, offset: usize, value: u16) -> Result<(), Error>;
    fn write_le_u32(&mut self, offset: usize, value: u32) -> Result<(), Error>;
    fn write_le_u64(&mut self, offset: usize, value: u64) -> Result<(), Error>;

    unsafe fn write_le_u8_unchecked(&mut self, offset: usize, value: u8);
    unsafe fn write_le_u16_unchecked(&mut self, offset: usize, value: u16);
    unsafe fn write_le_u32_unchecked(&mut self, offset: usize, value: u32);
    unsafe fn write_le_u64_unchecked(&mut self, offset: usize, value: u64);
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> MutStorage for T {
    type MutSlice<'a> = &'a mut [u8] where Self: 'a;
    fn slice_mut(&mut self, range: core::ops::Range<usize>) -> Self::MutSlice<'_> {
        &mut self.as_mut()[range]
    }


    fn write_le_u8(&mut self, offset: usize, value: u8) -> Result<(), Error> {
        let slice = self.as_mut();
        let cell = slice.get_mut(offset).ok_or(Error::OutOfBounds)?;
        *cell = value;
        Ok(())
    }

    fn write_le_u16(&mut self, offset: usize, value: u16) -> Result<(), Error> {
        let slice = self.as_mut();
        let dst = slice.get_mut(offset..offset + 2).ok_or(Error::OutOfBounds)?;
        dst.copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn write_le_u32(&mut self, offset: usize, value: u32) -> Result<(), Error> {
        let slice = self.as_mut();
        let dst = slice.get_mut(offset..offset + 4).ok_or(Error::OutOfBounds)?;
        dst.copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn write_le_u64(&mut self, offset: usize, value: u64) -> Result<(), Error> {
        let slice = self.as_mut();
        let dst = slice.get_mut(offset..offset + 8).ok_or(Error::OutOfBounds)?;
        dst.copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    unsafe fn write_le_u8_unchecked(&mut self, offset: usize, value: u8) {
        *self.as_mut().get_unchecked_mut(offset) = value;
    }

    unsafe fn write_le_u16_unchecked(&mut self, offset: usize, value: u16) {
        let dst = self.as_mut().get_unchecked_mut(offset..offset + 2);
        dst.copy_from_slice(&value.to_le_bytes());
    }

    unsafe fn write_le_u32_unchecked(&mut self, offset: usize, value: u32) {
        let dst = self.as_mut().get_unchecked_mut(offset..offset + 4);
        dst.copy_from_slice(&value.to_le_bytes());
    }

    unsafe fn write_le_u64_unchecked(&mut self, offset: usize, value: u64) {
        let dst = self.as_mut().get_unchecked_mut(offset..offset + 8);
        dst.copy_from_slice(&value.to_le_bytes());
    }
}

/*
pub struct BitStorage<S> {
    storage: S,
    bit_offset: usize,
}

impl<S: Storage> BitStorage<S> {
    pub fn new(storage: S, bit_offset: usize) -> Self {
        Self { storage, bit_offset }
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn read_bits(&self, bit_offset: usize, bit_size: usize) -> Result<u64, Error> {
        let total_bit_offset = self.bit_offset + bit_offset;
        let byte_offset = total_bit_offset / 8;
        let bit_in_byte = total_bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        if bytes_to_read > 8 {
            return Err(Error::OutOfBounds);
        }
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8(byte_offset + i)? as u64;
            val |= b << (i * 8);
        }
        
        let shifted = val >> bit_in_byte;
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        Ok(shifted & mask)
    }
}

impl<S: MutStorage> BitStorage<S> {
    pub fn write_bits(&mut self, bit_offset: usize, bit_size: usize, value: u64) -> Result<(), Error> {
        let total_bit_offset = self.bit_offset + bit_offset;
        let byte_offset = total_bit_offset / 8;
        let bit_in_byte = total_bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        if bytes_to_read > 8 {
            return Err(Error::OutOfBounds);
        }
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8(byte_offset + i)? as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let masked_val = val & !(mask << bit_in_byte);
        let new_val = masked_val | ((value & mask) << bit_in_byte);
        
        for i in 0..bytes_to_read {
            let b = (new_val >> (i * 8)) as u8;
            self.storage.write_le_u8(byte_offset + i, b)?;
        }
        
        Ok(())
    }

    pub unsafe fn write_bits_unchecked(&mut self, bit_offset: usize, bit_size: usize, value: u64) {
        let total_bit_offset = self.bit_offset + bit_offset;
        let byte_offset = total_bit_offset / 8;
        let bit_in_byte = total_bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8_unchecked(byte_offset + i) as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let masked_val = val & !(mask << bit_in_byte);
        let new_val = masked_val | ((value & mask) << bit_in_byte);
        
        for i in 0..bytes_to_read {
            let b = (new_val >> (i * 8)) as u8;
            self.storage.write_le_u8_unchecked(byte_offset + i, b);
        }
    }
}

impl<S: MutStorage> MutStorage for BitStorage<S> {
    type MutSlice<'a> = BitStorage<S::MutSlice<'a>> where Self: 'a;
    fn slice_mut(&mut self, range: core::ops::Range<usize>) -> Self::MutSlice<'_> {
        BitStorage::new(self.storage.slice_mut(range), self.bit_offset)
    }
    fn write_le_u8(&mut self, offset: usize, value: u8) -> Result<(), Error> { self.storage.write_le_u8(offset, value) }
    fn write_le_u16(&mut self, offset: usize, value: u16) -> Result<(), Error> { self.storage.write_le_u16(offset, value) }
    fn write_le_u32(&mut self, offset: usize, value: u32) -> Result<(), Error> { self.storage.write_le_u32(offset, value) }
    fn write_le_u64(&mut self, offset: usize, value: u64) -> Result<(), Error> { self.storage.write_le_u64(offset, value) }

    unsafe fn write_le_u8_unchecked(&mut self, offset: usize, value: u8) { self.storage.write_le_u8_unchecked(offset, value) }
    unsafe fn write_le_u16_unchecked(&mut self, offset: usize, value: u16) { self.storage.write_le_u16_unchecked(offset, value) }
}

impl<S: Storage> Storage for BitStorage<S> {
    type Slice<'a> = BitStorage<S::Slice<'a>> where Self: 'a;
    
    fn slice(&self, range: core::ops::Range<usize>) -> Self::Slice<'_> {
        BitStorage {
            storage: self.storage.slice(range),
            bit_offset: self.bit_offset, // Simplified for PoC
        }
    }
    
    fn read_le_u8(&self, offset: usize) -> Result<u8, Error> { self.storage.read_le_u8(offset) }
    fn read_le_u16(&self, offset: usize) -> Result<u16, Error> { self.storage.read_le_u16(offset) }
    fn read_le_u32(&self, offset: usize) -> Result<u32, Error> { self.storage.read_le_u32(offset) }
    fn read_le_u64(&self, offset: usize) -> Result<u64, Error> { self.storage.read_le_u64(offset) }
    unsafe fn read_le_u8_unchecked(&self, offset: usize) -> u8 { self.storage.read_le_u8_unchecked(offset) }
    unsafe fn read_le_u16_unchecked(&self, offset: usize) -> u16 { self.storage.read_le_u16_unchecked(offset) }
    fn len(&self) -> usize { self.storage.len() }
}
*/

#[derive(Clone, Copy)]
pub struct FieldLayout {
    pub offset: usize,
    pub size: usize,
}

pub trait CoerceAs<T> {
    fn coerce_as(self) -> T;
}

impl CoerceAs<u8> for u64 { fn coerce_as(self) -> u8 { self as u8 } }
impl CoerceAs<u16> for u64 { fn coerce_as(self) -> u16 { self as u16 } }
impl CoerceAs<u32> for u64 { fn coerce_as(self) -> u32 { self as u32 } }
impl CoerceAs<u64> for u64 { fn coerce_as(self) -> u64 { self } }

impl CoerceAs<i8> for u64 { fn coerce_as(self) -> i8 { self as i8 } }
impl CoerceAs<i16> for u64 { fn coerce_as(self) -> i16 { self as i16 } }
impl CoerceAs<i32> for u64 { fn coerce_as(self) -> i32 { self as i32 } }
impl CoerceAs<i64> for u64 { fn coerce_as(self) -> i64 { self as i64 } }

pub trait ReadableFromStorage: Sized {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error>;
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self;
}

impl ReadableFromStorage for u8 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u8(offset) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u8_unchecked(offset) }
}
impl ReadableFromStorage for u16 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u16(offset) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u16_unchecked(offset) }
}
impl ReadableFromStorage for u32 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u32(offset) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u32_unchecked(offset) }
}
impl ReadableFromStorage for u64 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u64(offset) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u64_unchecked(offset) }
}

impl ReadableFromStorage for i8 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u8(offset).map(|v| v as i8) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u8_unchecked(offset) as i8 }
}
impl ReadableFromStorage for i16 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u16(offset).map(|v| v as i16) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u16_unchecked(offset) as i16 }
}
impl ReadableFromStorage for i32 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u32(offset).map(|v| v as i32) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u32_unchecked(offset) as i32 }
}
impl ReadableFromStorage for i64 {
    fn read_from_storage<S: Storage + ?Sized>(storage: &S, offset: usize) -> Result<Self, Error> { storage.read_le_u64(offset).map(|v| v as i64) }
    unsafe fn read_from_storage_unchecked<S: Storage + ?Sized>(storage: &S, offset: usize) -> Self { storage.read_le_u64_unchecked(offset) as i64 }
}

pub trait WritableToStorage {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error>;
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize);
}

impl WritableToStorage for u8 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u8(offset, self) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u8_unchecked(offset, self) }
}
impl WritableToStorage for u16 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u16(offset, self) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u16_unchecked(offset, self) }
}
impl WritableToStorage for u32 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u32(offset, self) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u32_unchecked(offset, self) }
}
impl WritableToStorage for u64 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u64(offset, self) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u64_unchecked(offset, self) }
}

impl WritableToStorage for i8 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u8(offset, self as u8) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u8_unchecked(offset, self as u8) }
}
impl WritableToStorage for i16 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u16(offset, self as u16) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u16_unchecked(offset, self as u16) }
}
impl WritableToStorage for i32 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u32(offset, self as u32) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u32_unchecked(offset, self as u32) }
}
impl WritableToStorage for i64 {
    fn write_to_storage<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) -> Result<(), Error> { storage.write_le_u64(offset, self as u64) }
    unsafe fn write_to_storage_unchecked<S: MutStorage + ?Sized>(self, storage: &mut S, offset: usize) { storage.write_le_u64_unchecked(offset, self as u64) }
}

pub struct FieldReference<Parent, T> {
    pub parent: Parent,
    pub layout: FieldLayout,
    pub _phantom: core::marker::PhantomData<T>,
}

impl<Parent, T> FieldReference<Parent, T> {
    pub fn new(parent: Parent, layout: FieldLayout) -> Self {
        Self { parent, layout, _phantom: core::marker::PhantomData }
    }
}

pub struct FieldWriter<Parent, T> {
    pub parent: Parent,
    pub layout: FieldLayout,
    pub _phantom: core::marker::PhantomData<T>,
}

impl<Parent, T> FieldWriter<Parent, T> {
    pub fn new(parent: Parent, layout: FieldLayout) -> Self {
        Self { parent, layout, _phantom: core::marker::PhantomData }
    }
}


impl<Parent: EmbossView, T: ReadableFromStorage> FieldReference<Parent, T> {
    pub fn try_read(&self) -> Result<T, Error> {
        T::read_from_storage(self.parent.emboss_reserved_get_storage(), self.layout.offset)
    }

    pub unsafe fn read_unchecked(&self) -> T {
        T::read_from_storage_unchecked(self.parent.emboss_reserved_get_storage(), self.layout.offset)
    }
}

impl<Parent: EmbossView, T: ReadableFromStorage> FieldReference<Parent, T>
where Parent::State: IsMinimallyComplete {
    pub fn read(&self) -> T {
        unsafe { self.read_unchecked() }
    }
}

impl<Parent: EmbossMutView, T: WritableToStorage> FieldWriter<Parent, T> 
where Parent::Storage: MutStorage {
    pub fn try_write(mut self, value: T) -> Result<Parent, Error> {
        value.write_to_storage(self.parent.emboss_reserved_get_storage_mut(), self.layout.offset)?;
        Ok(self.parent)
    }

    pub unsafe fn write_unchecked(mut self, value: T) -> Parent {
        value.write_to_storage_unchecked(self.parent.emboss_reserved_get_storage_mut(), self.layout.offset);
        self.parent
    }
}

impl<Parent: EmbossMutView, T: WritableToStorage> FieldWriter<Parent, T>
where Parent::Storage: MutStorage, Parent::State: IsMinimallyComplete {
    pub fn write(self, value: T) -> Parent {
        unsafe { self.write_unchecked(value) }
    }
}

pub struct Bits<'a, S: ?Sized> {
    pub storage: &'a S,
}

impl<'a, S: Storage + ?Sized> Bits<'a, S> {
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn read_bits(&self, layout: FieldLayout) -> Result<u64, Error> {
        let bit_offset = layout.offset;
        let bit_size = layout.size;
        let byte_offset = bit_offset / 8;
        let bit_in_byte = bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        if bytes_to_read > 8 {
            return Err(Error::OutOfBounds);
        }
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8(byte_offset + i)? as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let shifted = val >> bit_in_byte;
        Ok(shifted & mask)
    }

    pub unsafe fn read_bits_unchecked(&self, layout: FieldLayout) -> u64 {
        let bit_offset = layout.offset;
        let bit_size = layout.size;
        let byte_offset = bit_offset / 8;
        let bit_in_byte = bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8_unchecked(byte_offset + i) as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let shifted = val >> bit_in_byte;
        shifted & mask
    }
}

pub struct BitsMut<'a, S: ?Sized> {
    pub storage: &'a mut S,
}

impl<'a, S: MutStorage + ?Sized> BitsMut<'a, S> {
    pub fn write_bits(&mut self, layout: FieldLayout, value: u64) -> Result<(), Error> {
        let bit_offset = layout.offset;
        let bit_size = layout.size;
        let byte_offset = bit_offset / 8;
        let bit_in_byte = bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        if bytes_to_read > 8 {
            return Err(Error::OutOfBounds);
        }
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8(byte_offset + i)? as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let masked_val = val & !(mask << bit_in_byte);
        let new_val = masked_val | ((value & mask) << bit_in_byte);
        
        for i in 0..bytes_to_read {
            let b = (new_val >> (i * 8)) as u8;
            self.storage.write_le_u8(byte_offset + i, b)?;
        }
        
        Ok(())
    }

    pub unsafe fn write_bits_unchecked(&mut self, layout: FieldLayout, value: u64) {
        let bit_offset = layout.offset;
        let bit_size = layout.size;
        let byte_offset = bit_offset / 8;
        let bit_in_byte = bit_offset % 8;
        
        let bytes_to_read = (bit_size + bit_in_byte + 7) / 8;
        
        let mut val: u64 = 0;
        for i in 0..bytes_to_read {
            let b = self.storage.read_le_u8_unchecked(byte_offset + i) as u64;
            val |= b << (i * 8);
        }
        
        let mask = if bit_size == 64 {
            core::u64::MAX
        } else {
            (1u64 << bit_size) - 1
        };
        
        let masked_val = val & !(mask << bit_in_byte);
        let new_val = masked_val | ((value & mask) << bit_in_byte);
        
        for i in 0..bytes_to_read {
            let b = (new_val >> (i * 8)) as u8;
            self.storage.write_le_u8_unchecked(byte_offset + i, b);
        }
    }
}

pub struct BitReference<Parent, T> {
    pub parent: Parent,
    pub layout: FieldLayout,
    pub _phantom: core::marker::PhantomData<T>,
}

impl<Parent, T> BitReference<Parent, T> {
    pub fn new(parent: Parent, layout: FieldLayout) -> Self {
        Self { parent, layout, _phantom: core::marker::PhantomData }
    }
}

impl<Parent: EmbossView, T> BitReference<Parent, T> 
where u64: CoerceAs<T> {
    pub fn try_read(&self) -> Result<T, Error> {
        let bits = Bits { storage: self.parent.emboss_reserved_get_storage() };
        bits.read_bits(self.layout).map(|v| v.coerce_as())
    }

    pub unsafe fn read_unchecked(&self) -> T {
        let bits = Bits { storage: self.parent.emboss_reserved_get_storage() };
        bits.read_bits_unchecked(self.layout).coerce_as()
    }
}

impl<Parent: EmbossView, T> BitReference<Parent, T>
where u64: CoerceAs<T>, Parent::State: IsMinimallyComplete {
    pub fn read(&self) -> T {
        unsafe { self.read_unchecked() }
    }
}

pub struct BitWriter<Parent, T> {
    pub parent: Parent,
    pub layout: FieldLayout,
    pub _phantom: core::marker::PhantomData<T>,
}

impl<Parent, T> BitWriter<Parent, T> {
    pub fn new(parent: Parent, layout: FieldLayout) -> Self {
        Self { parent, layout, _phantom: core::marker::PhantomData }
    }
}


impl<Parent: EmbossMutView, T: Into<u64>> BitWriter<Parent, T> 
where Parent::Storage: MutStorage {
    pub fn try_write(mut self, value: T) -> Result<Parent, Error> {
        let mut bits = BitsMut { storage: self.parent.emboss_reserved_get_storage_mut() };
        bits.write_bits(self.layout, value.into())?;
        Ok(self.parent)
    }

    pub unsafe fn write_unchecked(mut self, value: T) -> Parent {
        let mut bits = BitsMut { storage: self.parent.emboss_reserved_get_storage_mut() };
        bits.write_bits_unchecked(self.layout, value.into());
        self.parent
    }
}

impl<Parent: EmbossMutView, T: Into<u64>> BitWriter<Parent, T>
where Parent::Storage: MutStorage, Parent::State: IsMinimallyComplete {
    pub fn write(self, value: T) -> Parent {
        unsafe { self.write_unchecked(value) }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_u8() {
        let mut buf = [0u8; 1];
        let mut mut_slice: &mut [u8] = &mut buf[..];
        unsafe { mut_slice.write_le_u8_unchecked(0, 42); }
        let slice: &[u8] = &buf[..];
        assert_eq!(unsafe { slice.read_le_u8_unchecked(0) }, 42);
    }

    #[test]
    fn test_read_write_u16() {
        let mut buf = [0u8; 2];
        let mut mut_slice: &mut [u8] = &mut buf[..];
        unsafe { mut_slice.write_le_u16_unchecked(0, 0x1234); }
        let slice: &[u8] = &buf[..];
        assert_eq!(unsafe { slice.read_le_u16_unchecked(0) }, 0x1234);
    }

    #[test]
    fn test_out_of_bounds() {
        let buf = [0u8; 1];
        let slice: &[u8] = &buf[..];
        assert_eq!(slice.read_le_u16(0), Err(Error::OutOfBounds));
        
        let mut buf = [0u8; 1];
        let mut mut_slice: &mut [u8] = &mut buf[..];
        assert_eq!(mut_slice.write_le_u16(0, 42), Err(Error::OutOfBounds));
    }
    
    #[test]
    fn test_len() {
        let buf = [0u8; 4];
        let slice: &[u8] = &buf[..];
        assert_eq!(slice.len(), 4);
    }
}
