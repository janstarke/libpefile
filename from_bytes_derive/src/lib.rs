extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Index};
use std::mem::size_of;
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


#[proc_macro_derive(PackedSize_u8)]
pub fn packed_size_derive_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_packed_size(size_of::<u8>(), input)
}

#[proc_macro_derive(PackedSize_u16)]
pub fn packed_size_derive_u16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_packed_size(size_of::<u16>(), input)
}

#[proc_macro_derive(PackedSize_u32)]
pub fn packed_size_derive_u32(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_packed_size(size_of::<u32>(), input)
}

fn static_packed_size(size: usize, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let expanded = quote! {
        impl PackedSize for #name {
            fn packed_size() -> usize {
                #size
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(PackedSize)]
pub fn packed_size_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let sum = packed_bytes_sum(&ast.data);

    //println!("{}", sum);

    let expanded = quote! {
        impl PackedSize for #name {
            fn packed_size() -> usize {
                #sum
            }
        }
    };
    expanded.into()
}

fn packed_bytes_sum(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! {f.span() =>
                            <#name>::packed_size()
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            &self.#index::PackedSize::packed_size()
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}