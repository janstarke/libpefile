extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StructFromBytes)]
pub fn from_bytes_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_from_bytes(&ast)
}

fn impl_from_bytes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl StructFromBytes for #name {
            fn from_bytes(bytes: &[u8]) -> std::io::Result<Box<Self>> {
                match Self::unpack(bytes.try_into().expect("slice with incorrect length")) {
                    Ok(v)    => Ok(Box::new(v)),
                    Err(why) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{:?}", why)))
                }
            }
        }
    };
    gen.into()
}