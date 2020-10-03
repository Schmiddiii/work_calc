use druid::im::Vector;
use druid::lens::{self, LensExt};
use druid::widget::{Button, Container, Flex, Label, List, Painter, Parse, Scroll, TextBox};
use druid::{Env, RenderContext, Widget, WidgetExt};

use crate::states::{WorkedMonth, WorkerStateMonth};
use crate::theme;

pub fn ui_builder() -> impl Widget<WorkedMonth> {
    let month_label =
        Label::new(|data: &WorkedMonth, _env: &Env| data.month.format("%B %Y").to_string());

    let list = Scroll::new(List::new(|| {
        Flex::column()
            .with_child(Flex::row().with_child(side_buttons()).with_child(
                ui_worker_state_month().lens(lens::Id.map(
                    |d: &(Vector<WorkerStateMonth>, WorkerStateMonth)| d.1.clone(),
                    |d: &mut (Vector<WorkerStateMonth>, WorkerStateMonth), v: WorkerStateMonth| {
                        d.0.set(d.0.index_of(&d.1).unwrap(), v.clone());
                        d.1 = v;
                    },
                )),
            ))
            .with_spacer(theme::SPACER_SIZE)
    }))
    .lens(lens::Id.map(
        |d: &WorkedMonth| (d.workers.clone(), d.workers.clone()),
        |d: &mut WorkedMonth, x: (Vector<WorkerStateMonth>, Vector<WorkerStateMonth>)| {
            d.workers = x.0
        },
    ));

    let layout = Flex::column().with_child(month_label).with_child(list);

    return layout;
}

fn side_buttons() -> impl Widget<(Vector<WorkerStateMonth>, WorkerStateMonth)> {
    Flex::column()
        .with_child(Button::new("▲").on_click(
            |_, (shared, item): &mut (Vector<WorkerStateMonth>, WorkerStateMonth), _| {
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != 0 {
                    shared.swap(index.unwrap(), index.unwrap()-1);
                }
                println!("Moving up {:?}", item)
            },
        ))
        .with_child(Button::new("-").on_click(
            |_, (shared, item): &mut (Vector<WorkerStateMonth>, WorkerStateMonth), _| {
                shared.retain(|v| v != item);
                println!("Deleting {:?}", item)
            },
        ))
        .with_child(Button::new("▼").on_click(
            |_, (shared, item): &mut (Vector<WorkerStateMonth>, WorkerStateMonth), _| {
                // shared.retain(|v| v != item);
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != shared.len()-1 {
                    shared.swap(index.unwrap(), index.unwrap()+1);
                }
                println!("Moving down {:?}", item)
            },
        ))
}

fn ui_worker_state_month() -> impl Widget<WorkerStateMonth> {
    let name_label = Label::new(|data: &WorkerStateMonth, _env: &Env| {
        format!("{} {}", data.name.0, data.name.1)
    })
    .with_text_size(20.0)
    .padding(5.0)
    .center();

    let painter = Painter::new(|ctx, data: &WorkerStateMonth, _env| {
        let bounds = ctx.size().to_rect().inset(-theme::STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(theme::CORNER_RADIUS);
        if data.done {
            ctx.stroke(rounded, &theme::COLOR_DONE, theme::STROKE_WIDTH);
        } else {
            ctx.stroke(rounded, &theme::COLOR_NOT_DONE, theme::STROKE_WIDTH);
        }
    });

    let has_to_work_input = Parse::new(TextBox::new()).lens(WorkerStateMonth::has_to_work);
    let worked_input = Parse::new(TextBox::new()).lens(WorkerStateMonth::worked);
    let paid_out_input = Parse::new(TextBox::new()).lens(WorkerStateMonth::paid_out);

    let has_to_work_flex = Flex::column()
        .with_child(Label::new("Has to work"))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(has_to_work_input);
    let worked_flex = Flex::column()
        .with_child(Label::new("Worked"))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(worked_input);
    let paid_out_flex = Flex::column()
        .with_child(Label::new("Paid out"))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(paid_out_input);

    let inputs = Flex::row()
        .with_spacer(theme::SPACER_SIZE)
        .with_child(has_to_work_flex)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(worked_flex)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(paid_out_flex)
        .with_spacer(theme::SPACER_SIZE);

    let all = Flex::column()
        .with_child(name_label)
        .with_child(inputs)
        .with_spacer(theme::SPACER_SIZE);

    let container = Container::new(all).background(painter);

    return container;
}
