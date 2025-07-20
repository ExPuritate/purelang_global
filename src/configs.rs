#![allow(clippy::derivable_impls)]

pub mod runtime {
    use crate::path_searcher;
    use bon::Builder;
    use getset::{CopyGetters, Getters, MutGetters};
    use std::sync::Arc;

    #[derive(Clone, Getters, MutGetters, derive_more::Debug, Builder, CopyGetters)]
    #[getset(get = "pub")]
    pub struct VMConfig {
        #[builder(default)]
        default_cpu_config: CPUConfig,
        #[cfg_attr(debug_assertions, builder(default = true))]
        #[cfg_attr(not(debug_assertions), builder(default = false))]
        #[getset(skip)]
        #[get_copy = "pub"]
        is_dynamic_checking_enabled: bool,
        #[debug(skip)]
        #[getset(get_mut = "pub")]
        assembly_lookuper: Option<Arc<dyn Fn(&str) -> Option<String>>>,
    }

    impl Default for VMConfig {
        fn default() -> Self {
            Self {
                default_cpu_config: CPUConfig::default(),
                #[cfg(debug_assertions)]
                is_dynamic_checking_enabled: true,
                #[cfg(not(debug_assertions))]
                is_dynamic_checking_enabled: false,
                assembly_lookuper: Some(Arc::new(|name| {
                    let stdlib = path_searcher::get_stdlib_dir().ok()?;
                    let std_dir = std::fs::read_dir(&stdlib).ok()?;
                    for entry in std_dir {
                        let entry = entry.ok()?;
                        if entry.file_type().unwrap().is_file() && *entry.file_name() == *name {
                            return Some(entry.path().to_str()?.to_owned());
                        }
                    }
                    None
                })),
            }
        }
    }

    #[derive(Clone, Debug, Getters, CopyGetters)]
    pub struct CPUConfig {
        #[get_copy = "pub"]
        default_register_num: u64,
    }

    impl Default for CPUConfig {
        fn default() -> Self {
            Self {
                default_register_num: u8::MAX as _,
            }
        }
    }
}

pub mod compiler {
    use bon::Builder;
    use getset::Getters;

    #[derive(Getters, Builder, Clone, Debug)]
    #[getset(get = "pub")]
    pub struct CompilerConfig {
        stdlib_dir: String,
    }

    impl Default for CompilerConfig {
        fn default() -> Self {
            Self {
                stdlib_dir: crate::path_searcher::get_stdlib_dir().unwrap(),
            }
        }
    }

    #[derive(Getters, Builder, Clone, Debug)]
    #[getset(get = "pub")]
    pub struct CompileServiceConfig {
        default_compiler_config: CompilerConfig,
    }

    impl Default for CompileServiceConfig {
        fn default() -> Self {
            Self {
                default_compiler_config: Default::default(),
            }
        }
    }
}
