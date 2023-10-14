extern crate proc_macro;

mod state_matchers;
use proc_macro::TokenStream;
use state_matchers::state_matches_macro;

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
