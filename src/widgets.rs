use druid::{Env, Widget, WidgetExt};
use druid::im::Vector;
use druid::lens::{self, LensExt};
use druid::widget::{Button, Container, Flex, Label, List, Scroll};

use crate::smallwidgets;
use crate::states::{WorkedMonth, WorkerStateMonth};
use crate::theme;

pub fn ui_builder() -> impl Widget<WorkedMonth> {
    let month_label =
        Label::new(|data: &WorkedMonth, _env: &Env| data.month.format("%B %Y").to_string());

    let list = Scroll::new(List::new(|| {
        Flex::column()
            .with_child(Flex::row().with_child(side_buttons()).with_child(
                ui_worker_state_month().lens(lens::Id.map(
                    |d: &(Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>))| d.1.clone(),
                    |d: &mut (Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>)), v: (WorkerStateMonth, Option<f32>)| {
                        d.0.set(d.0.index_of(&d.1).unwrap(), v.clone());
                        d.1 = v;
                    },
                )),
            ))
            .with_spacer(theme::SPACER_SIZE)
    }))
        .lens(lens::Id.map(
            |d: &WorkedMonth| {
                let overall_with_state_previous = d.get_overall_with_state_all_previous();
                (overall_with_state_previous.clone(), overall_with_state_previous)
            },
            |d: &mut WorkedMonth, x: (Vector<(WorkerStateMonth, Option<f32>)>, Vector<(WorkerStateMonth, Option<f32>)>)| {
                d.workers = x.0.iter().map(|v| v.0.clone()).collect()
            },
        ));

    let layout = Flex::column().with_child(month_label).with_child(list);

    return layout;
}

fn side_buttons() -> impl Widget<(Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>))> {
    Flex::column()
        .with_child(Button::new("▲").on_click(
            |_, (shared, item): &mut (Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>)), _| {
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != 0 {
                    shared.swap(index.unwrap(), index.unwrap() - 1);
                }
            },
        ))
        .with_child(Button::new("-").on_click(
            |_, (shared, item): &mut (Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>)), _| {
                shared.retain(|v| v != item);
            },
        ))
        .with_child(Button::new("▼").on_click(
            |_, (shared, item): &mut (Vector<(WorkerStateMonth, Option<f32>)>, (WorkerStateMonth, Option<f32>)), _| {
                // shared.retain(|v| v != item);
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != shared.len() - 1 {
                    shared.swap(index.unwrap(), index.unwrap() + 1);
                }
            },
        ))
}

fn ui_worker_state_month() -> impl Widget<(WorkerStateMonth, Option<f32>)> {
    let name_label = smallwidgets::build_name_label();

    let painter = smallwidgets::build_painter();

    let has_to_work_flex = smallwidgets::build_label_with_input(Label::new("Has to work"), WorkerStateMonth::has_to_work);
    let worked_flex = smallwidgets::build_label_with_input(Label::new("Worked"), WorkerStateMonth::worked);
    let paid_out_flex = smallwidgets::build_label_with_input(Label::new("Paid out"), WorkerStateMonth::paid_out);

    let inputs = smallwidgets::build_flex_column(vec![Box::new(has_to_work_flex), Box::new(worked_flex), Box::new(paid_out_flex)])
        .lens(lens::Id.map(
            |d: &(WorkerStateMonth, Option<f32>)| d.0.clone(),
            |d: &mut (WorkerStateMonth, Option<f32>), v: WorkerStateMonth| d.0 = v,
        ));

    let delta_output = Label::new(|data: &WorkerStateMonth, _env: &Env| {
        let delta = data.get_delta();
        if delta.is_none() {
            "".to_string()
        } else {
            format!("{}", delta.unwrap())
        }
    }).lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f32>)| d.0.clone(),
        |d: &mut (WorkerStateMonth, Option<f32>), v: WorkerStateMonth| d.0 = v,
    ));

    let delta_flex = smallwidgets::build_widget_with_label_row("Delta", delta_output);

    let last_month_output = Label::new(|data: &Option<f32>, _env: &Env| {
        if data.is_none() {
            "".to_string()
        } else {
            format!("{}", data.unwrap())
        }
    }).lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f32>)| d.1.clone(),
        |d: &mut (WorkerStateMonth, Option<f32>), v: Option<f32>| d.1 = v,
    ));

    let last_month_flex = smallwidgets::build_widget_with_label_row("Last month", last_month_output);

    let overall_output = Label::new(|data: &(WorkerStateMonth, Option<f32>), _env: &Env| {
        let delta = data.0.get_delta();
        if delta.is_none() {
            return "".to_string();
        }
        format!("{}", delta.unwrap() + data.1.unwrap_or(0.0))
    });

    let overall_flex = smallwidgets::build_widget_with_label_row("Overall", overall_output);

    let worker_stats = smallwidgets::build_flex_column(vec![Box::new(inputs), Box::new(delta_flex), Box::new(last_month_flex), Box::new(overall_flex)]);


    let all = Flex::column()
        .with_child(name_label)
        .with_child(worker_stats)
        .with_spacer(theme::SPACER_SIZE);

    let container = Container::new(all).background(painter);

    return container;
}
