use druid::im::Vector;
use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;

use std::sync::Arc;

mod states;
mod theme;
mod widgets;
mod two_key_map;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(widgets::ui_builder);
    let worker1 = states::WorkerStateMonth {
        name: ("Hallo".to_string(), "Welt".to_string()),
        done: true,
        has_to_work: Some(100.0),
        worked: Some(133.0),
        paid_out: Some(0.0),
    };

    let worker2 = states::WorkerStateMonth {
        name: ("Bye".to_string(), "Welt".to_string()),
        done: false,
        has_to_work: Some(130.0),
        worked: Some(122.0),
        paid_out: Some(0.0),
    };

    let worker3 = states::WorkerStateMonth {
        name: ("ASD".to_string(), "asdf".to_string()),
        done: false,
        has_to_work: Some(330.0),
        worked: None,
        paid_out: None,
    };

    let mut data = states::WorkedMonth {
        month: Arc::new(Local.ymd(2020, 10, 1)),
        workers: Vector::new(),
    };

    data.workers.push_front(worker1);
    data.workers.push_back(worker2);
    data.workers.push_back(worker3);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}
