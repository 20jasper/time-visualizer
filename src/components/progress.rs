use core::fmt::Display;
use core::time::Duration;
use instant::Instant;

use leptonic::components::prelude::TableCell;
use leptonic::components::prelude::{
    Box, ProgressBar, Table, TableBody, TableHeader, TableHeaderCell, TableRow,
};
use leptos::*;
use leptos_router::use_query_map;

const SECONDS_PER_MINUTE: u64 = 60;
const MINUTES_PER_HOUR: u64 = 60;
const SECONDS_PER_HOUR: u64 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;

fn format_time(duration: Duration) -> String {
    let length = duration;

    let mut s = String::new();

    let hours = length.as_secs() / SECONDS_PER_HOUR;
    if hours > 0 {
        s += &format!("{hours} hours ");
    }
    let minutes = (length.as_secs() / SECONDS_PER_MINUTE) % MINUTES_PER_HOUR;
    if minutes > 0 {
        s += &format!("{minutes} minutes ");
    }
    let seconds = length.as_secs() % SECONDS_PER_MINUTE;
    if seconds > 0 {
        s += &format!("{seconds} seconds ");
    }
    s += &format!("{} milliseconds", length.subsec_millis());

    s
}

#[component]
pub fn TimedProgressBar(
    #[prop(default = Instant::now())] start_time: Instant,
    #[prop(default = Duration::from_millis(100))] interval: Duration,
    length: Option<Duration>,
) -> impl IntoView {
    let percentage_done = move || {
        length.map(|length| {
            100. * ((start_time.elapsed().as_micros()) as f64 / (length.as_micros()) as f64)
        })
    };

    let (progress, set_progress) = create_signal(Some(0.));
    let progress_remainder = create_memo(move |_| progress.get().map(|x| x % 100.));
    let loops = move || (progress.get().unwrap_or_default() as u32) / 100;

    set_interval(
        move || {
            set_progress.set(percentage_done());
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

#[derive(Debug, Clone)]
enum Length {
    Determinate(Duration),
    Indeterminate(String),
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Length::Determinate(length) => format_time(*length),
            Length::Indeterminate(message) => message.to_owned(),
        };
        write!(f, "{formatted}")
    }
}
struct TimedProgressParams {
    title: String,
    length: Length,
}

impl TimedProgressParams {
    fn build(title: String, length_or_message: String) -> TimedProgressParams {
        let length = if let Ok(length) = length_or_message.parse() {
            Length::Determinate(Duration::from_millis(length))
        } else {
            let message = length_or_message;
            Length::Indeterminate(message)
        };
        Self { title, length }
    }
}

#[component]
pub fn TimedProgressContainer() -> impl IntoView {
    view! {
        <Box
            class="timed-container"
            style="display: flex; flex-direction: column; align-items: center;"
        >
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
                                .into_iter()
                                .map(|(title, length)| TimedProgressParams::build(title, length))
                        }

                        key=|x| x.title.clone()
                        children=move |TimedProgressParams { title, length }| {
                            let time = if let Length::Determinate(len) = length {
                                Some(len)
                            } else {
                                None
                            };
                            view! {
                                <TableRow>
                                    <TableCell>{title}</TableCell>
                                    <TableCell>{format!("{}", length)}</TableCell>
                                    <TimedProgressBar length=time/>
                                </TableRow>
                            }
                        }
                    />

                </TableBody>
            </Table>
        </Box>
    }
}
