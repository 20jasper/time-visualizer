use core::time::Duration;
use instant::Instant;

use leptonic::prelude::*;
use leptos::*;

fn format_time(duration: Duration) -> String {
    let length = duration;

    let seconds_str = if length.as_secs() == 0 {
        "".into()
    } else {
        format!("{} seconds,", length.as_secs())
    };

    format!(
        "length: {seconds_str} {} milliseconds",
        length.subsec_millis()
    )
}

#[component]
pub fn TimedProgressBar(
    #[prop(default = Instant::now())] start_time: Instant,
    #[prop(default = Duration::from_millis(100))] interval: Duration,
    title: String,
    length: Duration,
) -> impl IntoView {
    let percentage_done =
        move || 100. * ((start_time.elapsed().as_micros()) as f64 / (length.as_micros()) as f64);

    let (progress, set_progress) = create_signal(Some(0.));
    let progress_remainder = create_memo(move |_| progress.get().map(|x| x % 100.));
    let loops = move || (progress.get().unwrap_or_default() as u32) / 100;

    set_interval(
        move || {
            set_progress.set(Some(percentage_done()));
        },
        interval,
    );

    view! {
        <H2>{title}</H2>
        <H3>{format_time(length)}</H3>
        <P>
            loop:
            {loops}
        </P>
        <ProgressBar progress=progress_remainder/>
    }
}
