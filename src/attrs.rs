use derive_ctor::ctor;
use enumflags2::{BitFlags, bitflags};
use getset::{CopyGetters, MutGetters, Setters};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use proc_macros::{UnwrapEnum, WithType};
use std::fmt::Debug;

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldImplementationFlags {
    Static,
}

#[derive(Clone, Copy, Debug, ctor, CopyGetters, Setters, MutGetters)]
#[ctor(pub new)]
#[getset(set = "pub", get_mut = "pub")]
#[get_copy = "pub"]
pub struct FieldAttr {
    vis: Visibility,
    impl_flags: BitFlags<FieldImplementationFlags>,
}

impl FieldAttr {
    pub fn is_static(&self) -> bool {
        self.impl_flags.contains(FieldImplementationFlags::Static)
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MethodImplementationFlags {
    Static,
    ImplementedByRuntime,
}

#[derive(Clone, Copy, CopyGetters, Debug, ctor, Setters, MutGetters)]
#[ctor(pub new)]
#[getset(set = "pub", get_mut = "pub")]
#[get_copy = "pub"]
pub struct MethodAttr {
    vis: Visibility,
    impl_flags: BitFlags<MethodImplementationFlags>,
    register_len: u64,
}

#[derive(Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Visibility {
    Public,
    Private,
    AssemblyOnly,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, UnwrapEnum, WithType)]
#[with_type(repr = u8)]
#[with_type(derive = (TryFromPrimitive, IntoPrimitive, Clone, Copy))]
#[unwrap_enum(ref, ref_mut)]
pub enum TypeSpecificAttr {
    Class(BitFlags<ClassImplementationFlags>),
    Struct(BitFlags<StructImplementationFlags>),
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StructImplementationFlags {
    Ref,
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClassImplementationFlags {
    Static,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, ctor, CopyGetters, Setters, MutGetters)]
#[ctor(pub new)]
#[getset(set = "pub", get_mut = "pub")]
#[get_copy = "pub"]
pub struct TypeAttr {
    vis: Visibility,
    specific: TypeSpecificAttr,
}
