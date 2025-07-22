use crate::errors::{GenericError, ParseStrError};
use crate::{StringName, string_name};
use derive_more::Unwrap;
use indexmap::IndexMap;
use proc_macros::ThreadSafe;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;
use std::{hash::Hash, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, ThreadSafe)]
pub enum StringTypeReference {
    Single {
        assem: StringName,
        ty: StringName,
    },
    Generic(StringName),
    WithGeneric {
        assem: StringName,
        ty: StringName,
        type_vars: Arc<IndexMap<StringName, StringTypeReference>>,
    },
}

impl Hash for StringTypeReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            StringTypeReference::Single { assem, ty } => {
                0u8.hash(state);
                assem.hash(state);
                ty.hash(state);
            }
            StringTypeReference::Generic(string_name) => {
                1u8.hash(state);
                string_name.hash(state);
            }
            StringTypeReference::WithGeneric {
                assem,
                ty,
                type_vars,
            } => {
                2u8.hash(state);
                assem.hash(state);
                ty.hash(state);
                for (k, v) in type_vars.iter() {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}

impl StringTypeReference {
    pub const CORE_ASSEMBLY_NAME: StringName = string_name!("!");

    pub const fn core_static_single_type(ty: &'static str) -> Self {
        Self::core_single_type(StringName::from_static_str(ty))
    }

    pub const fn core_single_type(ty: StringName) -> Self {
        Self::Single {
            assem: Self::CORE_ASSEMBLY_NAME,
            ty,
        }
    }
    pub const fn core_generic_type(
        ty: StringName,
        type_vars: Arc<IndexMap<StringName, StringTypeReference>>,
    ) -> Self {
        Self::WithGeneric {
            assem: Self::CORE_ASSEMBLY_NAME,
            ty,
            type_vars,
        }
    }
    pub const fn unwrap_single_name_ref(&self) -> &StringName {
        match self {
            Self::Single { assem: _, ty } => ty,
            _ => core::panicking::panic("unwrap single failed"),
        }
    }

    pub const fn make_static_single(assem: &'static str, ty: &'static str) -> Self {
        Self::Single {
            assem: StringName::from_static_str(assem),
            ty: StringName::from_static_str(ty),
        }
    }
}

impl StringTypeReference {
    pub fn is_generic(&self) -> bool {
        matches!(self, Self::Generic(_))
    }
}

impl StringTypeReference {
    pub fn string_name_repr_without_assembly(&self) -> StringName {
        match self {
            StringTypeReference::Single { assem: _, ty } => {
                StringName::from(ty.as_str().to_string())
            }
            StringTypeReference::Generic(s) => StringName::from(s.as_str()),
            StringTypeReference::WithGeneric {
                assem: _,
                ty,
                type_vars,
            } => StringName::from(format!(
                "{}[{}]",
                ty.as_str(),
                type_vars
                    .iter()
                    .map(|(n, a)| format!("{n}:{}", a.string_name_repr().as_str().to_owned()))
                    .collect::<Vec<_>>()
                    .join("|")
            )),
        }
    }
    pub fn assembly_name(&self) -> Option<&StringName> {
        match self {
            StringTypeReference::Single { assem, .. } => Some(assem),
            StringTypeReference::Generic(_) => None,
            StringTypeReference::WithGeneric { assem, .. } => Some(assem),
        }
    }
    pub fn string_name_repr(&self) -> StringName {
        match self {
            StringTypeReference::Single { assem, ty } => {
                StringName::from(format!("[{}]{}", assem.as_str(), ty.as_str()))
            }
            StringTypeReference::Generic(s) => StringName::from(s.as_str()),
            StringTypeReference::WithGeneric {
                assem,
                ty,
                type_vars,
            } => StringName::from(format!(
                "[{}]{}[{}]",
                assem.as_str(),
                ty.as_str(),
                type_vars
                    .iter()
                    .map(|(n, a)| format!("{n}:{}", a.string_name_repr().as_str().to_owned()))
                    .collect::<Vec<_>>()
                    .join("|")
            )),
        }
    }
    #[track_caller]
    pub fn from_string_repr<T: AsRef<str>>(
        s: T,
    ) -> crate::Result<Self, GenericError<ParseStrError>> {
        let s = s.as_ref();
        if s.starts_with('@') {
            Ok(Self::Generic(s.into()))
        } else {
            if !s.starts_with('[') {
                return Err(ParseStrError::AtStringTypeReference(s.into()).throw());
            }
            let (assem, ty) = s
                .split_once(']')
                .ok_or(ParseStrError::AtStringTypeReference(s.into()).throw())?;
            if ty.contains('[') {
                if !ty.ends_with(']') {
                    return Err(ParseStrError::AtStringTypeReference(s.into()).throw());
                }
                let (name, type_vars) = ty
                    .split_once('[')
                    .ok_or(ParseStrError::AtStringTypeReference(s.into()).throw())?;
                let type_vars = type_vars[..(type_vars.len() - 1)]
                    .split('|')
                    .map(|x| x.split_once(":"))
                    .map(|x| {
                        let (k, v) = x?;
                        Some((
                            StringName::from(k),
                            StringTypeReference::from_string_repr(v).ok()?,
                        ))
                    })
                    .try_collect::<IndexMap<_, _>>()
                    .ok_or(ParseStrError::AtStringTypeReference(s.into()).throw())?;
                Ok(StringTypeReference::WithGeneric {
                    assem: (&assem[1..]).into(),
                    ty: name.into(),
                    type_vars: Arc::new(type_vars),
                })
            } else {
                Ok(Self::Single {
                    assem: (&assem[1..]).into(),
                    ty: ty.into(),
                })
            }
        }
    }
}
impl Display for StringTypeReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string_name_repr().as_str())
    }
}

