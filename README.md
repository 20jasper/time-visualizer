# Time Visualizer

A Leptos progress visualizer! Add query parameters in the format of `title=milliseconds` to add a progress bar

## Commands

`just --list`

## Challenges

I ran into issues with `setInterval`'s accuracy. I switched to using time elapsed to calculate the percentage finished on an interval instead. There was no `Instant` implementation for `wasm32-unknown-unknown`, so I installed the `instant` crate with the `wasm-bindgen` flag

`leptonic` and `cookie` wanted different versions of `time`. I could not resolve the issue by updating this package's dependencies, so I forked Leptonic to update the `leptos-use` and `time` crates, which worked great!
