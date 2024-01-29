> [!CAUTION]
> This Repository is no longer maintained, as the associated PR has been closed. It's archived to prevent breaking dependent projects, but should not be used.
---
# Bevy State Matching Prototype

![Bevy Logo State Matching Variant](/assets/icon.png)

This crate is an implementation of the [State Pattern Matching - Alternative](https://github.com/bevyengine/bevy/pull/10088) PR as a third party crate, to provide access to test out the feature in `0.12` before it gets merged in for, hopefully, `0.13`.

NOTE: much of the code is directly copied and pasted from the Bevy repository, and while I implemented the changes in the PR - there are many others who were involved in creating the original code. As such, despite this being a 3rd party repository, I'd consider all the code here to be Bevy's rather than mine - I'm just providing early access to this API.

## Differences from the PR

Due to being a 3rd party crate, there are some added limitations on what we can do, which result in some API differences:

- we can't auto-implement `IntoSystem` for state matchers. As a result, we have opted to add a `run_in<S: States, Sm: StateMatcher<S>>(Sm)` to all implementors of `IntoSystem` - that way it'll just be a search and replace when migrating to `0.13` - assuming the API remains intact. In addition, I added an `and_then` function to `StateMatcher<S>` directly, but it won't support conditions - only other matchers. If you want both state matchers  and other conditions, you will have to chain `run_in` and `run_if` commands.
- we can't replace existing types with the same name - so the enum version of `NextState` has been renamed `NextMatchableState`
- we can't replace the existing `add_state` with one that triggers the `Entering` & `Exiting` states when states change, or uses the updated `NextMatchableState` implementation, so we added an `add_matchable_state` method for that purpose.

## Installation

To install, you currently need to specify the github repo in `Cargo.toml`:

```toml
[dependencies]
bevy = "0.12",
bevy_state_matching_prototype = { git = "https://github.com/lee-orr/bevy-state-match-prototype" }
```
