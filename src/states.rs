use druid::im::Vector;
use druid::{Data, Lens};

use std::sync::Arc;

use chrono::prelude::*;

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkerStateMonth {
    pub name: (String, String), // (First name, Last name)
    pub done: bool,
    pub has_to_work: Option<f32>,
    pub worked: Option<f32>,
    pub paid_out: Option<f32>,
}

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkedMonth {
    pub month: Arc<Date<Local>>,
    pub workers: Vector<WorkerStateMonth>,
}
