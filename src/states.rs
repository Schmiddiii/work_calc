use druid::im::Vector;
use druid::{Data, Lens};

use std::sync::Arc;

use chrono::prelude::*;

pub type Name = (String, String);

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkerStateMonth {
    pub name: (String, String), // (First name, Last name)
    pub has_to_work: Option<f32>,
    pub worked: Option<f32>,
    pub paid_out: Option<f32>,
}

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkedMonth {
    pub month: Arc<Date<Local>>,
    pub workers: Vector<WorkerStateMonth>,
    pub last_month: Option<Box<WorkedMonth>>
}

impl Data for Box<WorkedMonth> {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}




impl WorkerStateMonth {
    pub fn get_delta(&self) -> Option<f32> {
        if self.has_to_work.is_none() || self.worked.is_none() || self.paid_out.is_none() {
            return None;
        }
        Some(self.worked.unwrap() - self.has_to_work.unwrap() - self.paid_out.unwrap())
    }
}

impl WorkedMonth {
    pub fn get_from_name(&self, name: Name) -> Option<WorkerStateMonth> {
        self.workers.clone().into_iter().find(|v| v.name == name)
    }

    pub fn get_overall(&self, name: Name) -> Option<f32> {
        let worker = self.get_from_name(name.clone());

        if worker.clone().is_none() || worker.clone().unwrap().get_delta().is_none() {
            return None;
        }
        let last_month =
        if self.last_month.is_none() || self.last_month.as_ref().unwrap().get_overall(name.clone()).is_none() {
            0.0
        } else {self.last_month.as_ref().unwrap().get_overall(name.clone()).unwrap()};


        Some(worker.unwrap().get_delta().unwrap() + last_month)
    }

    pub fn get_overall_previous(&self, name: Name) -> Option<f32> {
        if self.last_month.is_none() {
            return None;
        }
        self.last_month.as_ref().unwrap().get_overall(name)
    }

    pub fn get_overall_with_state(&self, name: Name) -> Option<(WorkerStateMonth, Option<f32>)> {
        let wsm = self.get_from_name(name.clone());
        if wsm.is_none() {
            return None;
        }

        return Some((wsm.unwrap(), self.get_overall(name)));
    }


    pub fn get_overall_with_state_previous(&self, name: Name) -> Option<(WorkerStateMonth, Option<f32>)> {
        let wsm = self.get_from_name(name.clone());
        if wsm.is_none() {
            return None;
        }

        return Some((wsm.unwrap(), self.get_overall_previous(name)));
    }

    pub fn get_overall_with_state_all(&self) -> Vector<(WorkerStateMonth, Option<f32>)> {
        let names_iter = self.workers.iter().map(|w| w.name.clone());

        names_iter.map(|n| self.get_overall_with_state(n).unwrap()).collect()

    }

    pub fn get_overall_with_state_all_previous(&self) -> Vector<(WorkerStateMonth, Option<f32>)> {
        let names_iter = self.workers.iter().map(|w| w.name.clone());

        names_iter.map(|n| self.get_overall_with_state_previous(n).unwrap()).collect()

    }
}
