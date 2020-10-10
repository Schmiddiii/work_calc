extern crate chrono;

use druid::im::Vector;
use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;

use std::sync::Arc;

mod save_open;
mod smallwidgets;
mod states;
mod strings;
mod theme;
mod widgets;
mod write_to_pdf;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(widgets::ui_builder);

    let data = states::WorkedMonth {
        month: Arc::new(Local::now().with_day(1).unwrap().naive_local().date()),
        workers: Vector::new(),
        new_worker_name: ("".to_string(), "".to_string()),
    };

    let mut work_data = states::WorkData {
        index: 0,
        months: Vector::new(),
    };

    work_data.months.push_back(data);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .delegate(save_open::Delegate)
        .launch(work_data)
}
