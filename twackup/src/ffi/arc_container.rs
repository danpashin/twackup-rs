/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

use safer_ffi::layout::{OpaqueKind, ReprC};
use std::{mem, ops::Deref, ptr::NonNull, sync::Arc};

pub trait ArcContainerFFIType {
    fn name() -> String;
}

#[repr(transparent)]
pub struct ArcContainer<Type>(pub NonNull<Type>);

#[repr(transparent)]
pub struct ContainerTypeImpl<Type>(NonNull<Type>);

impl<Type> ArcContainer<Type> {
    pub fn leak(object: Type) -> Self {
        let pointer = Arc::into_raw(Arc::new(object)).cast_mut();
        let pointer = unsafe { NonNull::new_unchecked(pointer) };
        Self(pointer)
    }

    pub fn from_ref(object: &Type) -> Self {
        let pointer = (object as *const Type).cast_mut();
        let pointer = unsafe { NonNull::new_unchecked(pointer) };

        let container = Self(pointer);
        container.retain();

        container
    }

    pub fn ref_count(&self) -> usize {
        let reference = unsafe { Arc::from_raw(self.0.as_ptr()) };
        let count = Arc::strong_count(&reference);
        mem::forget(reference);

        count
    }

    pub fn retain(&self) {
        unsafe { Arc::increment_strong_count(self.0.as_ptr()) };
    }

    pub fn release(&self) {
        unsafe { Arc::decrement_strong_count(self.0.as_ptr()) };
    }
}

impl<Type> Clone for ArcContainer<Type> {
    fn clone(&self) -> Self {
        self.retain();
        Self(self.0)
    }
}

impl<Type> Deref for ArcContainer<Type> {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<Type> Drop for ArcContainer<Type> {
    fn drop(&mut self) {
        self.release();
    }
}

impl<Type> AsRef<Type> for ArcContainer<Type> {
    fn as_ref(&self) -> &Type {
        self
    }
}

unsafe impl<Type> Send for ArcContainer<Type> {}

unsafe impl<Type> ReprC for ArcContainer<Type>
where
    Type: ArcContainerFFIType,
{
    type CLayout = ContainerTypeImpl<Type>;

    fn is_valid(_it: &'_ Self::CLayout) -> bool {
        true
    }
}

impl<Type> Clone for ContainerTypeImpl<Type> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Type> Copy for ContainerTypeImpl<Type> {}

unsafe impl<Type> safer_ffi::layout::CType for ContainerTypeImpl<Type>
where
    Type: ArcContainerFFIType,
{
    type OPAQUE_KIND = OpaqueKind::Concrete;

    #[cfg(feature = "ffi-headers")]
    fn short_name() -> String {
        Type::name()
    }

    #[cfg(feature = "ffi-headers")]
    fn define_self__impl(
        language: &'_ dyn safer_ffi::headers::languages::HeaderLanguage,
        definer: &'_ mut dyn safer_ffi::headers::Definer,
    ) -> std::io::Result<()> {
        use safer_ffi::headers::languages;
        if language.is::<languages::C>() {
            let me = Self::name(language);
            writeln!(definer.out(), "typedef void *{me};")
        } else {
            Ok(())
        }
    }
}
