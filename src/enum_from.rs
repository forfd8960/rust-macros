use quote::quote;

pub(crate) fn procee_enum_from_input(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_name = input.ident;

    // get generics
    let generics = input.generics;

    // get enum variant
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("only works for enum"),
    };

    //for each variant, get the ident and fields
    let from_impls = variants.iter().map(|var| {
        let variant_name = &var.ident;
        match &var.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().unwrap();
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #enum_name #generics {
                            fn from(v: #ty) -> Self {
                                #enum_name::#variant_name(v)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
}
