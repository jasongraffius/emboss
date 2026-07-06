





pub struct IsoDataFrameHeader<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> IsoDataFrameHeader<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> IsoDataFrameHeader<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    fn try_emboss_reserved_anonymous_field_1(&self) -> Result<EmbossReservedAnonymousField1<S::Slice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice(0..0 + 4);
        Ok(EmbossReservedAnonymousField1::new(slice))
    }


    pub fn try_connection_handle(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.connection_handle().try_read()?) as u64)
    }


    pub fn try_pb_flag(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.pb_flag().try_read()?) as u64)
    }


    pub fn try_ts_flag(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.ts_flag().try_read()?) as u64)
    }


    pub fn try_data_total_length(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.data_total_length().try_read()?) as u64)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> IsoDataFrameHeader<S, ST> {
    #[allow(dead_code)]
    fn try_emboss_reserved_anonymous_field_1_mut(&mut self) -> Result<EmbossReservedAnonymousField1<S::MutSlice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice_mut(0..0 + 4);
        Ok(EmbossReservedAnonymousField1::new(slice))
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> IsoDataFrameHeader<S, ST> {
// Processing struct IsoDataFrameHeader
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> IsoDataFrameHeader<S, ST> {
    fn emboss_reserved_anonymous_field_1(&self) -> EmbossReservedAnonymousField1<S::Slice<'_>, ST> {
        let slice = self.storage.slice(0..0 + 4);
        EmbossReservedAnonymousField1 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn connection_handle(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().connection_handle().try_read().unwrap()) as u64
    }


    pub fn pb_flag(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().pb_flag().try_read().unwrap()) as u64
    }


    pub fn ts_flag(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().ts_flag().try_read().unwrap()) as u64
    }


    pub fn data_total_length(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().data_total_length().try_read().unwrap()) as u64
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> IsoDataFrameHeader<S, ST> {
    pub fn into_writer(self) -> IsoDataFrameHeaderWriter<S, ST> {
        IsoDataFrameHeaderWriter { storage: self.storage, _state: self._state }
    }

}

pub struct IsoDataFrameHeaderWriter<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> IsoDataFrameHeaderWriter<S, ST> {
    pub fn into_view(self) -> IsoDataFrameHeader<S, ST> {
        IsoDataFrameHeader { storage: self.storage, _state: self._state }
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_connection_handle<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_connection_handle<IsoDataFrameHeaderWriter<S, ST>> {
    pub fn write(mut self, value: u16) -> IsoDataFrameHeaderWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.connection_handle().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_pb_flag<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_pb_flag<IsoDataFrameHeaderWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> IsoDataFrameHeaderWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.pb_flag().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_ts_flag<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_ts_flag<IsoDataFrameHeaderWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> IsoDataFrameHeaderWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.ts_flag().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_data_total_length<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_data_total_length<IsoDataFrameHeaderWriter<S, ST>> {
    pub fn write(mut self, value: u16) -> IsoDataFrameHeaderWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.data_total_length().write(value);
        self.parent
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> IsoDataFrameHeader<S, ST> {
    pub fn check_ok(self) -> Result<IsoDataFrameHeader<S, ST::OkVersion>, emboss_runtime::Error> {

        Ok(IsoDataFrameHeader { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> IsoDataFrameHeader<S, ST::OkVersion> {
        IsoDataFrameHeader { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<IsoDataFrameHeader<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(IsoDataFrameHeader { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<IsoDataFrameHeader<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(IsoDataFrameHeader { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<IsoDataFrameHeader<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(IsoDataFrameHeader { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for IsoDataFrameHeader<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(std::cmp::max(0, (if true { (0 + 4) } else { 0 })))
    }
    fn min_size_in_bytes() -> usize {
        4
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for IsoDataFrameHeaderWriter<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(std::cmp::max(0, (if true { (0 + 4) } else { 0 })))
    }
    fn min_size_in_bytes() -> usize {
        4
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for IsoDataFrameHeaderWriter<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> IsoDataFrameHeaderWriter<S, ST> {
    #[allow(dead_code)]
    fn emboss_reserved_anonymous_field_1_mut(&mut self) -> EmbossReservedAnonymousField1<S::MutSlice<'_>, ST> {
        let slice = self.storage.slice_mut(0..0 + 4);
        EmbossReservedAnonymousField1 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn connection_handle(self) -> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_connection_handle<Self> {
        EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_connection_handle { parent: self }
    }


    pub fn pb_flag(self) -> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_pb_flag<Self> {
        EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_pb_flag { parent: self }
    }


    pub fn ts_flag(self) -> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_ts_flag<Self> {
        EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_ts_flag { parent: self }
    }


    pub fn data_total_length(self) -> EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_data_total_length<Self> {
        EmbossReserved_HoistedFieldWriter_IsoDataFrameHeader_data_total_length { parent: self }
    }

}

impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn connection_handle(self) -> emboss_runtime::BitWriter<Self, u16> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 0, size: 12 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn pb_flag(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 12, size: 2 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn ts_flag(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 14, size: 1 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn data_total_length(self) -> emboss_runtime::BitWriter<Self, u16> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 16, size: 14 })
    }
}





pub struct EmbossReservedAnonymousField1<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> EmbossReservedAnonymousField1<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn try_connection_handle(&self) -> Result<u16, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 0, size: 12 })? as u16)
    }


    pub fn try_pb_flag(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 12, size: 2 })? as u8)
    }


    pub fn try_ts_flag(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 14, size: 1 })? as u8)
    }


    pub fn try_data_total_length(&self) -> Result<u16, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 16, size: 14 })? as u16)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
    pub fn try_set_connection_handle(mut self, value: u16) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 0, size: 12 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_pb_flag(mut self, value: u8) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 12, size: 2 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_ts_flag(mut self, value: u8) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 14, size: 1 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_data_total_length(mut self, value: u16) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 16, size: 14 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
// Processing struct EmbossReservedAnonymousField1

    pub fn connection_handle(&self) -> emboss_runtime::BitReference<&Self, u16> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 0, size: 12 })
    }


    pub fn pb_flag(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 12, size: 2 })
    }


    pub fn ts_flag(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 14, size: 1 })
    }


    pub fn data_total_length(&self) -> emboss_runtime::BitReference<&Self, u16> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 16, size: 14 })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> EmbossReservedAnonymousField1<S, ST> {

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField1<S, ST> {
    pub fn into_writer(self) -> EmbossReservedAnonymousField1Writer<S, ST> {
        EmbossReservedAnonymousField1Writer { storage: self.storage, _state: self._state }
    }

}

pub struct EmbossReservedAnonymousField1Writer<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn into_view(self) -> EmbossReservedAnonymousField1<S, ST> {
        EmbossReservedAnonymousField1 { storage: self.storage, _state: self._state }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
    pub fn check_ok(self) -> Result<EmbossReservedAnonymousField1<S, ST::OkVersion>, emboss_runtime::Error> {

        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> EmbossReservedAnonymousField1<S, ST::OkVersion> {
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<EmbossReservedAnonymousField1<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<EmbossReservedAnonymousField1<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<EmbossReservedAnonymousField1<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField1<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(4)
    }
    fn min_size_in_bytes() -> usize {
        4
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField1Writer<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(4)
    }
    fn min_size_in_bytes() -> usize {
        4
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for EmbossReservedAnonymousField1Writer<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn set_connection_handle(mut self, value: u16) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 0, size: 12 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_pb_flag(mut self, value: u8) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 12, size: 2 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_ts_flag(mut self, value: u8) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 14, size: 1 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_data_total_length(mut self, value: u16) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 16, size: 14 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }

}
