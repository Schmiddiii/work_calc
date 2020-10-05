
use druid::im::Vector;
use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;

use std::sync::Arc;

mod states;
mod theme;
mod widgets;
mod smallwidgets;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(widgets::ui_builder);
    let worker1 = states::WorkerStateMonth {
        name: ("Hallo".to_string(), "Welt".to_string()),
        has_to_work: Some(100.0),
        worked: Some(133.0),
        paid_out: Some(0.0),
    };

    let worker2 = states::WorkerStateMonth {
        name: ("Bye".to_string(), "Welt".to_string()),
        has_to_work: Some(130.0),
        worked: Some(122.0),
        paid_out: Some(0.0),
    };

    let worker3 = states::WorkerStateMonth {
        name: ("Hallo".to_string(), "Welt".to_string()),
        has_to_work: Some(330.0),
        worked: Some(120.0),
        paid_out: Some(0.0),
    };

    let mut data_pre = states::WorkedMonth {
        month: Arc::new(Local.ymd(2020,9,1)),
        workers: Vector::new(),
        last_month: None
    };

    data_pre.workers.push_back(worker3);

    let mut data = states::WorkedMonth {
        month: Arc::new(Local.ymd(2020, 10, 1)),
        workers: Vector::new(),
        last_month: Some(Box::new(data_pre))
    };

    data.workers.push_front(worker1);
    data.workers.push_back(worker2);


    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}
