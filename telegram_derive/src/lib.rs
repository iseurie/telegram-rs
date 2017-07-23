extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use std::u32;

use proc_macro::TokenStream;
use syn::{Attribute, Body, Field, Lit, MetaItem, StrStyle, VariantData};

#[proc_macro_derive(Serialize, attributes(id))]
pub fn serialize(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_serialize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

enum BodyType {
    Struct,
    Enum,
}

fn impl_serialize(ast: &syn::DeriveInput) -> quote::Tokens {
    let item_name = &ast.ident;

    let full_serialize_to_body = match ast.body {
        Body::Struct(ref data) => {
            match *data {
                VariantData::Struct(ref fields) => {
                    impl_serialize_to_body(BodyType::Struct, &ast.attrs, Some(fields))
                }

                VariantData::Tuple(_) => unreachable!(),

                VariantData::Unit => impl_serialize_to_body(BodyType::Struct, &ast.attrs, None),
            }
        }

        Body::Enum(ref variants) => {
            let mut tokens_variants = quote::Tokens::new();

            for variant in variants {
                let ref variant_name = variant.ident;

                match variant.data {
                    VariantData::Struct(ref fields) => {
                        let serialize_to_body =
                            impl_serialize_to_body(BodyType::Enum, &variant.attrs, Some(fields));

                        let quoted_fields = fields
                            .iter()
                            .map(|field| match field.ident {
                                Some(ref ident) => quote! { ref #ident },
                                None => unreachable!(),
                            })
                            .collect::<Vec<_>>();

                        tokens_variants.append(quote! {
                            #item_name::#variant_name { #(#quoted_fields),* } => {
                                #serialize_to_body
                            },
                        });
                    }

                    VariantData::Tuple(_) => unreachable!(),

                    VariantData::Unit => {
                        let serialize_to_body =
                            impl_serialize_to_body(BodyType::Enum, &variant.attrs, None);

                        tokens_variants.append(quote! {
                            #item_name::#variant_name => {
                                #serialize_to_body
                            },
                        });
                    }
                }
            }

            quote! {
                match *self {
                    #tokens_variants
                }
            }
        }
    };

    quote! {
        impl ::ser::Serialize for #item_name {
            fn serialize_to(&self, buffer: &mut Vec<u8>) -> ::error::Result<()> {
                #full_serialize_to_body
            }
        }
    }
}

fn impl_serialize_to_body(
    body_type: BodyType,
    attrs: &[Attribute],
    fields: Option<&[Field]>,
) -> quote::Tokens {
    let mut id = None;

    for attr in attrs {
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

    let mut properties = Vec::new();

    if let Some(fields) = fields {
        for field in fields {
            if let Some(ref field_name) = field.ident {
                let property = match body_type {
                    BodyType::Struct => {
                        quote! {
                            self.#field_name.serialize_to(buffer)?;
                        }
                    }

                    BodyType::Enum => {
                        quote! {
                            #field_name.serialize_to(buffer)?;
                        }
                    }
                };

                properties.push(property);
            }
        }
    }

    quote! {
        // Identifier
        #id

        // Properties
        #(#properties)*

        Ok(())
    }
}
