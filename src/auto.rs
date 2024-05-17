use darling::{
    ast::{self, Data},
    FromDeriveInput, FromField,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), FieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let Ok(AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    }) = AutoDerefInfo::from_derive_input(&input)
    else {
        panic!("")
    };

    println!("{:?}", ident);
    println!("{:?}", generics);
    println!("mutable {:#?}", mutable);
    println!("field {:#?}", field);

    let (f, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => panic!(""),
        }
    } else if fields.len() == 1 {
        let f = fields.iter().next().unwrap();
        (f.ident.as_ref().unwrap().clone(), &f.ty)
    } else {
        panic!("Autoderef only works for 1 field")
    };

    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #ty;
            fn deref(&self) -> &Self::Target {
                &self.#f
            }
        }
    }];

    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#f
            }
        }
        });
    }

    quote! {#(#code)*}
}

pub(crate) fn process_auto_debug(_input: DeriveInput) -> TokenStream {
    quote! {}
}
