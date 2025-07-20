/// # Examples
/// ```rust,ignore
/// match_iota! {
///     match 2 => {
///         iota => 1,
///         iota => 2,
///         iota => 3,
///     } with iota type u64;
///     _ => -1
/// }
/// ```
pub macro match_iota(
        match $e:expr => {
            $(
                $i:ident => $x:expr
            ),* $(,)?
        } $t:ty
        $(;_ => $alt:expr)?
    ) {
    $crate::paste::paste! {
        {
            $crate::iota::iota!{
                $(
                    ${ignore($x)}
                    const [<$i:snake:upper _ ${index()}>]: $t = iota;
                )*
            }
            match $e {
                $(
                    [<$i:snake:upper _ ${index()}>] => $x,
                )*
                $(_ => $alt)?
            }
        }
    }
}

/// # Examples
/// ```rust,ignore
/// hash_map! {
///     "a" => "a"
/// }
/// ```
pub macro hash_map($(
        $k:expr => $v:expr
    ),* $(,)?) {{
    let mut map = ::std::collections::HashMap::with_capacity(${count($k)});
    $(
        map.insert($k, $v);
    )*
    map
}}

/// # Examples
/// ```rust,ignore
/// lit_string_hash_map! {
///     "a" => "a"
/// }
/// ```
pub macro lit_string_hash_map($(
        $k:literal => $v:expr
    ),* $(,)?) {{
    let mut map = ::std::collections::HashMap::with_capacity(${count($k)});
    $(
        map.insert(($k).to_owned(), $v);
    )*
    map
}}

/// # Examples
/// ```rust,ignore
/// lit_string_index_map! {
///     "a" => "a"
/// }
/// ```
pub macro lit_string_index_map($(
    $k:literal => $v:expr
),* $(,)?) {{
    let mut map = $crate::IndexMap::with_capacity(${count($k)});
    $(
        map.insert($crate::string_name!($k), $v);
    )*
    map
}}

pub macro string_name($s:literal) {
    $crate::StringName::from_static_str($s)
}

#[cfg(test)]
#[allow(unused)]
mod tests4proc_macro {
    use proc_macros::UnwrapEnum;

    use super::*;

    #[derive(UnwrapEnum)]
    #[unwrap_enum(ref, ref_mut)]
    enum TestUnwrap {
        A,
        B,
        C,
        #[unwrap_enum(owned)]
        D(u64),
        E(u8),
        Multi(u8, u64),
    }
}
