use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, File, FnArg, Item, ItemTrait, Pat, TraitItem};

mod template;

const MAX_ITEMS: usize = 12;

pub fn gen_all() -> anyhow::Result<TokenStream> {
    let file: File = parse_str(include_str!("template.rs"))?;
    let mut uses = Vec::new();
    let mut traits = Vec::new();
    for item in file.items {
        match item {
            Item::Trait(x) => traits.push(x),
            Item::Use(x) => uses.push(x),
            _ => {}
        }
    }
    let mut defs = Vec::new();
    for n in 1..=MAX_ITEMS {
        defs.push(build_type(n, &traits));
    }
    Ok(quote! {
        #(#uses)*
        #(#defs)*
    })
}

fn build_type(len: usize, traits: &[ItemTrait]) -> TokenStream {
    let mut trait_into_iter_n = Vec::new();
    let iter_ty = format_ident!("Iter{}", len);
    for n0 in 0..len {
        let trait_name = format_ident!("IntoIter{}", n0);
        let fn_name = format_ident!("into_iter{}", n0);
        let variant = format_ident!("I{}", n0);
        let mut fn_g_arg = Vec::new();
        let mut iter_ty_arg = Vec::new();
        for n1 in 0..len {
            let g_arg = format_ident!("I{}", n1);
            if n0 == n1 {
                iter_ty_arg.push(quote!(Self::IntoIter));
            } else {
                fn_g_arg.push(quote!(#g_arg));
                iter_ty_arg.push(quote!(#g_arg));
            }
        }
        trait_into_iter_n.push(quote! {
            pub trait #trait_name : IntoIterator{
                #[allow(clippy::type_complexity)]
                fn #fn_name<#(#fn_g_arg,)*>(self) -> #iter_ty<#(#iter_ty_arg,)*>;
            }
            impl<T: IntoIterator> #trait_name for T {
                #[allow(clippy::type_complexity)]
                fn #fn_name<#(#fn_g_arg,)*>(self) -> #iter_ty<#(#iter_ty_arg,)*> {
                    #iter_ty::#variant(self.into_iter())
                }
            }
        });
    }
    let mut iter_ty_arg = Vec::new();
    let mut variant_name = Vec::new();
    let mut variant_ty = Vec::new();
    for n in 0..len {
        iter_ty_arg.push(format_ident!("I{}", n));
        variant_name.push(format_ident!("I{}", n));
        variant_ty.push(format_ident!("I{}", n));
    }
    let mut impl_traits = Vec::new();
    for t in traits {
        let mut trait_items = Vec::new();
        for item in &t.items {
            match item {
                TraitItem::Type(_) => {
                    trait_items.push(quote!(
                        type Item = Item;
                    ));
                }
                TraitItem::Fn(f) => {
                    let sig = &f.sig;
                    let fn_ident = &sig.ident;
                    let mut fn_inputs = Vec::new();
                    for input in &sig.inputs {
                        match input {
                            FnArg::Receiver(_) => {}
                            FnArg::Typed(t) => {
                                if let Pat::Ident(ident) = &*t.pat {
                                    fn_inputs.push(ident.clone());
                                }
                            }
                        }
                    }
                    let mut arms = Vec::new();
                    for n in 0..len {
                        let variant_name = &variant_name[n];
                        arms.push(quote! {
                            Self::#variant_name(this) => this.#fn_ident(#(#fn_inputs),*),
                        });
                    }
                    trait_items.push(quote! {
                        #sig {
                            match self {
                                #(#arms)*
                            }

                        }
                    });
                }
                _ => {}
            }
        }

        let trait_ident = &t.ident;
        impl_traits.push(quote! {
            impl<Item, #(#iter_ty_arg,)*> #trait_ident for #iter_ty<#(#iter_ty_arg,)*>
                where
                    #(
                        #iter_ty_arg: #trait_ident<Item = Item>,
                    )*
            {
                #(#trait_items)*
            }
        })
    }

    let mod_name = format_ident!("iter{}", len);
    quote! {
        pub mod #mod_name {
            use super::*;
            use core::iter::FusedIterator;

            #[derive(Debug, Clone, Copy)]
            pub enum #iter_ty<#(#iter_ty_arg,)*> {
                #(#variant_name(#variant_ty),)*
            }
            #(#trait_into_iter_n)*
            #(#impl_traits)*
        }
    }
}
