





pub struct Simple<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> Simple<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> Simple<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn try_a(&self) -> Result<u8, emboss_runtime::Error> {
        self.storage.read_le_u8(0)
    }


    pub fn try_b(&self) -> Result<u16, emboss_runtime::Error> {
        self.storage.read_le_u16(1)
    }



    pub fn try_c_is_present(&self) -> Result<bool, emboss_runtime::Error> {
        Ok((self.a().try_read()? == 42))
    }

    pub fn try_c(&self) -> Result<Option<u8>, emboss_runtime::Error> {
        if self.try_c_is_present()? {
            Ok(Some(self.storage.read_le_u8(3)?))
        } else {
            Ok(None)
        }
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> Simple<S, ST> {
    pub fn try_set_a(mut self, value: u8) -> Result<Simple<S, ST::OnLayoutMutation>, emboss_runtime::Error> {
        self.storage.write_le_u8(0, value)?;
        Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_b(mut self, value: u16) -> Result<Simple<S, ST>, emboss_runtime::Error> {
        self.storage.write_le_u16(1, value)?;
        Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_c(mut self, value: u8) -> Result<Simple<S, ST>, emboss_runtime::Error> {
        self.storage.write_le_u8(3, value)?;
        Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> Simple<S, ST> {
// Processing struct Simple

    pub fn a(&self) -> emboss_runtime::FieldReference<&Self, u8> {
        emboss_runtime::FieldReference::new(self, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }


    pub fn b(&self) -> emboss_runtime::FieldReference<&Self, u16> {
        emboss_runtime::FieldReference::new(self, emboss_runtime::FieldLayout { offset: 1, size: 2 })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> Simple<S, ST> {

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> Simple<S, ST> {
    pub fn into_writer(self) -> SimpleWriter<S, ST> {
        SimpleWriter { storage: self.storage, _state: self._state }
    }


    pub fn c_is_present(&self) -> bool {
        self.try_c_is_present().unwrap()
    }

    pub fn c(&self) -> Option<u8> {
        if self.c_is_present() {
            Some(unsafe { self.storage.read_le_u8_unchecked(3) })
        } else {
            None
        }
    }

}

pub struct SimpleWriter<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> SimpleWriter<S, ST> {
    pub fn into_view(self) -> Simple<S, ST> {
        Simple { storage: self.storage, _state: self._state }
    }
}

impl<S, ST: emboss_runtime::State> SimpleWriter<S, ST> {
    pub fn a(self) -> emboss_runtime::FieldWriter<SimpleWriter<S, ST::OnLayoutMutation>, u8> {
        emboss_runtime::FieldWriter::new(SimpleWriter { storage: self.storage, _state: core::marker::PhantomData }, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }
}

impl<S, ST: emboss_runtime::State> SimpleWriter<S, ST> {
    pub fn b(self) -> emboss_runtime::FieldWriter<Self, u16> {
        emboss_runtime::FieldWriter::new(self, emboss_runtime::FieldLayout { offset: 1, size: 2 })
    }
}

impl<S, ST: emboss_runtime::State> SimpleWriter<S, ST> {
    pub fn c(self) -> emboss_runtime::FieldWriter<Self, u8> {
        emboss_runtime::FieldWriter::new(self, emboss_runtime::FieldLayout { offset: 3, size: 1 })
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> Simple<S, ST> {
    pub fn check_ok(self) -> Result<Simple<S, ST::OkVersion>, emboss_runtime::Error> {
        self.try_a()?;
        self.try_b()?;
        if self.try_c_is_present()? { self.try_c()?; }
        Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> Simple<S, ST::OkVersion> {
        Simple { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<Simple<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<Simple<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<Simple<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(Simple { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for Simple<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(std::cmp::max(std::cmp::max(std::cmp::max(0, (if true { (0 + 1) } else { 0 })), (if true { (1 + 2) } else { 0 })), (if (self.a().try_read()? == 42) { (3 + 1) } else { 0 })))
    }
    fn min_size_in_bytes() -> usize {
        3
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for SimpleWriter<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = Simple::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        3
    }
    fn max_size_in_bytes() -> usize {
        4
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for SimpleWriter<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> SimpleWriter<S, ST> {
    pub fn set_a(mut self, value: u8) -> Simple<S, ST::OnLayoutMutation> {
        unsafe { self.storage.write_le_u8_unchecked(0, value); }
        Simple { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_b(mut self, value: u16) -> Simple<S, ST> {
        unsafe { self.storage.write_le_u16_unchecked(1, value); }
        Simple { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_c(mut self, value: u8) -> Simple<S, ST> {
        unsafe { self.storage.write_le_u8_unchecked(3, value); }
        Simple { storage: self.storage, _state: core::marker::PhantomData }
    }

}

pub struct ComplexEdgeCases<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> ComplexEdgeCases<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> ComplexEdgeCases<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    fn try_emboss_reserved_anonymous_field_3(&self) -> Result<EmbossReservedAnonymousField3<S::Slice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice(0..0 + 1);
        Ok(EmbossReservedAnonymousField3::new(slice))
    }


    pub fn try_bit_0(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_3()?.bit_0().try_read()?) as u64)
    }


    pub fn try_bit_1_to_7(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_3()?.bit_1_to_7().try_read()?) as u64)
    }


    fn try_emboss_reserved_anonymous_field_2(&self) -> Result<EmbossReservedAnonymousField2<S::Slice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice(1..1 + 1);
        Ok(EmbossReservedAnonymousField2::new(slice))
    }


    pub fn try_nibble_0(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_2()?.nibble_0().try_read()?) as u64)
    }


    pub fn try_nibble_1(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_2()?.nibble_1().try_read()?) as u64)
    }


    fn try_emboss_reserved_anonymous_field_1(&self) -> Result<EmbossReservedAnonymousField1<S::Slice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice(2..2 + 5);
        Ok(EmbossReservedAnonymousField1::new(slice))
    }


    pub fn try_spanning_31_bits(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.spanning_31_bits().try_read()?) as u64)
    }


    pub fn try_another_field(&self) -> Result<u64, emboss_runtime::Error> {
        Ok((self.try_emboss_reserved_anonymous_field_1()?.another_field().try_read()?) as u64)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> ComplexEdgeCases<S, ST> {
    #[allow(dead_code)]
    fn try_emboss_reserved_anonymous_field_3_mut(&mut self) -> Result<EmbossReservedAnonymousField3<S::MutSlice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice_mut(0..0 + 1);
        Ok(EmbossReservedAnonymousField3::new(slice))
    }


    #[allow(dead_code)]
    fn try_emboss_reserved_anonymous_field_2_mut(&mut self) -> Result<EmbossReservedAnonymousField2<S::MutSlice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice_mut(1..1 + 1);
        Ok(EmbossReservedAnonymousField2::new(slice))
    }


    #[allow(dead_code)]
    fn try_emboss_reserved_anonymous_field_1_mut(&mut self) -> Result<EmbossReservedAnonymousField1<S::MutSlice<'_>>, emboss_runtime::Error> {
        let slice = self.storage.slice_mut(2..2 + 5);
        Ok(EmbossReservedAnonymousField1::new(slice))
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> ComplexEdgeCases<S, ST> {
// Processing struct ComplexEdgeCases
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> ComplexEdgeCases<S, ST> {
    fn emboss_reserved_anonymous_field_3(&self) -> EmbossReservedAnonymousField3<S::Slice<'_>, ST> {
        let slice = self.storage.slice(0..0 + 1);
        EmbossReservedAnonymousField3 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn bit_0(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_3().bit_0().read()) as u64
    }


    pub fn bit_1_to_7(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_3().bit_1_to_7().read()) as u64
    }


    fn emboss_reserved_anonymous_field_2(&self) -> EmbossReservedAnonymousField2<S::Slice<'_>, ST> {
        let slice = self.storage.slice(1..1 + 1);
        EmbossReservedAnonymousField2 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn nibble_0(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_2().nibble_0().read()) as u64
    }


    pub fn nibble_1(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_2().nibble_1().read()) as u64
    }


    fn emboss_reserved_anonymous_field_1(&self) -> EmbossReservedAnonymousField1<S::Slice<'_>, ST> {
        let slice = self.storage.slice(2..2 + 5);
        EmbossReservedAnonymousField1 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn spanning_31_bits(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().spanning_31_bits().read()) as u64
    }


    pub fn another_field(&self) -> u64 {
        (self.emboss_reserved_anonymous_field_1().another_field().read()) as u64
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> ComplexEdgeCases<S, ST> {
    pub fn into_writer(self) -> ComplexEdgeCasesWriter<S, ST> {
        ComplexEdgeCasesWriter { storage: self.storage, _state: self._state }
    }

}

pub struct ComplexEdgeCasesWriter<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> ComplexEdgeCasesWriter<S, ST> {
    pub fn into_view(self) -> ComplexEdgeCases<S, ST> {
        ComplexEdgeCases { storage: self.storage, _state: self._state }
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_0<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_0<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_3_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.bit_0().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_1_to_7<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_1_to_7<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_3_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.bit_1_to_7().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_0<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_0<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_2_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.nibble_0().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_1<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_1<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_2_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.nibble_1().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_spanning_31_bits<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_spanning_31_bits<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u32) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.spanning_31_bits().write(value);
        self.parent
    }
}

#[allow(non_camel_case_types)]
pub struct EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_another_field<Parent> {
    pub parent: Parent,
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_another_field<ComplexEdgeCasesWriter<S, ST>> {
    pub fn write(mut self, value: u8) -> ComplexEdgeCasesWriter<S, ST> {
        let anonymous = self.parent.emboss_reserved_anonymous_field_1_mut();
        let anonymous_writer = anonymous.into_writer();
        anonymous_writer.another_field().write(value);
        self.parent
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> ComplexEdgeCases<S, ST> {
    pub fn check_ok(self) -> Result<ComplexEdgeCases<S, ST::OkVersion>, emboss_runtime::Error> {

        Ok(ComplexEdgeCases { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> ComplexEdgeCases<S, ST::OkVersion> {
        ComplexEdgeCases { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<ComplexEdgeCases<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(ComplexEdgeCases { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<ComplexEdgeCases<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(ComplexEdgeCases { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<ComplexEdgeCases<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(ComplexEdgeCases { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for ComplexEdgeCases<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(std::cmp::max(std::cmp::max(std::cmp::max(0, (if true { (0 + 1) } else { 0 })), (if true { (1 + 1) } else { 0 })), (if true { (2 + 5) } else { 0 })))
    }
    fn min_size_in_bytes() -> usize {
        7
    }
    fn max_size_in_bytes() -> usize {
        7
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for ComplexEdgeCasesWriter<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = ComplexEdgeCases::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        7
    }
    fn max_size_in_bytes() -> usize {
        7
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for ComplexEdgeCasesWriter<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> ComplexEdgeCasesWriter<S, ST> {
    #[allow(dead_code)]
    fn emboss_reserved_anonymous_field_3_mut(&mut self) -> EmbossReservedAnonymousField3<S::MutSlice<'_>, ST> {
        let slice = self.storage.slice_mut(0..0 + 1);
        EmbossReservedAnonymousField3 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn bit_0(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_0<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_0 { parent: self }
    }


    pub fn bit_1_to_7(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_1_to_7<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_bit_1_to_7 { parent: self }
    }


    #[allow(dead_code)]
    fn emboss_reserved_anonymous_field_2_mut(&mut self) -> EmbossReservedAnonymousField2<S::MutSlice<'_>, ST> {
        let slice = self.storage.slice_mut(1..1 + 1);
        EmbossReservedAnonymousField2 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn nibble_0(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_0<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_0 { parent: self }
    }


    pub fn nibble_1(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_1<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_nibble_1 { parent: self }
    }


    #[allow(dead_code)]
    fn emboss_reserved_anonymous_field_1_mut(&mut self) -> EmbossReservedAnonymousField1<S::MutSlice<'_>, ST> {
        let slice = self.storage.slice_mut(2..2 + 5);
        EmbossReservedAnonymousField1 { storage: slice, _state: core::marker::PhantomData }
    }


    pub fn spanning_31_bits(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_spanning_31_bits<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_spanning_31_bits { parent: self }
    }


    pub fn another_field(self) -> EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_another_field<Self> {
        EmbossReserved_HoistedFieldWriter_ComplexEdgeCases_another_field { parent: self }
    }

}

impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField3Writer<S, ST> {
    pub fn bit_0(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField3Writer<S, ST> {
    pub fn bit_1_to_7(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 1, size: 7 })
    }
}





pub struct EmbossReservedAnonymousField3<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> EmbossReservedAnonymousField3<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField3<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn try_bit_0(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 0, size: 1 })? as u8)
    }


    pub fn try_bit_1_to_7(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 1, size: 7 })? as u8)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> EmbossReservedAnonymousField3<S, ST> {
    pub fn try_set_bit_0(mut self, value: u8) -> Result<EmbossReservedAnonymousField3<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 0, size: 1 }, value as u64)?;
        Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_bit_1_to_7(mut self, value: u8) -> Result<EmbossReservedAnonymousField3<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 1, size: 7 }, value as u64)?;
        Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField3<S, ST> {
// Processing struct EmbossReservedAnonymousField3

    pub fn bit_0(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }


    pub fn bit_1_to_7(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 1, size: 7 })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> EmbossReservedAnonymousField3<S, ST> {

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField3<S, ST> {
    pub fn into_writer(self) -> EmbossReservedAnonymousField3Writer<S, ST> {
        EmbossReservedAnonymousField3Writer { storage: self.storage, _state: self._state }
    }

}

pub struct EmbossReservedAnonymousField3Writer<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> EmbossReservedAnonymousField3Writer<S, ST> {
    pub fn into_view(self) -> EmbossReservedAnonymousField3<S, ST> {
        EmbossReservedAnonymousField3 { storage: self.storage, _state: self._state }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField3<S, ST> {
    pub fn check_ok(self) -> Result<EmbossReservedAnonymousField3<S, ST::OkVersion>, emboss_runtime::Error> {

        Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> EmbossReservedAnonymousField3<S, ST::OkVersion> {
        EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<EmbossReservedAnonymousField3<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<EmbossReservedAnonymousField3<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<EmbossReservedAnonymousField3<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField3<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(1)
    }
    fn min_size_in_bytes() -> usize {
        1
    }
    fn max_size_in_bytes() -> usize {
        1
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField3Writer<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = EmbossReservedAnonymousField3::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        1
    }
    fn max_size_in_bytes() -> usize {
        1
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for EmbossReservedAnonymousField3Writer<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField3Writer<S, ST> {
    pub fn set_bit_0(mut self, value: u8) -> EmbossReservedAnonymousField3<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 0, size: 1 }, value as u64); }
        EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_bit_1_to_7(mut self, value: u8) -> EmbossReservedAnonymousField3<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 1, size: 7 }, value as u64); }
        EmbossReservedAnonymousField3 { storage: self.storage, _state: core::marker::PhantomData }
    }

}

impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField2Writer<S, ST> {
    pub fn nibble_0(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 0, size: 4 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField2Writer<S, ST> {
    pub fn nibble_1(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 4, size: 4 })
    }
}





pub struct EmbossReservedAnonymousField2<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> EmbossReservedAnonymousField2<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField2<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn try_nibble_0(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 0, size: 4 })? as u8)
    }


    pub fn try_nibble_1(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 4, size: 4 })? as u8)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> EmbossReservedAnonymousField2<S, ST> {
    pub fn try_set_nibble_0(mut self, value: u8) -> Result<EmbossReservedAnonymousField2<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 0, size: 4 }, value as u64)?;
        Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_nibble_1(mut self, value: u8) -> Result<EmbossReservedAnonymousField2<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 4, size: 4 }, value as u64)?;
        Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField2<S, ST> {
// Processing struct EmbossReservedAnonymousField2

    pub fn nibble_0(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 0, size: 4 })
    }


    pub fn nibble_1(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 4, size: 4 })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> EmbossReservedAnonymousField2<S, ST> {

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField2<S, ST> {
    pub fn into_writer(self) -> EmbossReservedAnonymousField2Writer<S, ST> {
        EmbossReservedAnonymousField2Writer { storage: self.storage, _state: self._state }
    }

}

pub struct EmbossReservedAnonymousField2Writer<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> EmbossReservedAnonymousField2Writer<S, ST> {
    pub fn into_view(self) -> EmbossReservedAnonymousField2<S, ST> {
        EmbossReservedAnonymousField2 { storage: self.storage, _state: self._state }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField2<S, ST> {
    pub fn check_ok(self) -> Result<EmbossReservedAnonymousField2<S, ST::OkVersion>, emboss_runtime::Error> {

        Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> EmbossReservedAnonymousField2<S, ST::OkVersion> {
        EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<EmbossReservedAnonymousField2<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<EmbossReservedAnonymousField2<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<EmbossReservedAnonymousField2<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField2<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(1)
    }
    fn min_size_in_bytes() -> usize {
        1
    }
    fn max_size_in_bytes() -> usize {
        1
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField2Writer<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = EmbossReservedAnonymousField2::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        1
    }
    fn max_size_in_bytes() -> usize {
        1
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for EmbossReservedAnonymousField2Writer<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField2Writer<S, ST> {
    pub fn set_nibble_0(mut self, value: u8) -> EmbossReservedAnonymousField2<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 0, size: 4 }, value as u64); }
        EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_nibble_1(mut self, value: u8) -> EmbossReservedAnonymousField2<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 4, size: 4 }, value as u64); }
        EmbossReservedAnonymousField2 { storage: self.storage, _state: core::marker::PhantomData }
    }

}

impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn spanning_31_bits(self) -> emboss_runtime::BitWriter<Self, u32> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 1, size: 31 })
    }
}





impl<S, ST: emboss_runtime::State> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn another_field(self) -> emboss_runtime::BitWriter<Self, u8> {
        emboss_runtime::BitWriter::new(self, emboss_runtime::FieldLayout { offset: 32, size: 7 })
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

    pub fn try_spanning_31_bits(&self) -> Result<u32, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 1, size: 31 })? as u32)
    }


    pub fn try_another_field(&self) -> Result<u8, emboss_runtime::Error> {
        Ok(emboss_runtime::Bits { storage: &self.storage }.read_bits(emboss_runtime::FieldLayout { offset: 32, size: 7 })? as u8)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
    pub fn try_set_spanning_31_bits(mut self, value: u32) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 1, size: 31 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_another_field(mut self, value: u8) -> Result<EmbossReservedAnonymousField1<S, ST>, emboss_runtime::Error> {
        emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits(emboss_runtime::FieldLayout { offset: 32, size: 7 }, value as u64)?;
        Ok(EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> EmbossReservedAnonymousField1<S, ST> {
// Processing struct EmbossReservedAnonymousField1

    pub fn spanning_31_bits(&self) -> emboss_runtime::BitReference<&Self, u32> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 1, size: 31 })
    }


    pub fn another_field(&self) -> emboss_runtime::BitReference<&Self, u8> {
        emboss_runtime::BitReference::new(self, emboss_runtime::FieldLayout { offset: 32, size: 7 })
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
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(5)
    }
    fn min_size_in_bytes() -> usize {
        5
    }
    fn max_size_in_bytes() -> usize {
        5
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for EmbossReservedAnonymousField1Writer<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = EmbossReservedAnonymousField1::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        5
    }
    fn max_size_in_bytes() -> usize {
        5
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for EmbossReservedAnonymousField1Writer<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> EmbossReservedAnonymousField1Writer<S, ST> {
    pub fn set_spanning_31_bits(mut self, value: u32) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 1, size: 31 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_another_field(mut self, value: u8) -> EmbossReservedAnonymousField1<S, ST> {
        unsafe { emboss_runtime::BitsMut { storage: &mut self.storage }.write_bits_unchecked(emboss_runtime::FieldLayout { offset: 32, size: 7 }, value as u64); }
        EmbossReservedAnonymousField1 { storage: self.storage, _state: core::marker::PhantomData }
    }

}

pub struct NormalizedStruct<S, ST = emboss_runtime::UncheckedState> {
    storage: S,
    _state: core::marker::PhantomData<ST>,
}

impl<S: emboss_runtime::Storage> NormalizedStruct<S, emboss_runtime::UncheckedState> {
    pub fn new(storage: S) -> Self {
        Self { storage, _state: core::marker::PhantomData }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> NormalizedStruct<S, ST> {
    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn into_storage(self) -> S {
        self.storage
    }

    pub fn try_a(&self) -> Result<u8, emboss_runtime::Error> {
        self.storage.read_le_u8(0)
    }


    pub fn try_b(&self) -> Result<u16, emboss_runtime::Error> {
        self.storage.read_le_u16(1)
    }

}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> NormalizedStruct<S, ST> {
    pub fn try_set_a(mut self, value: u8) -> Result<NormalizedStruct<S, ST::OnLayoutMutation>, emboss_runtime::Error> {
        self.storage.write_le_u8(0, value)?;
        Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
    }


    pub fn try_set_b(mut self, value: u16) -> Result<NormalizedStruct<S, ST>, emboss_runtime::Error> {
        self.storage.write_le_u16(1, value)?;
        Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> NormalizedStruct<S, ST> {
// Processing struct NormalizedStruct

    pub fn a(&self) -> emboss_runtime::FieldReference<&Self, u8> {
        emboss_runtime::FieldReference::new(self, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }


    pub fn b(&self) -> emboss_runtime::FieldReference<&Self, u16> {
        emboss_runtime::FieldReference::new(self, emboss_runtime::FieldLayout { offset: 1, size: 2 })
    }

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsMinimallyComplete> NormalizedStruct<S, ST> {

}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::IsComplete> NormalizedStruct<S, ST> {
    pub fn into_writer(self) -> NormalizedStructWriter<S, ST> {
        NormalizedStructWriter { storage: self.storage, _state: self._state }
    }

}

pub struct NormalizedStructWriter<S, ST> {
    pub storage: S,
    pub _state: core::marker::PhantomData<ST>,
}

impl<S, ST> NormalizedStructWriter<S, ST> {
    pub fn into_view(self) -> NormalizedStruct<S, ST> {
        NormalizedStruct { storage: self.storage, _state: self._state }
    }
}

impl<S, ST: emboss_runtime::State> NormalizedStructWriter<S, ST> {
    pub fn a(self) -> emboss_runtime::FieldWriter<Self, u8> {
        emboss_runtime::FieldWriter::new(self, emboss_runtime::FieldLayout { offset: 0, size: 1 })
    }
}

impl<S, ST: emboss_runtime::State> NormalizedStructWriter<S, ST> {
    pub fn b(self) -> emboss_runtime::FieldWriter<Self, u16> {
        emboss_runtime::FieldWriter::new(self, emboss_runtime::FieldLayout { offset: 1, size: 2 })
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> NormalizedStruct<S, ST> {
    pub fn check_ok(self) -> Result<NormalizedStruct<S, ST::OkVersion>, emboss_runtime::Error> {
        self.try_a()?;
        self.try_b()?;
        Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
    }

    pub unsafe fn assume_ok(self) -> NormalizedStruct<S, ST::OkVersion> {
        NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData }
    }

    pub fn check_always_complete(self) -> Result<NormalizedStruct<S, emboss_runtime::AlwaysCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::max_size_in_bytes() {
            Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    pub fn check_complete(self) -> Result<NormalizedStruct<S, emboss_runtime::CompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= emboss_runtime::EmbossView::size_in_bytes(&self)? {
            Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
    
    pub fn check_minimally_complete(self) -> Result<NormalizedStruct<S, emboss_runtime::MinimallyCompleteState>, emboss_runtime::Error> {
        if self.storage.len() >= <Self as emboss_runtime::EmbossView>::min_size_in_bytes() {
            Ok(NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData })
        } else {
            Err(emboss_runtime::Error::OutOfBounds)
        }
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for NormalizedStruct<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[allow(unused_parens)]
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        Ok(std::cmp::max(std::cmp::max(0, (if true { (0 + 1) } else { 0 })), (if true { (1 + 2) } else { 0 })))
    }
    fn min_size_in_bytes() -> usize {
        3
    }
    fn max_size_in_bytes() -> usize {
        3
    }
}

impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for NormalizedStructWriter<S, ST> {
    type Storage = S;
    type State = ST;
    fn emboss_reserved_get_storage(&self) -> &S {
        &self.storage
    }
    #[inline]
    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {
        let view = NormalizedStruct::<emboss_runtime::BorrowedStorage<S>, ST> { storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData };
        view.size_in_bytes()
    }
    fn min_size_in_bytes() -> usize {
        3
    }
    fn max_size_in_bytes() -> usize {
        3
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for NormalizedStructWriter<S, ST> {
    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::IsComplete> NormalizedStructWriter<S, ST> {
    pub fn set_a(mut self, value: u8) -> NormalizedStruct<S, ST::OnLayoutMutation> {
        unsafe { self.storage.write_le_u8_unchecked(0, value); }
        NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData }
    }


    pub fn set_b(mut self, value: u16) -> NormalizedStruct<S, ST> {
        unsafe { self.storage.write_le_u16_unchecked(1, value); }
        NormalizedStruct { storage: self.storage, _state: core::marker::PhantomData }
    }

}
