extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let identifier = syn::Ident::new(
        format!("{}Builder", &ast.ident).as_str(),
        ast.ident.span().clone(),
    );

    let named_fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        todo!()
    };

    // Convert fields in user written struct to be wrapped in an Option<..>
    let optional_fields = named_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        quote! {
            #field_name: std::option::Option<#field_ty>
        }
    });

    let optional_fields_default = named_fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: std::option::Option::None
        }
    });

    let field_setters = named_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        quote! {
            pub fn #field_name(&mut self, value: #field_type) -> &mut Self {
                self.#field_name = Some(value);
                self
            }
        }
    });

    let struct_values = named_fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: self.#field_name.clone().ok_or(concat!(stringify!(#field_name), "is not set"))?
        }
    });

    let output: TokenStream = quote! {
        struct #identifier {
            #(#optional_fields,)*
        }

        impl #identifier {
            pub fn builder() -> Self {
                Self {
                    #(#optional_fields_default,)*
                }
            }

            #(#field_setters)*

            pub fn build(&self) -> Result<#name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#struct_values,)*
                })
            }
        }
    }
    .into();

    output
}
