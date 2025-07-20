#![feature(proc_macro_totokens)]
#![feature(extend_one)]
#![feature(lazy_get)]
#![allow(static_mut_refs)]

mod util;
mod with_type;

use crate::util::get_crate_name_of;
use with_type::*;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, Ident, ItemImpl, LitStr, Token, Visibility,
    parse::Parser, parse_macro_input, spanned::Spanned, token::Paren,
};

#[proc_macro_derive(WithType, attributes(with_type))]
pub fn derive_with_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_with_type_impl(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ThreadSafe)]
pub fn derive_thread_safe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (i_generics, t_generics, where_clause) = &input.generics.split_for_impl();
    quote! {
        unsafe impl #i_generics Send for #name #t_generics #where_clause {}
        unsafe impl #i_generics Sync for #name #t_generics #where_clause {}
    }
    .into()
}

#[proc_macro_attribute]
pub fn inline_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let inline_attr = quote! { #[inline] };
    let inline_attr = match Parser::parse2(Attribute::parse_outer, inline_attr) {
        Ok(x) => x[0].clone(),
        Err(e) => return e.into_compile_error().into(),
    };
    for i in ast.items.iter_mut() {
        if let syn::ImplItem::Fn(f) = i {
            f.attrs.push(inline_attr.clone())
        }
    }
    ast.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn public_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let pub_tok = Token![pub](Span::call_site());
    for i in ast.items.iter_mut() {
        if let syn::ImplItem::Fn(f) = i {
            f.vis = Visibility::Public(pub_tok);
        }
    }
    ast.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn instrument_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let tracing_name = &get_crate_name_of("tracing", Span::call_site());
    let attr: proc_macro2::TokenStream = attr.into();
    let attr = quote! { #[#tracing_name::instrument(#attr)] };
    let attr = match Parser::parse2(Attribute::parse_outer, attr) {
        Ok(x) => x[0].clone(),
        Err(e) => return e.into_compile_error().into(),
    };
    for i in ast.items.iter_mut() {
        if let syn::ImplItem::Fn(f) = i {
            if f.attrs.iter().any(|x| {
                let p = x.path();
                p.segments
                    .iter()
                    .any(|x| x.ident == Ident::new("instrument", x.span()))
            }) {
            } else {
                f.attrs.push(attr.clone())
            }
        }
    }
    ast.to_token_stream().into()
}

#[proc_macro_derive(PartialEq, attributes(custom_eq, fully_eq))]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, generics, where_clauses) = ast.generics.split_for_impl();
    let is_fully_eq = ast.attrs.iter().any(|x| {
        x.path()
            .get_ident()
            .map(|x| x.eq("fully_eq"))
            .unwrap_or(false)
    });
    match &ast.data {
        Data::Struct(_data_struct) => todo!(),
        Data::Enum(data_enum) => {
            let mut tokens = Vec::new();
            for variant in &data_enum.variants {
                let name = &variant.ident;
                let fields = match &variant.fields {
                    Fields::Named(named) => {
                        let identifiers: Vec<_> = named
                            .named
                            .iter()
                            .map(|x| x.ident.clone().unwrap())
                            .collect();
                        let _identifiers: Vec<_> = named
                            .named
                            .iter()
                            .map(|x| format_ident!("{}_", x.ident.clone().unwrap()))
                            .collect();
                        let custom_eqs = &variant
                            .attrs
                            .iter()
                            .find_map(|x| {
                                if !x
                                    .path()
                                    .get_ident()
                                    .map(|a| a.eq("custom_eq"))
                                    .unwrap_or(false)
                                {
                                    return None;
                                }
                                x.parse_args::<Expr>().ok().map(|x| x.to_token_stream())
                            })
                            .unwrap_or(quote!(#(#identifiers.eq(#_identifiers) &&)* true));
                        quote_spanned! {
                            variant.span() =>
                                (Self::#name {
                                    #(#identifiers)*
                                }, Self::#name {
                                    #(#identifiers: #_identifiers)*
                                }) => #custom_eqs,
                        }
                    }
                    Fields::Unnamed(unnamed) => {
                        let identifiers: Vec<_> = unnamed
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|x| {
                                Ident::new(
                                    ("a".to_string() + &x.0.to_string()).as_str(),
                                    x.1.span(),
                                )
                            })
                            .collect();
                        let _identifiers: Vec<_> = identifiers
                            .iter()
                            .map(|x| format_ident!("{}_", x))
                            .collect();
                        let custom_eqs = &variant
                            .attrs
                            .iter()
                            .find_map(|x| {
                                if !x
                                    .path()
                                    .get_ident()
                                    .map(|a| a.eq("custom_eq"))
                                    .unwrap_or(false)
                                {
                                    return None;
                                }
                                x.parse_args::<Expr>().ok().map(|x| x.to_token_stream())
                            })
                            .unwrap_or(quote!(#(#identifiers.eq(#_identifiers) &&)* true));
                        quote_spanned! {
                            variant.span() =>
                                (Self::#name(
                                    #(#identifiers)*
                                ), Self::#name(
                                    #(#_identifiers)*
                                )) => #custom_eqs,
                        }
                    }
                    Fields::Unit => quote_spanned! {
                        variant.span() =>
                            (Self::#name, Self::#name) => true,

                    },
                };
                tokens.push(fields);
            }
            let eq_ts = if is_fully_eq {
                quote! {
                    impl #impl_generics Eq for #type_name #generics #where_clauses {}
                }
            } else {
                quote!()
            };
            quote! {
                impl #impl_generics PartialEq for #type_name #generics #where_clauses {
                    fn eq(&self, other: &Self) -> bool {
                        match (self, other) {
                            #(#tokens)*
                            #[allow(unreachable_pattern)]
                            _ => false,
                        }
                    }
                }
                #eq_ts
            }
            .into()
        }
        Data::Union(_data_union) => todo!(),
    }
}

