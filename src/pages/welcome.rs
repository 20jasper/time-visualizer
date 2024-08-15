use core::time::Duration;

use leptonic::prelude::*;
use leptos::*;

use crate::components::progress::TimedProgressBar;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <H2>"Welcome to Leptonic"</H2>
            <TimedProgressBar length=Duration::from_secs(10)/>
            <TimedProgressBar length=Duration::from_millis(60)/>
        </Box>
    }
}
