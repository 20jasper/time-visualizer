use core::time::{self, Duration};

use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn TimedProgressBar(
    #[prop(default = Some(0.))] start: Option<f64>,
    length: Duration,
) -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(start.unwrap_or_default()));

    let interval = length.div_f32(100.);

    set_interval(
        move || {
            set_progress.update(|p| *p = p.map(|v| v + 1.));
        },
        interval,
    );

    let seconds_str = if length.as_secs() == 0 {
        "".into()
    } else {
        format!("{} seconds,", length.as_secs())
    };

    view! {
        <H2>{format!("length: {seconds_str} {} milliseconds", length.subsec_millis())}</H2>
        <ProgressBar progress=progress/>
    }
}
