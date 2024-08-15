use core::time::Duration;

use leptonic::prelude::*;
use leptos::*;

use crate::components::progress::{TimedProgressBar, TimedProgressContainer};

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <H1>"Progress Time Visualizer"</H1>
            <TimedProgressContainer/>
        </Box>
    }
}
