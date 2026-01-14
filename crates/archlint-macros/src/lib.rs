use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromMeta)]
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

#[proc_macro_attribute]
pub fn detector(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_cloned = proc_macro2::TokenStream::from(args.clone());
    let mut attr_args_list = Vec::new();

    // Try to parse as meta list first
    match syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        args_cloned.clone(),
    ) {
        Ok(meta_list) => {
            // Check if first item is a path (positional argument like SmellType::Variant)
            if let Some(first) = meta_list.first() {
                if let syn::Meta::Path(path) = first {
                    // First argument is a path, treat it as smell_type
                    attr_args_list.push(syn::Meta::NameValue(syn::MetaNameValue {
                        path: syn::parse_quote!(smell_type),
                        value: syn::Expr::Path(syn::ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path.clone(),
                        }),
                        eq_token: syn::token::Eq::default(),
                    }));
                    // Add remaining arguments
                    for meta in meta_list.iter().skip(1) {
                        attr_args_list.push(meta.clone());
                    }
                } else {
                    // Regular named arguments
                    attr_args_list.extend(meta_list);
                }
            }
        }
        Err(_) => {
            // Try to parse as a single path (e.g., #[detector(SmellType::Variant)])
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
                return TokenStream::from(
                    syn::Error::new_spanned(
                        proc_macro2::TokenStream::from(args),
                        "Invalid detector attribute arguments",
                    )
                    .to_compile_error(),
                );
            }
        }
    }

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

    let struct_name = &input_struct.ident;
    let factory_name = quote::format_ident!("{}Factory", struct_name);

    let default_enabled = args.default_enabled.unwrap_or(true);
    let is_deep = args.is_deep.unwrap_or(false);

    let (id_tokens, name_tokens, description_tokens, category_tokens) = if let Some(smell_type) =
        args.smell_type
    {
        let variant = &smell_type.segments.last().unwrap().ident;
        let id = quote! { crate::detectors::SmellKind::#variant.to_id() };
        let name = if let Some(n) = args.name {
            quote! { #n }
        } else {
            quote! { crate::detectors::SmellKind::#variant.display_name() }
        };
        let description = if let Some(d) = args.description {
            quote! { #d }
        } else {
            quote! { {
                use strum::EnumProperty;
                crate::detectors::SmellKind::#variant.get_str("description").unwrap_or_else(|| crate::detectors::SmellKind::#variant.display_name())
            } }
        };
        let category = if let Some(c) = args.category {
            quote! { #c }
        } else {
            quote! { crate::detectors::SmellKind::#variant.default_category() }
        };
        (id, name, description, category)
    } else {
        let id = args.id;
        let name = args.name;
        let description = args.description;
        let category = args.category;
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