#[proc_macro_derive(UnwrapEnum, attributes(unwrap_enum))]
pub fn derive_unwrap_enum(input: TokenStream) -> TokenStream {
    let global_ident = get_crate_name_of("pure_lang_global", Span::call_site());
    let leading_colon2 = if global_ident.eq("crate") {
        None
    } else {
        Some(quote!(::))
    };
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, generics, where_clauses) = ast.generics.split_for_impl();
    let Data::Enum(data) = &ast.data else {
        return quote! {
            compile_error!("only enums are supported");
        }
        .into();
    };
    let mut tokens = Vec::new();
    let mut _enable_owned = false;
    let mut _enable_ref = false;
    let mut _enable_ref_mut = false;
    let mut _enable_try = false;
    if let Some(attr) = ast.attrs.iter().find(|x| {
        x.path()
            .get_ident()
            .map(|x| x.eq("unwrap_enum"))
            .unwrap_or(false)
    }) {
        let t = match attr.meta.require_list() {
            Ok(o) => o,
            Err(e) => return e.into_compile_error().into(),
        }
        .tokens
        .clone();
        for meta in t {
            match meta {
                proc_macro2::TokenTree::Ident(ident) => {
                    if ident.eq("ref") {
                        _enable_ref = true;
                    } else if ident.eq("ref_mut") {
                        _enable_ref_mut = true;
                    } else if ident.eq("owned") {
                        _enable_owned = true;
                    } else if ident.eq("try") {
                        _enable_try = true;
                    } else {
                        return quote!(compile_error!("unknown ident");).into();
                    }
                }
                proc_macro2::TokenTree::Punct(ref p) => {
                    if p.as_char().ne(&',') {
                        let msg = format!("abnormal meta: {meta}");
                        let msg = LitStr::new(&msg, Span::call_site());
                        return quote!(compile_error!(#msg);).into();
                    }
                }
                _ => {
                    let msg = format!("abnormal meta: {meta}");
                    let msg = LitStr::new(&msg, Span::call_site());
                    return quote!(compile_error!(#msg);).into();
                }
            }
        }
    }

    for variant in &data.variants {
        let name = &variant.ident;
        let mut enable_owned = _enable_owned;
        let mut enable_ref = _enable_ref;
        let mut enable_ref_mut = _enable_ref_mut;
        let mut enable_try = _enable_try;
        if let Some(attr) = variant.attrs.iter().find(|x| {
            x.path()
                .get_ident()
                .map(|x| x.eq("unwrap_enum"))
                .unwrap_or(false)
        }) {
            let t = match attr.meta.require_list() {
                Ok(o) => o,
                Err(e) => return e.into_compile_error().into(),
            }
            .tokens
            .clone();
            for meta in t {
                match meta {
                    proc_macro2::TokenTree::Ident(ident) => {
                        if ident.eq("ref") {
                            enable_ref = true;
                        } else if ident.eq("ref_mut") {
                            enable_ref_mut = true;
                        } else if ident.eq("owned") {
                            enable_owned = true;
                        } else if ident.eq("try") {
                            enable_try = true;
                        } else {
                            return quote!(compile_error!("unknown ident");).into();
                        }
                    }
                    proc_macro2::TokenTree::Punct(ref p) => {
                        if p.as_char().ne(&',') {
                            let msg = format!("abnormal meta: {meta}");
                            let msg = LitStr::new(&msg, Span::call_site());
                            return quote!(compile_error!(#msg);).into();
                        }
                    }
                    _ => {
                        let msg = format!("abnormal meta: {meta}");
                        let msg = LitStr::new(&msg, Span::call_site());
                        return quote!(compile_error!(#msg);).into();
                    }
                }
            }
        }

        let (ref_impl, ref_mut_impl, owned_impl) = match &variant.fields {
            Fields::Named(_fields_named) => todo!(),
            Fields::Unnamed(fields_unnamed) => {
                let identifiers: Vec<_> = fields_unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|x| Ident::new(("a".to_string() + &x.0.to_string()).as_str(), x.1.span()))
                    .collect();
                let ref_impl = if enable_ref {
                    let mut ret_ty = proc_macro2::TokenStream::new();
                    Paren::default().surround(&mut ret_ty, |x| {
                        let mut ts = proc_macro2::TokenStream::new();
                        for field in &fields_unnamed.unnamed {
                            ts.extend_one(quote!(&));
                            ts.extend_one(field.ty.to_token_stream());
                            ts.extend_one(Token![,](Span::call_site()).into_token_stream());
                        }
                        x.extend_one(ts);
                    });
                    let success = if enable_try {
                        quote!(Ok((#(#identifiers, )*)))
                    } else {
                        quote!((#(#identifiers, )*))
                    };
                    let fallback = if enable_try {
                        quote!(Err(#leading_colon2 #global_ident::errors::UnwrapError.into()))
                    } else {
                        quote!(panic!("call unwrap at incorrect value"))
                    };
                    if enable_try {
                        ret_ty = quote!(#leading_colon2 #global_ident::Result<#ret_ty>);
                    }

                    let fn_name = format_ident!(
                        "unwrap_{ident}_ref",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(&self) -> #ret_ty {
                            match self {
                                Self::#name(#(#identifiers, )*) => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                let ref_mut_impl = if enable_ref_mut {
                    let mut ret_ty = proc_macro2::TokenStream::new();
                    Paren::default().surround(&mut ret_ty, |x| {
                        let mut ts = proc_macro2::TokenStream::new();
                        for field in &fields_unnamed.unnamed {
                            ts.extend_one(quote!(&mut));
                            ts.extend_one(field.ty.to_token_stream());
                            ts.extend_one(Token![,](Span::call_site()).into_token_stream());
                        }
                        x.extend_one(ts);
                    });
                    let success = if enable_try {
                        quote!(Ok((#(#identifiers, )*)))
                    } else {
                        quote!((#(#identifiers, )*))
                    };
                    let fallback = if enable_try {
                        quote!(Err(#leading_colon2 #global_ident::errors::UnwrapError.into()))
                    } else {
                        quote!(panic!("call unwrap at incorrect value"))
                    };
                    if enable_try {
                        ret_ty = quote!(#leading_colon2 #global_ident::Result<#ret_ty>);
                    }
                    let fn_name = format_ident!(
                        "unwrap_{ident}_mut",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(&mut self) -> #ret_ty {
                            match self {
                                Self::#name(#(#identifiers, )*) => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                let owned_impl = if enable_owned {
                    let mut ret_ty = proc_macro2::TokenStream::new();
                    Paren::default().surround(&mut ret_ty, |x| {
                        let mut ts = proc_macro2::TokenStream::new();
                        for field in &fields_unnamed.unnamed {
                            ts.extend_one(field.ty.to_token_stream());
                            ts.extend_one(Token![,](Span::call_site()).into_token_stream());
                        }
                        x.extend_one(ts);
                    });
                    let success = if enable_try {
                        quote!(Ok((#(#identifiers, )*)))
                    } else {
                        quote!((#(#identifiers, )*))
                    };
                    let fallback = if enable_try {
                        quote!(Err(#leading_colon2 #global_ident::errors::UnwrapError.into()))
                    } else {
                        quote!(panic!("call unwrap at incorrect value"))
                    };
                    if enable_try {
                        ret_ty = quote!(#leading_colon2 #global_ident::Result<#ret_ty>);
                    }
                    let fn_name = format_ident!(
                        "unwrap_{ident}",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(self) -> #ret_ty {
                            match self {
                                Self::#name(#(#identifiers, )*) => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                (ref_impl, ref_mut_impl, owned_impl)
            }
            Fields::Unit => {
                let mut ret_ty = quote!(());
                let success = if enable_try {
                    quote!(Ok(()))
                } else {
                    quote!(())
                };
                let fallback = if enable_try {
                    quote!(Err(#leading_colon2 #global_ident::errors::UnwrapError.into()))
                } else {
                    quote!(panic!("call unwrap at incorrect value"))
                };
                if enable_try {
                    ret_ty = quote!(#leading_colon2 #global_ident::Result<#ret_ty>);
                }
                let ref_impl = if enable_ref {
                    let fn_name = format_ident!(
                        "unwrap_{ident}_ref",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(&self) -> #ret_ty {
                            match self {
                                Self::#name => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                let ref_mut_impl = if enable_ref_mut {
                    let fn_name = format_ident!(
                        "unwrap_{ident}_mut",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(&mut self) -> #ret_ty {
                            match self {
                                Self::#name => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                let owned_impl = if enable_owned {
                    let fn_name = format_ident!(
                        "unwrap_{ident}",
                        ident = name.to_string().to_case(Case::Snake)
                    );
                    Some(quote! {
                        pub fn #fn_name(self) -> #ret_ty {
                            match self {
                                Self::#name => #success,
                                _ => #fallback,
                            }
                        }
                    })
                } else {
                    None
                };
                (ref_impl, ref_mut_impl, owned_impl)
            }
        };
        tokens.push(ref_impl);
        tokens.push(ref_mut_impl);
        tokens.push(owned_impl);
    }
    quote! {
        #[allow(unreachable_code)] // omit warnings for `!` and other unreachable types
        #[automatically_derived]
        impl #impl_generics #type_name #generics #where_clauses {
            #(#tokens)*
        }
    }
    .into()
}
