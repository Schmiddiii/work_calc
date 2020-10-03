use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;


mod states;
mod theme;
mod widgets;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(widgets::ui_builder);
    let worker1 = states::WorkerStateMonth {
        done: true,
        has_to_work: Some(100.0),
        worked: Some(133.0),
        paid_out: Some(0.0),
    };

    let worker2 = states::WorkerStateMonth {
        done: false,
        has_to_work: Some(130.0),
        worked: Some(122.0),
        paid_out: Some(0.0),
    };

    let worker3 = states::WorkerStateMonth {
        done: false,
        has_to_work: Some(330.0),
        worked: None,
        paid_out: None,
    };

    let mut data = states::WorkData::new(Local.ymd(2020, 10, 1));
    data.insert(Local.ymd(2020, 10, 1), ("Hallo".to_string(), "Welt".to_string()), worker1);
    data.insert(Local.ymd(2020, 10, 1), ("Bye".to_string(), "Welt".to_string()), worker2);
    data.insert(Local.ymd(2020, 10, 1), ("Asdf".to_string(), "sf".to_string()), worker3);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}
