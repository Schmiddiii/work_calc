use druid::im::Vector;
use druid::lens::{self, LensExt};
use druid::widget::{Align, Button, Container, Flex, Label, List, Scroll};
use druid::{Env, Widget, WidgetExt};

use crate::save_open;
use crate::smallwidgets;
use crate::states::{WorkData, WorkedMonth, WorkerStateMonth};
use crate::strings::{
    STR_DELTA, STR_HAS_TO_WORK, STR_LAST_MONTH, STR_MONTH_FORMAT, STR_OVERALL, STR_PAID_OUT,
    STR_WORKED,
};
use crate::translate::translate;
use crate::theme;

pub fn ui_builder() -> impl Widget<WorkData> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    Align::left(
                        Button::new("◀")
                            .on_click(|_, data: &mut WorkData, _| data.previous_month()),
                    ),
                    1.0,
                )
                .with_child(save_open::build_save_widget().padding(2.5))
                .with_child(save_open::build_open_widget().padding(2.5))
                .with_child(save_open::build_save_to_pdf_widget().padding(2.5))
                .with_flex_child(
                    Align::right(
                        Button::new("▶").on_click(|_, data: &mut WorkData, _| data.next_month()),
                    ),
                    1.0,
                ).padding(5.0),
        )
        .with_flex_child(
            Scroll::new(Flex::row().with_child(ui_month_overview())).vertical(),
            1.0,
        )
}

pub fn ui_month_overview() -> impl Widget<WorkData> {
    let month_label = Label::new(|data: &WorkData, _env: &Env| {
        translate(data.months[data.index]
            .month
            .format(STR_MONTH_FORMAT)
            .to_string())
    }).with_text_size(20.0).padding(10.0);

    let list = List::new(|| {
        Flex::column()
            .with_child(Flex::row().with_child(side_buttons()).with_child(
                ui_worker_state_month().lens(lens::Id.map(
                    |d: &(
                        Vector<(WorkerStateMonth, Option<f64>)>,
                        (WorkerStateMonth, Option<f64>),
                    )| d.1.clone(),
                    |d: &mut (
                        Vector<(WorkerStateMonth, Option<f64>)>,
                        (WorkerStateMonth, Option<f64>),
                    ),
                     v: (WorkerStateMonth, Option<f64>)| {
                        d.0.set(d.0.index_of(&d.1).unwrap(), v.clone());
                        d.1 = v;
                    },
                )),
            ))
            .with_spacer(theme::SPACER_SIZE)
    });

    let list_lens = list.lens(lens::Id.map(
        |d: &WorkData| {
            let overall_with_state_previous = d.get_overall_with_state_all_previous();
            (
                overall_with_state_previous.clone(),
                overall_with_state_previous,
            )
        },
        |d: &mut WorkData,
         x: (
            Vector<(WorkerStateMonth, Option<f64>)>,
            Vector<(WorkerStateMonth, Option<f64>)>,
        )| { d.months[d.index].workers = x.0.iter().map(|v| v.0.clone()).collect() },
    ));

    let layout = Flex::column()
        .with_child(month_label)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(list_lens)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(smallwidgets::build_new_worker_widget().lens(lens::Id.map(
            |d: &WorkData| d.months[d.index].clone(),
            |d: &mut WorkData, v: WorkedMonth| d.months[d.index] = v,
        )));

    return layout;
}

fn side_buttons() -> impl Widget<(
    Vector<(WorkerStateMonth, Option<f64>)>,
    (WorkerStateMonth, Option<f64>),
)> {
    Flex::column()
        .with_child(Button::new("▲").on_click(
            |_,
             (shared, item): &mut (
                Vector<(WorkerStateMonth, Option<f64>)>,
                (WorkerStateMonth, Option<f64>),
            ),
             _| {
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != 0 {
                    shared.swap(index.unwrap(), index.unwrap() - 1);
                }
            },
        ))
        .with_child(Button::new("-").on_click(
            |_,
             (shared, item): &mut (
                Vector<(WorkerStateMonth, Option<f64>)>,
                (WorkerStateMonth, Option<f64>),
            ),
             _| {
                shared.retain(|v| v != item);
            },
        ))
        .with_child(Button::new("▼").on_click(
            |_,
             (shared, item): &mut (
                Vector<(WorkerStateMonth, Option<f64>)>,
                (WorkerStateMonth, Option<f64>),
            ),
             _| {
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != shared.len() - 1 {
                    shared.swap(index.unwrap(), index.unwrap() + 1);
                }
            },
        ))
}

fn ui_worker_state_month() -> impl Widget<(WorkerStateMonth, Option<f64>)> {
    let name_label = smallwidgets::build_name_label();

    let painter = smallwidgets::build_painter();

    let has_to_work_flex = smallwidgets::build_label_with_input(
        Label::new(STR_HAS_TO_WORK),
        WorkerStateMonth::has_to_work,
    );
    let worked_flex =
        smallwidgets::build_label_with_input(Label::new(STR_WORKED), WorkerStateMonth::worked);
    let paid_out_flex =
        smallwidgets::build_label_with_input(Label::new(STR_PAID_OUT), WorkerStateMonth::paid_out);

    let inputs = smallwidgets::build_flex_column(vec![
        Box::new(has_to_work_flex),
        Box::new(worked_flex),
        Box::new(paid_out_flex),
    ])
    .lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f64>)| d.0.clone(),
        |d: &mut (WorkerStateMonth, Option<f64>), v: WorkerStateMonth| d.0 = v,
    ));

    let delta_output = Label::new(|data: &WorkerStateMonth, _env: &Env| {
        let delta = data.get_delta();
        if delta.is_none() {
            "".to_string()
        } else {
            format!("{}", delta.unwrap())
        }
    })
    .lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f64>)| d.0.clone(),
        |d: &mut (WorkerStateMonth, Option<f64>), v: WorkerStateMonth| d.0 = v,
    ));

    let delta_flex = smallwidgets::build_widget_with_label_row(STR_DELTA, delta_output);

    let last_month_output = Label::new(|data: &Option<f64>, _env: &Env| {
        if data.is_none() {
            "".to_string()
        } else {
            format!("{}", data.unwrap())
        }
    })
    .lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f64>)| d.1.clone(),
        |d: &mut (WorkerStateMonth, Option<f64>), v: Option<f64>| d.1 = v,
    ));

    let last_month_flex =
        smallwidgets::build_widget_with_label_row(STR_LAST_MONTH, last_month_output);

    let overall_output = Label::new(|data: &(WorkerStateMonth, Option<f64>), _env: &Env| {
        let delta = data.0.get_delta();
        if delta.is_none() {
            return "".to_string();
        }
        format!(
            "{}",
            (((delta.unwrap() + data.1.unwrap_or(0.0)) * 100.0).round() / 100.0)
        )
    });

    let overall_flex = smallwidgets::build_widget_with_label_row(STR_OVERALL, overall_output);

    let worker_stats = smallwidgets::build_flex_column(vec![
        Box::new(inputs),
        Box::new(delta_flex),
        Box::new(last_month_flex),
        Box::new(overall_flex),
    ]);

    let all = Flex::column()
        .with_child(name_label)
        .with_child(worker_stats)
        .with_spacer(theme::SPACER_SIZE);

    let container = Container::new(all).background(painter);

    return container;
}