#[derive(Unwrap, Clone, Debug)]
pub enum StringMethodReference {
    /// e.g. A(), A(\[!\]A), A(\[!\]A,\[!\]B)
    /// No spaces around commas
    Single(StringName),
    WithGeneric(StringName, Arc<IndexMap<StringName, StringTypeReference>>),
}

impl StringMethodReference {
    pub const STATIC_CTOR_REF: Self = Self::Single(string_name!(".sctor()"));
    pub const fn static_single(name: &'static str) -> Self {
        Self::Single(StringName::from_static_str(name))
    }

    pub fn string_name_repr(&self) -> StringName {
        match self {
            Self::Single(s) => s.clone(),
            Self::WithGeneric(name, type_vars) => StringName::from_string(format!(
                "{}[{}]",
                name.as_str(),
                type_vars
                    .iter()
                    .map(|x| format!("{}:{}", x.0.as_str(), x.1.string_name_repr().as_str()))
                    .collect::<Vec<_>>()
                    .join("|")
            )),
        }
    }
    pub fn from_string_repr<T: AsRef<str>>(s: T) -> crate::Result<Self, crate::Error> {
        let s = s.as_ref();
        static REGEX: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
            fancy_regex::Regex::new(r"^(?P<Name>.*?\(.*\))(?P<TypeVars>\[.*\])?$").unwrap()
        });
        let Some(captures) = REGEX.captures(s)? else {
            return Err(ParseStrError::AtStringMethodReference(s.into())
                .throw()
                .into());
        };
        let type_vars = captures.name("TypeVars");
        dbg!(&type_vars);
        let name = captures
            .name("Name")
            .ok_or(ParseStrError::AtStringMethodReference(s.into()).throw())?;
        if let Some(type_vars) = type_vars {
            Ok(Self::WithGeneric(
                StringName::from(name.as_str()),
                Arc::new(
                    type_vars
                        .as_str()
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .split('|')
                        .map(|x| {
                            if let Some((k, v)) = x.split_once(':') {
                                Ok((
                                    StringName::from(k),
                                    StringTypeReference::from_string_repr(v)?,
                                ))
                            } else {
                                Err(ParseStrError::AtStringMethodReference(s.into()).throw())
                            }
                        })
                        .try_collect()?,
                ),
            ))
        } else {
            Ok(Self::Single(StringName::from(name.as_str())))
        }
    }
}

impl Display for StringMethodReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string_name_repr().as_str())
    }
}
