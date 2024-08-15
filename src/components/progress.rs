use core::{num::ParseIntError, time::Duration};
use instant::Instant;

use leptonic::components::prelude::TableCell;
use leptonic::components::prelude::{
    Box, ProgressBar, Table, TableBody, TableHeader, TableHeaderCell, TableRow,
};
use leptos::*;
use leptos_router::use_query_map;

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
        <TableCell>{loops}</TableCell>
        <TableCell>
            <ProgressBar progress=progress_remainder/>
        </TableCell>
    }
}

struct TimedProgressParams {
    title: String,
    length: Duration,
}

impl TimedProgressParams {
    fn build(title: String, length: String) -> Result<TimedProgressParams, ParseIntError> {
        Ok(Self {
            title,
            length: Duration::from_millis(length.parse()?),
        })
    }
}

#[component]
pub fn TimedProgressContainer() -> impl IntoView {
    view! {
        <Box class="timed-container">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"Title"</TableHeaderCell>
                        <TableHeaderCell>"Length"</TableHeaderCell>
                        <TableHeaderCell min_width=true>"Loops"</TableHeaderCell>
                        <TableHeaderCell min_width=true>"Progress"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <For
                        each=move || {
                            use_query_map()
                                .get()
                                .0
                                .clone()
                                .into_iter()
                                .filter_map(|(title, length)| {
                                    TimedProgressParams::build(title, length).ok()
                                })
                        }

                        key=|x| x.title.clone()
                        children=move |TimedProgressParams { title, length }| {
                            view! {
                                <TableRow>
                                    <TableCell>{title}</TableCell>
                                    <TableCell>{format_time(length)}</TableCell>
                                    <TimedProgressBar length=length/>
                                </TableRow>
                            }
                        }
                    />

                </TableBody>
            </Table>
        </Box>
    }
}
