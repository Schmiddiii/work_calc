use druid::lens::{self, LensExt};
use druid::widget::{Button, Flex, Label, Painter, Parse, TextBox};
use druid::{Data, Lens};
use druid::{Env, RenderContext, Widget, WidgetExt};

use crate::states::{WorkedMonth, WorkerStateMonth};
use crate::strings::*;
use crate::theme;

pub fn build_name_label() -> impl Widget<(WorkerStateMonth, Option<f64>)> {
    Label::new(|data: &WorkerStateMonth, _env: &Env| format!("{} {}", data.name.0, data.name.1))
        .with_text_size(20.0)
        .padding(5.0)
        .center()
        .lens(lens::Id.map(
            |d: &(WorkerStateMonth, Option<f64>)| d.0.clone(),
            |d: &mut (WorkerStateMonth, Option<f64>), v: WorkerStateMonth| d.0 = v,
        ))
}

pub fn build_painter() -> Painter<(WorkerStateMonth, Option<f64>)> {
    Painter::new(|ctx, data: &(WorkerStateMonth, Option<f64>), _env| {
        let bounds = ctx.size().to_rect().inset(-theme::STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(theme::CORNER_RADIUS);
        if data.0.has_to_work.is_some()
            && data.0.worked.is_some()
            && data.0.paid_out.is_some()
            && data.0.has_to_work.unwrap() != 0.0
            && data.0.worked.unwrap() != 0.0
        {
            ctx.stroke(rounded, &theme::COLOR_DONE, theme::STROKE_WIDTH);
        } else {
            ctx.stroke(rounded, &theme::COLOR_NOT_DONE, theme::STROKE_WIDTH);
        }
    })
}

pub fn build_label_with_input<L: Lens<WorkerStateMonth, Option<f64>> + 'static>(
    label: Label<WorkerStateMonth>,
    lens: L,
) -> impl Widget<WorkerStateMonth> {
    let input = Parse::new(TextBox::new()).lens(lens);

    Flex::column()
        .with_child(label)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(input)
}

pub fn build_flex_column<T: Data>(widgets: Vec<Box<dyn Widget<T>>>) -> impl Widget<T> {
    let mut result_widget = Flex::row().with_spacer(theme::SPACER_SIZE);

    for widget in widgets {
        result_widget = result_widget
            .with_child(widget)
            .with_spacer(theme::SPACER_SIZE);
    }

    return result_widget;
}

pub fn build_widget_with_label_row<T: Data>(
    label1: &str,
    label2: impl Widget<T> + 'static,
) -> impl Widget<T> {
    return Flex::column()
        .with_child(Label::new(label1))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(label2)
        .with_spacer(theme::SPACER_SIZE);
}

pub fn build_new_worker_widget() -> impl Widget<WorkedMonth> {
    Flex::row()
        .with_child(build_widget_with_label_row(
            STR_FIRST_NAME,
            TextBox::new().lens(lens::Id.map(
                |d: &WorkedMonth| d.new_worker_name.0.clone(),
                |d: &mut WorkedMonth, v: String| d.new_worker_name.0 = v.clone(),
            )),
        ))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(build_widget_with_label_row(
            STR_LAST_NAME,
            TextBox::new().lens(lens::Id.map(
                |d: &WorkedMonth| d.new_worker_name.1.clone(),
                |d: &mut WorkedMonth, v: String| d.new_worker_name.1 = v.clone(),
            )),
        ))
        .with_spacer(theme::SPACER_SIZE)
        .with_child(
            Button::new(STR_SUBMIT).on_click(|_, data: &mut WorkedMonth, _| {
                if data.get_from_name(data.new_worker_name.clone()).is_none() {
                    data.workers
                        .push_back(WorkerStateMonth::new(data.new_worker_name.clone()));
                    data.new_worker_name = ("".to_string(), "".to_string());
                }
            }),
        )
        .with_spacer(theme::SPACER_SIZE)
}
