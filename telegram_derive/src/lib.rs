extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use std::u32;

use proc_macro::TokenStream;
use syn::{VariantData, Body, MetaItem, Lit, StrStyle};

#[proc_macro_derive(Serialize, attributes(id))]
pub fn serialize(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_serialize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_serialize(ast: &syn::MacroInput) -> quote::Tokens {
    let mut properties = Vec::new();

    match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) => {
            for field in fields {
                if let Some(ref field_name) = field.ident {
                    properties.push(quote! {
                        self.#field_name.serialize_to(buffer)?;
                    });
                }
            }
        }

        _ => {
            // Do nothing
        }
    }

    let mut id = None;

    for attr in &ast.attrs {
        match attr.value {
            MetaItem::NameValue(ref name, ref value) => {
                if name.as_ref() == "id" {
                    if let Lit::Str(ref value, StrStyle::Cooked) = *value {
                        // Found an identifier
                        let value = u32::from_str_radix(&value[2..], 16).unwrap();
                        id = Some(quote! {
                            #value.serialize_to(buffer)?;
                        });

                        break;
                    }
                }
            }

            _ => {
                // Do nothing
            }
        }
    }

    let name = &ast.ident;

    quote! {
        impl ::ser::Serialize for #name {
            fn serialize_to(&self, buffer: &mut Vec<u8>) -> ::error::Result<()> {
                // Identifier
                #id

                // Properties
                #(#properties)*

                Ok(())
            }
        }
    }
}
