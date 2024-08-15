use core::time::Duration;

use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn TimedProgressBar(#[prop(default = Some(0.))] start: Option<f64>) -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(start.unwrap_or_default()));

    set_interval(
        move || {
            set_progress.update(|p| *p = p.map(|v| v + 1.));
        },
        Duration::from_secs(1),
    );

    view! { <ProgressBar progress=progress/> }
}
