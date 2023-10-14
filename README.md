# Bevy State Matching Prototype

This crate is an implementation of the [State Pattern Matching - Alternative](https://github.com/bevyengine/bevy/pull/10088) PR as a third party crate, to provide access to test out the feature in `0.12` before it gets merged in for, hopefully, `0.13`.

In addition, it provides a couple of optional features to test out some additional APIs, that were not included in the PR with the hopes of having 3rd part crates showing the usability of those features first. These are disabled by default.

NOTE: much of the code is directly copied and pasted from the Bevy repository, and while I implemented the changes in the PR - there are many others who were involved in creating the original code. As such, despite this being a 3rd party repository, I'd consider all the code here to be Bevy's rather than mine - I'm just providing early access to this API.

## Installation

To install, you currently need to specify the github repo in `Cargo.toml`:

```toml
[dependencies]
bevy = { git = "https://github.com/bevyengine/bevy" },
bevy_state_matching_prototype = "0.1"
```
