#![allow(non_camel_case_types)]

use crate::{StringMethodReference, StringName, StringTypeReference};
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use proc_macros::WithType;

#[derive(Debug, Clone, WithType)]
#[with_type(repr = u64)]
#[with_type(derive = (Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive))]
pub enum StringInstruction {
    LoadTrue {
        register_addr: u64,
    },
    LoadFalse {
        register_addr: u64,
    },

    //<editor-fold desc="Load u8">
    Load_u8 {
        register_addr: u64,
        val: u8,
    },
    Load_u8_0 {
        register_addr: u64,
    },
    Load_u8_1 {
        register_addr: u64,
    },
    Load_u8_2 {
        register_addr: u64,
    },
    Load_u8_3 {
        register_addr: u64,
    },
    Load_u8_4 {
        register_addr: u64,
    },
    Load_u8_5 {
        register_addr: u64,
    },
    //</editor-fold>

    //<editor-fold desc="Load u64">
    Load_u64 {
        register_addr: u64,
        val: u64,
    },
    //</editor-fold>
    NewObject {
        ty: StringTypeReference,
        ctor_name: StringName,
        args: Vec<u64>,
        register_addr: u64,
    },

    InstanceCall {
        val: u64,
        method: StringMethodReference,
        args: Vec<u64>,
        ret_at: u64,
    },

    StaticCall {
        ty: StringTypeReference,
        method: StringMethodReference,
        args: Vec<u64>,
        ret_at: u64,
    },

    LoadArg {
        register_addr: u64,
        arg: u64,
    },

    #[deprecated = "It does not perform as what you expected"]
    LoadAllArgsAsArray {
        register_addr: u64,
    },

    LoadStatic {
        register_addr: u64,
        ty: StringTypeReference,
        name: StringName,
    },

    SetField {
        register_addr: u64,
        field: StringName,
    },

    ReturnVal {
        register_addr: u64,
    },
}
