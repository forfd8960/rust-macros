use darling::{
    ast::{self, Data},
    FromDeriveInput, FromField,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(debug))]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), FieldsInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let Ok(AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    }) = AutoDebugInfo::from_derive_input(&input)
    else {
        panic!("AutoDebug macro only apply for struct")
    };

    println!("name: {}", ident);
    println!("fields: {:#?}", fields);

    let debug_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let skip = f.skip;
        if skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        impl std::fmt::Debug for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                #(#debug_fields)*
                .finish()
            }
        }
    }
}
