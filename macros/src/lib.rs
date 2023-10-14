extern crate proc_macro;

mod component;
mod fetch;
mod set;
mod state_matchers;
mod states;

use crate::{fetch::derive_world_query_impl, set::derive_set};
use bevy_macro_utils::{
    derive_boxed_label, ensure_no_collision, get_named_struct_fields, BevyManifest,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use state_matchers::state_matches_macro;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma,
    ConstParam, DeriveInput, GenericParam, Ident, Index, TypeParam,
};

/// Run a system only if the current state matches the provided expressions.
///
/// This can be done by:
/// - using matching pattern, like so `state_matches!(AppState, InGame { .. })`. Note that when matching
/// enums, you do  not need to repeat the type within the pattern.
/// - using a closure with a type that automatically implements `StateMatcher<S>`, like so `state_matches!(AppState, |state| { /// some logic here - return a bool})`
/// - using an expression preceded by a `=`, like so `state_matches!(=AppState::Menu)`
///
/// You can also add additional comma-separated expressions, patterns or closures - which will be evaluated in order.
#[proc_macro]
pub fn state_matches(input: TokenStream) -> TokenStream {
    let result =
        state_matchers::define_match_macro(input).expect("Couldn't parse `state_matches!`");
    state_matches_macro(result)
}
