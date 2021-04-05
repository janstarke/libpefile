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
            fn from_bytes(slice: &[u8], offset: usize) -> std::io::Result<Box<Self>> {
                let size = Self::packed_size();
                match Self::unpack_from_slice(&slice[offset..offset+size]) {
                    Ok(v)    => Ok(Box::new(v)),
                    Err(why) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{:?}", why)))
                }
            }
        }
    };
    gen.into()
}