#[derive(Debug, Default)]
struct CombinedEvents {
    total_events: u32,
    events: Vec<String>,
}

fn main() {
    let events = ["event 1", "event 2", "event 3", "event 4", "event 5"];

    let res = events.iter().rev().fold(
        CombinedEvents::default(),
        |mut combined_events, next_event| {
            combined_events.total_events += 1;
            combined_events.events.push(next_event.to_string());
            combined_events
        },
    );

    println!("{:?}", res);
}
