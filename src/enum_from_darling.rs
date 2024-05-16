use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariant {
    ident: syn::Ident,
    fields: Fields<EnumVariantField>,
}

#[derive(Debug, FromField)]
struct EnumVariantField {
    ty: syn::Type,
}

pub(crate) fn process_from_darling(input: DeriveInput) -> TokenStream {
    let Ok(EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    }) = EnumFromDarling::from_derive_input(&input)
    else {
        panic!("");
    };

    // println!("{:#?}", ident);
    // println!("{:#?}", generics);
    // println!("{:#?}", data);

    let impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;

        match style {
            Style::Tuple => {
                let field = variant.fields.iter().next().unwrap();
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {#(#impls)*}
}
