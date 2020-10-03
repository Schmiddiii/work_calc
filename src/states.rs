use druid::im::{Vector, HashMap, HashSet, OrdMap};
use druid::{Data, Lens};

use std::sync::Arc;

use chrono::prelude::*;

pub type Name = (String, String);    // (First name, Last name)

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkerStateMonth {
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

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct WorkData {
    pub current_date: Arc<Date<Local>>,
    pub known_names: HashSet<Name>,
    map: OrdMap<(Arc<Date<Local>>, Name), WorkerStateMonth>
}

impl WorkData {
    pub fn new(current_date: Date<Local>) -> WorkData {
        WorkData {
            current_date: Arc::new(current_date),
            known_names: HashSet::new(),
            map: OrdMap::new()
        }
    }

    pub fn insert(&mut self, date: Date<Local>, name: Name, value: WorkerStateMonth) {
        self.known_names.insert(name.clone());
        self.map.insert((Arc::new(date), name), value);
    }

    pub fn get_from_month(&self, date: Date<Local>) -> Vector<(Name, WorkerStateMonth)> {
        let with_none = self.known_names.clone().into_iter().map(|n| (n.clone(), self.map.get(&(Arc::new(date), n))));

        with_none.filter(|v| (v).1.is_some()).map(|v| (v.0, v.1.unwrap().clone())).collect()
    }

    pub fn get_mut(&mut self, date: Date<Local>, name: Name) -> Option<&mut WorkerStateMonth> {
        self.map.get_mut(&(Arc::new(date), name))

    }

}



