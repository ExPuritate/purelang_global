use derive_more::Display;
use std::panic::Location;

use crate::{StringMethodReference, StringName, StringTypeReference};

pub enum TypeSystemError {}

#[derive(Clone, Debug, Display, thiserror::Error)]
pub enum RuntimeError {
    FailedGetRegister,
    FailedReadRegister,
    FailedWriteRegister,
    FailedGetMethod(StringMethodReference),
    #[display("FailedGetType({_0})")]
    FailedGetType(StringTypeReference),
    FailedGetAssembly,
    #[display("FailedGetField({_0})")]
    FailedGetField(StringName),
    FailedMakeGeneric,

    ArrayIndexOutOfRange,

    UnsupportedEntryType,
    UnsupportedInstanceType,
    UnsupportedObjectType,
    UnsupportedGettingField,
    UnsupportedParentType,

    MethodReturnsAbnormally,

    NonGenericType(StringName),

    WrongType,

    BrokenReference,
    UnloadedType(StringTypeReference),
    ConstructStaticClass,
    DynamicCheckingFailed(DynamicCheckingItem),
    InvalidOperation(RuntimeMayBeInvalidOperation),
    NoConsole,
    #[cfg(windows)]
    WindowsAPIError,
    #[cfg(unix)]
    LibcError(std::ffi::c_int),
    ConsoleBufferLessThanWindowSize {
        /// System_Console_ buffer width is less than window size if `true`, or height is less than window size
        is_width: bool,
    },
}

#[derive(Clone, Debug, Display)]
pub enum DynamicCheckingItem {
    #[display("DynamicCheckingItem::ArgLen {{got: {got}, expected: {expected}}}")]
    ArgLen { got: usize, expected: usize },
    #[display("DynamicCheckingItem::Type {{got: {got}, expected: {expected}}}")]
    Type {
        got: StringTypeReference,
        expected: StringTypeReference,
    },
}

#[derive(Clone, Debug, Display)]
pub enum RuntimeMayBeInvalidOperation {
    ConsoleKeyAvailableOnFile,
}

impl RuntimeError {
    #[track_caller]
    pub fn throw(self) -> GenericError<RuntimeError> {
        GenericError::throw(self)
    }
}

#[derive(Clone, Copy, Debug, Display, thiserror::Error)]
pub struct UnwrapError;

#[derive(thiserror::Error, Display, Debug)]
pub enum BinaryError {
    StringNotFound {
        index: u64,
    },
    IndexOutOfRange,
    #[display("Unexpected `TypeSpecificAttr`: {_0}")]
    UnexpectedTypeSpecificAttr(&'static str),
    WrongFileFormat,
    SectionNotFound,
    BinaryTooShort,
    EnumOutOfBounds(&'static str),
}

impl BinaryError {
    #[track_caller]
    #[inline(always)]
    pub fn throw(self) -> GenericError<Self> {
        GenericError::throw(self)
    }
}

#[derive(Clone, Debug, Display)]
#[display("Error: {e:#?}\n(caller: {caller})")]
pub struct GenericError<E: std::error::Error + 'static> {
    e: E,
    caller: &'static Location<'static>,
}

impl<E: std::error::Error + 'static> GenericError<E> {
    #[track_caller]
    #[inline(always)]
    pub fn throw(e: E) -> Self {
        Self {
            e,
            caller: Location::caller(),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for GenericError<E> {}

#[derive(Clone, Debug, Display, thiserror::Error)]
pub enum EncodingError {
    UnsupportedEncoding(&'static str),
}

pub use anyhow::anyhow;

#[derive(Clone, Debug, Display, thiserror::Error)]
pub enum ParseStrError {
    AtStringTypeReference(StringName),
    AtStringMethodReference(StringName),
}

impl ParseStrError {
    #[track_caller]
    pub fn throw(self) -> GenericError<Self> {
        GenericError::throw(self)
    }
}

#[derive(Clone, Debug, Display, thiserror::Error)]
pub enum CompileServiceError {
    NoCompilerMatched(StringName),
}
