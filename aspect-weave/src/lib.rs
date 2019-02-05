// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::rc::Rc;
use syn::parse::Parse;
use syn::Result;
use synattra::ParseAttributes;

pub trait Weave: ParseAttributes {
    type MacroAttributes: Parse;

    fn parse_macro_attributes(attrs: TokenStream) -> syn::Result<Self::MacroAttributes> {
        Ok(syn::parse(attrs)?)
    }

    fn update_fn_block(
        fn_def: &syn::ImplItemMethod,
        main_attr: &Self::MacroAttributes,
        fn_attr: &[Rc<<Self as ParseAttributes>::Type>],
    ) -> Result<syn::Block>;
}

use indexmap::IndexMap;
pub struct WovenImplBlock<M, F> {
    pub woven_block: syn::ItemImpl,
    pub main_attributes: M,
    pub woven_fns: IndexMap<syn::Ident, Vec<Rc<F>>>,
}

pub fn weave_impl_block<W: Weave>(
    attrs: TokenStream,
    item: TokenStream,
) -> Result<WovenImplBlock<W::MacroAttributes, <W as ParseAttributes>::Type>> {
    let main_attributes = W::parse_macro_attributes(attrs)?;

    let mut parsed_input: syn::ItemImpl = syn::parse(item)?;
    let mut attrs = &mut parsed_input.attrs;
    let main_extra_attributes: Vec<Rc<<W as ParseAttributes>::Type>> =
        process_custom_attributes::<W, _, _>(&mut attrs, Rc::new)?;

    let mut woven = indexmap::map::IndexMap::new();

    for item in parsed_input.items.iter_mut() {
        if let syn::ImplItem::Method(item_fn) = item {
            let mut attrs = &mut item_fn.attrs;

            let method_attrs = process_custom_attributes::<W, _, _>(&mut attrs, Rc::new)?;

            if method_attrs.is_empty() {
                continue;
            }

            let mut fn_attributes: Vec<Rc<<W as ParseAttributes>::Type>> =
                main_extra_attributes.clone();
            fn_attributes.extend(method_attrs);

            item_fn.block = W::update_fn_block(item_fn, &main_attributes, &fn_attributes)?;

            woven.insert(item_fn.sig.ident.clone(), fn_attributes);
        }
    }

    Ok(WovenImplBlock {
        woven_block: parsed_input,
        main_attributes: main_attributes,
        woven_fns: woven,
    })
}

fn process_custom_attributes<W: ParseAttributes, R, F: Fn(W::Type) -> R>(
    attrs: &mut Vec<syn::Attribute>,
    f: F,
) -> Result<Vec<R>> {
    let (ours, theirs): (Vec<syn::Attribute>, Vec<syn::Attribute>) = attrs
        .clone()
        .into_iter()
        .partition(|attr| attr.path.is_ident(W::fn_attr_name()));

    *attrs = theirs;

    let mut fn_attributes: Vec<R> = Vec::new();
    for attr in ours.into_iter() {
        let p = W::parse_attributes(attr.tts)?;
        fn_attributes.push(f(p));
    }

    Ok(fn_attributes)
}
