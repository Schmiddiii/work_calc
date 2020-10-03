use druid::im::Vector;
use druid::lens::{self, LensExt};
use druid::widget::{Button, Container, Flex, Label, List, Painter, Parse, Scroll, TextBox};
use druid::{Env, RenderContext, Widget, WidgetExt};

use crate::states::{WorkerStateMonth, WorkData, Name};
use crate::theme;

pub fn ui_builder() -> impl Widget<WorkData> {
    let month_label =
        Label::new(|data: &WorkData, _env: &Env| data.current_date.format("%B %Y").to_string());

    let list = Scroll::new(List::new(|| {
        Flex::column()
            .with_child(Flex::row().with_child(side_buttons()).with_child(
                ui_worker_state_month().lens(lens::Id.map(
                    |d: &(Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth))| d.1.clone(),
                    |d: &mut (Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth)), v: (Name, WorkerStateMonth)| {
                        d.0.set(d.0.index_of(&d.1).unwrap(), v.clone());
                        d.1 = v;
                    },
                )),
            ))
            .with_spacer(theme::SPACER_SIZE)
    }))
    .lens(lens::Id.map(
        |d: &WorkData| {
            let data = d.get_from_month(*d.current_date);
            (data.clone(), data.clone())

        },
        |d: &mut WorkData, x: (Vector<(Name, WorkerStateMonth)>, Vector<(Name, WorkerStateMonth)>)| {
            x.0.iter().for_each(|v| d.insert(*d.current_date, v.0.clone(), v.1.clone()));
        },
    ));

    let layout = Flex::column().with_child(month_label).with_child(list);

    return layout;
}

fn side_buttons() -> impl Widget<(Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth))> {
    Flex::column()
/*        .with_child(Button::new("▲").on_click(
            |_, (shared, item): &mut (Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth)), _| {
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != 0 {
                    shared.swap(index.unwrap(), index.unwrap()-1);
                }
                println!("Moving up {:?}", item)
            },
        ))
*/        .with_child(Button::new("-").on_click(
            |_, (shared, item): &mut (Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth)), _| {
                shared.retain(|v| v != item);
                println!("Deleting {:?}", item)
            },
        ))
/*        .with_child(Button::new("▼").on_click(
            |_, (shared, item): &mut (Vector<(Name, WorkerStateMonth)>, (Name, WorkerStateMonth)), _| {
                // shared.retain(|v| v != item);
                let index = shared.index_of(item);
                if index.is_some() && index.unwrap() != shared.len()-1 {
                    shared.swap(index.unwrap(), index.unwrap()+1);
                }
                println!("Moving down {:?}", item)
            },*/
        //))
}

fn ui_worker_state_month() -> impl Widget<(Name, WorkerStateMonth)> {
    let name_label = Label::new(|data: &Name, _env: &Env| {
        format!("{} {}", data.0, data.1)
    })
    .with_text_size(20.0)
    .padding(5.0)
    .center()
    .lens(lens::Id.map(
        |d: &(Name, WorkerStateMonth)| d.0.clone(),
        |d: &mut (Name, WorkerStateMonth), v: Name| d.0 = v
    ));

    let painter = Painter::new(|ctx, data: &(Name, WorkerStateMonth), _env| {
        let bounds = ctx.size().to_rect().inset(-theme::STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(theme::CORNER_RADIUS);
        if data.1.done {
            ctx.stroke(rounded, &theme::COLOR_DONE, theme::STROKE_WIDTH);
        } else {
            ctx.stroke(rounded, &theme::COLOR_NOT_DONE, theme::STROKE_WIDTH);
        }
    })/*.lens(lens::Id.map(
        |d: &(Name, WorkerStateMonth)| d.1,
        |d: &mut (Name, WorkerStateMonth), v: WorkerStateMonth| d.1 = v

    ))*/;

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
        .with_spacer(theme::SPACER_SIZE)
        .lens(lens::Id.map(
            |d: &(Name, WorkerStateMonth)| d.1.clone(),
            |d: &mut (Name, WorkerStateMonth), v: WorkerStateMonth| d.1 = v
        ));

    let all = Flex::column()
        .with_child(name_label)
        .with_child(inputs)
        .with_spacer(theme::SPACER_SIZE);

    let container = Container::new(all).background(painter);

    return container;
}
