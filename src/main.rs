extern crate chrono;


use druid::im::Vector;
use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;

use std::sync::Arc;

mod save_open;
mod smallwidgets;
mod states;
mod theme;
mod widgets;
mod write_to_pdf;

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
        month: Arc::new(NaiveDate::from_ymd(2020, 9, 1)),
        workers: Vector::new(),
        new_worker_name: ("".to_string(), "".to_string()),
    };

    data_pre.workers.push_back(worker3);

    let mut data = states::WorkedMonth {
        month: Arc::new(NaiveDate::from_ymd(2020, 10, 1)),
        workers: Vector::new(),
        new_worker_name: ("".to_string(), "".to_string()),
    };

    data.workers.push_front(worker1);
    data.workers.push_back(worker2);

    let mut work_data = states::WorkData {
        index: 1,
        months: Vector::new(),
    };

    work_data.months.push_back(data_pre);
    work_data.months.push_back(data);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .delegate(save_open::Delegate)
        .launch(work_data)
}
