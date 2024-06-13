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

pub(crate) mod field;
mod priority;
mod section;
mod status;

use self::{priority::TwPackagePriority, section::TwPackageSection, status::TwPackageState};
use super::arc_container::{ArcContainer, ArcContainerFFIType};
use crate::package::Package;
use safer_ffi::{
    derive_ReprC,
    prelude::c_slice::{Raw, Ref},
};

#[derive_ReprC]
#[repr(C)]
pub struct TwPackage {
    pub(crate) inner: ArcContainer<Package>,
    identifier: Raw<u8>,
    name: Raw<u8>,
    version: Raw<u8>,
    section: TwPackageSection,
    state: TwPackageState,
    priority: TwPackagePriority,
}

impl From<ArcContainer<Package>> for TwPackage {
    fn from(container: ArcContainer<Package>) -> Self {
        let clone = container.clone();

        Self {
            inner: container,
            identifier: Raw::from(Ref::from(clone.id.as_bytes())),
            name: Raw::from(Ref::from(clone.human_name().as_bytes())),
            version: Raw::from(Ref::from(clone.version.as_bytes())),
            section: (&clone.section).into(),
            state: clone.status.into(),
            priority: clone.priority.into(),
        }
    }
}

impl From<Package> for TwPackage {
    fn from(package: Package) -> Self {
        Self::from(ArcContainer::leak(package))
    }
}

impl From<&Package> for TwPackage {
    fn from(package_ref: &Package) -> Self {
        Self::from(ArcContainer::from_ref(package_ref))
    }
}

impl ArcContainerFFIType for Package {
    fn name() -> String {
        "TwPackageRef".to_string()
    }
}
