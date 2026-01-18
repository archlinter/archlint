use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromMeta)]
#[allow(clippy::option_if_let_else)]
struct DetectorArgs {
    #[darling(default)]
    id: Option<String>,
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    description: Option<String>,
    #[darling(default)]
    category: Option<syn::Path>,
    #[darling(default)]
    default_enabled: Option<bool>,
    #[darling(default)]
    is_deep: Option<bool>,
    #[darling(default)]
    smell_type: Option<syn::Path>,
}

fn parse_attr_args(args: TokenStream) -> Result<Vec<syn::Meta>, TokenStream> {
    let args_cloned = proc_macro2::TokenStream::from(args.clone());
    let mut attr_args_list = Vec::new();
    let meta_list = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        args_cloned.clone(),
    );

    match meta_list {
        Ok(meta_list) => {
            if let Some(first) = meta_list.first() {
                if let syn::Meta::Path(path) = first {
                    attr_args_list.push(syn::Meta::NameValue(syn::MetaNameValue {
                        path: syn::parse_quote!(smell_type),
                        value: syn::Expr::Path(syn::ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path.clone(),
                        }),
                        eq_token: syn::token::Eq::default(),
                    }));
                    for meta in meta_list.iter().skip(1) {
                        attr_args_list.push(meta.clone());
                    }
                } else {
                    attr_args_list.extend(meta_list);
                }
            }
        }
        Err(_) => {
            if let Ok(path) = syn::parse2::<syn::Path>(args_cloned) {
                attr_args_list.push(syn::Meta::NameValue(syn::MetaNameValue {
                    path: syn::parse_quote!(smell_type),
                    value: syn::Expr::Path(syn::ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path,
                    }),
                    eq_token: syn::token::Eq::default(),
                }));
            } else {
                return Err(TokenStream::from(
                    syn::Error::new_spanned(
                        proc_macro2::TokenStream::from(args),
                        "Invalid detector attribute arguments",
                    )
                    .to_compile_error(),
                ));
            }
        }
    }

    Ok(attr_args_list)
}

#[proc_macro_attribute]
pub fn detector(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args_list = match parse_attr_args(args) {
        Ok(list) => list,
        Err(err) => return err,
    };
    let input_struct = parse_macro_input!(input as DeriveInput);
    let nested_meta: Vec<syn::Meta> = attr_args_list.into_iter().collect();

    let args = match DetectorArgs::from_list(
        &nested_meta
            .iter()
            .map(|m| darling::ast::NestedMeta::Meta(m.clone()))
            .collect::<Vec<_>>(),
    ) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let DetectorArgs {
        id,
        name,
        description,
        category,
        default_enabled,
        is_deep,
        smell_type,
    } = args;

    let struct_name = &input_struct.ident;
    let factory_name = quote::format_ident!("{}Factory", struct_name);

    let default_enabled = default_enabled.unwrap_or(true);
    let is_deep = is_deep.unwrap_or(false);

    let (id_tokens, name_tokens, description_tokens, category_tokens) = if let Some(smell_type) =
        smell_type
    {
        let Some(variant) = smell_type.segments.last().map(|segment| &segment.ident) else {
            return TokenStream::from(
                syn::Error::new_spanned(smell_type, "smell_type must be a non-empty path")
                    .to_compile_error(),
            );
        };
        let id_tokens = quote! { crate::detectors::SmellKind::#variant.to_id() };
        let name_tokens = name.map_or_else(
            || quote! { crate::detectors::SmellKind::#variant.display_name() },
            |n| quote! { #n },
        );
        let description_tokens = description.map_or_else(
            || {
                quote! { {
                    use strum::EnumProperty;
                    crate::detectors::SmellKind::#variant.get_str("description").unwrap_or_else(|| crate::detectors::SmellKind::#variant.display_name())
                } }
            },
            |d| quote! { #d },
        );
        let category_tokens = category.map_or_else(
            || quote! { crate::detectors::SmellKind::#variant.default_category() },
            |c| quote! { #c },
        );
        (id_tokens, name_tokens, description_tokens, category_tokens)
    } else {
        (
            quote! { #id },
            quote! { #name },
            quote! { #description },
            quote! { #category },
        )
    };

    let expanded = quote! {
        #input_struct

        impl #struct_name {
            pub fn metadata() -> crate::detectors::DetectorInfo {
                crate::detectors::DetectorInfo {
                    id: #id_tokens,
                    name: #name_tokens,
                    description: #description_tokens,
                    default_enabled: #default_enabled,
                    is_deep: #is_deep,
                    category: #category_tokens,
                }
            }
        }

        pub struct #factory_name;

        impl crate::detectors::DetectorFactory for #factory_name {
            fn info(&self) -> crate::detectors::DetectorInfo {
                #struct_name::metadata()
            }

            fn create(&self, config: &crate::config::Config) -> Box<dyn crate::detectors::Detector> {
                Box::new(#struct_name::new_default(config))
            }
        }

        inventory::submit! {
            &#factory_name as &dyn crate::detectors::DetectorFactory
        }
    };

    TokenStream::from(expanded)
}
