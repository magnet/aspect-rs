//! # An Aspect Toolkit for Rust
//!
//! Aspect-RS is a project aiming to provide common ground for the main Aspect-Oriented use cases in Rust. By leveraging the trait system, declarative and procedural macros, Aspect-RS provides blocks that let you wrap methods with your custom logic.
//!
//! The project has been extracted from the [Metered project](https://github.com/magnet/metered-rs), which uses the technique to build metrics that can work on expressions or methods, whether they're `async` or not. The technique seemed general enough to be in its own crate and see if it is of any interest to other parties.
//!
//! Aspect-RS provides "pointcut" traits when entering or exiting an expression (`OnEnter` and `OnResult`), experimental `Update` and `UpdateRef` traits that can use parameter shadowing to intercept and update method parameters, and weaving constructs useful when building procedural macros. Please look at the [Metered project](https://github.com/magnet/metered-rs) to see Aspect-RS in action.
//!
//! This crate provides method weaving support through methods re-usable in procedural macros.

#![deny(missing_docs)]
#![deny(warnings)]
// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Delimiter;
use quote::ToTokens;
use std::rc::Rc;
use syn::parse::Parse;
use syn::{MacroDelimiter, Result};
use synattra::ParseAttributes;

/// A trait to "Weave" an `impl` block, that is update each annotated method with your custom logic
///
/// This trait extends Synattra's `ParseAttributes` that parses custom, non-macro attributes that are attached to the `impl` block or methods.
pub trait Weave: ParseAttributes {
    /// The parameters of the macro attribute triggering the weaving, i.e the attributes passed by the compiler to your custom procedural macro.
    type MacroAttributes: Parse;

    /// Parse the main macro attributes.
    ///
    /// The default implementation should work out-of-the-box.
    fn parse_macro_attributes(attrs: TokenStream) -> syn::Result<Self::MacroAttributes> {
        Ok(syn::parse(attrs)?)
    }

    /// A callback that lets you alter the blocks of intercepted methods.
    fn update_fn_block(
        fn_def: &syn::ImplItemFn,
        main_attr: &Self::MacroAttributes,
        fn_attr: &[Rc<<Self as ParseAttributes>::Type>],
    ) -> Result<syn::Block>;
}

use indexmap::IndexMap;
/// An `impl` block after it's been woven.
pub struct WovenImplBlock<M, F> {
    /// The woven `impl` block, in which individual function blocks have been updated and intercepted attributes removed.
    pub woven_block: syn::ItemImpl,
    /// The macro attributes
    pub main_attributes: M,
    /// The woven functions, along with their intercepted attributes
    pub woven_fns: IndexMap<syn::Ident, Vec<Rc<F>>>,
}

/// Weave an `impl` block
///
/// This method is meant to be called from a custom procedural macro.
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
        if let syn::ImplItem::Fn(item_fn) = item {
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
        .partition(|attr| attr.path().is_ident(W::fn_attr_name()));

    *attrs = theirs;

    let mut fn_attributes: Vec<R> = Vec::new();
    for attr in ours.into_iter() {
        let mut ts: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

        match attr.meta {
            syn::Meta::List(l) => surround(&l.delimiter, &mut ts, l.tokens.clone()),
            syn::Meta::NameValue(nv) => {
                nv.eq_token.to_tokens(&mut ts);
                nv.value.to_tokens(&mut ts)
            }
            _ => {}
        };

        let p = W::parse_attributes(ts)?;
        fn_attributes.push(f(p));
    }

    Ok(fn_attributes)
}

fn surround(
    delim: &MacroDelimiter,
    tokens: &mut proc_macro2::TokenStream,
    inner: proc_macro2::TokenStream,
) {
    let (delim, span) = match delim {
        MacroDelimiter::Paren(paren) => (Delimiter::Parenthesis, paren.span),
        MacroDelimiter::Brace(brace) => (Delimiter::Brace, brace.span),
        MacroDelimiter::Bracket(bracket) => (Delimiter::Bracket, bracket.span),
    };
    let mut g = proc_macro2::Group::new(delim, inner);
    g.set_span(span.join());
    tokens.extend(std::iter::once(g.to_token_stream()));
}
