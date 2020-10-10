use druid::im::Vector;
use druid::{Data, Lens};

use serde::{Serialize, Deserialize};

use std::sync::Arc;

use chrono::prelude::*;

pub type Name = (String, String);

#[derive(Clone, Debug, Data, Lens, PartialEq, Serialize, Deserialize)]
pub struct WorkerStateMonth {
    pub name: (String, String), // (First name, Last name)
    pub has_to_work: Option<f32>,
    pub worked: Option<f32>,
    pub paid_out: Option<f32>,
}

#[derive(Clone, Debug, Data, Lens, PartialEq, Serialize, Deserialize)]
pub struct WorkedMonth {
    pub month: Arc<NaiveDate>,
    pub workers: Vector<WorkerStateMonth>,
    pub new_worker_name: Name,
}

#[derive(Clone, Debug, Data, Lens, PartialEq, Serialize, Deserialize)]
pub struct WorkData {
    pub months: Vector<WorkedMonth>,
    pub index: usize,
}

impl WorkerStateMonth {
    pub fn get_delta(&self) -> Option<f32> {
        if self.has_to_work.is_none() || self.worked.is_none() || self.paid_out.is_none() {
            return None;
        }
        Some(self.worked.unwrap() - self.has_to_work.unwrap() - self.paid_out.unwrap())
    }

    pub fn new(name: Name) -> WorkerStateMonth {
        WorkerStateMonth {
            name,
            has_to_work: Some(0.0),
            worked: Some(0.0),
            paid_out: Some(0.0),
        }
    }
}

impl WorkedMonth {
    pub fn create_next_month(&self) -> WorkedMonth {
        WorkedMonth {
            month: Arc::new(NaiveDate::from_ymd(
                self.month.year() + if self.month.month() == 12 { 1 } else { 0 },
                self.month.month() % 12 + 1,
                1,
            )),
            workers: self
                .workers
                .iter()
                .map(|w| WorkerStateMonth::new(w.name.clone()))
                .collect(),
            new_worker_name: ("".to_string(), "".to_string()),
        }
    }

    pub fn get_from_name(&self, name: Name) -> Option<WorkerStateMonth> {
        self.workers.clone().into_iter().find(|v| v.name == name)
    }
}

impl WorkData {
    pub fn previous_month(&mut self) {
        if self.index > 0 {
            self.index = self.index - 1;
        }
    }

    pub fn next_month(&mut self) {
        self.index = self.index + 1;
        if self.index >= self.months.len() {
            self.index = self.months.len();
            if self.months.len() != 0 {
                self.months
                    .push_back(self.months[self.index - 1].create_next_month())
            }
        }
    }

    pub fn get_overall_from_name_previous(&self, name: Name) -> Option<f32> {
        let mut result = None;

        let mut previous_months = self.months.clone();
        previous_months.truncate(self.index);

        for wsm in previous_months {
            let worker = wsm.workers.iter().find(|w| w.name == name);
            if worker.is_some() {
                result = Some(result.unwrap_or(0.0) + worker.unwrap().get_delta().unwrap_or(0.0));
            }
        }

        return result;
    }

    pub fn get_overall_with_state_all_previous(&self) -> Vector<(WorkerStateMonth, Option<f32>)> {
        let current_workers = self.months[self.index].workers.clone();
        current_workers
            .into_iter()
            .map(|wsm| (wsm.clone(), self.get_overall_from_name_previous(wsm.name)))
            .collect()
    }
}
