use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromMeta)]
struct DetectorArgs {
    #[darling(default)]
    id: Option<String>,
    name: String,
    description: String,
    category: syn::Path,
    #[darling(default)]
    default_enabled: Option<bool>,
    #[darling(default)]
    is_deep: Option<bool>,
    #[darling(default)]
    smell_type: Option<syn::Path>,
}

#[proc_macro_attribute]
pub fn detector(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        proc_macro2::TokenStream::from(args),
    ) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    let input_struct = parse_macro_input!(input as DeriveInput);

    let nested_meta: Vec<syn::Meta> = attr_args.into_iter().collect();

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

    let name = args.name;
    let description = args.description;
    let category = args.category;
    let default_enabled = args.default_enabled.unwrap_or(true);
    let is_deep = args.is_deep.unwrap_or(false);

    let id_tokens = if let Some(smell_type) = args.smell_type {
        quote! { crate::detectors::ConfigurableSmellType::#smell_type.to_id() }
    } else {
        let id = args.id;
        quote! { #id }
    };

    let expanded = quote! {
        #input_struct

        pub struct #factory_name;

        impl crate::detectors::DetectorFactory for #factory_name {
            fn info(&self) -> crate::detectors::DetectorInfo {
                crate::detectors::DetectorInfo {
                    id: #id_tokens,
                    name: #name,
                    description: #description,
                    default_enabled: #default_enabled,
                    is_deep: #is_deep,
                    category: #category,
                }
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
