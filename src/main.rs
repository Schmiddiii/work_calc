extern crate chrono;

use druid::im::Vector;
use druid::{AppLauncher, PlatformError, WindowDesc};

use chrono::prelude::*;

use std::sync::Arc;
use std::env;
use std::path::Path;

mod save_open;
mod smallwidgets;
mod states;
mod strings;
mod theme;
mod translate;
mod widgets;
mod write_to_pdf;

fn main() -> Result<(), PlatformError> {
    let args: Vec<String> = env::args().collect();

    let work_data;
    if args.len() > 1 {
        work_data = data_with_args(&args[1]);
    } else {
        work_data = data_without_args();
    }

    let main_window = WindowDesc::new(widgets::ui_builder);


    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .delegate(save_open::Delegate)
        .launch(work_data)
}

fn data_without_args() -> states::WorkData {
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

    return work_data;

}

fn data_with_args(string: &str) -> states::WorkData {
    let path = Path::new(string);
    let result_data = save_open::open_file(path);
    return result_data.unwrap_or(data_without_args());
}
