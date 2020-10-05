use druid::widget::{Flex, Label, Painter, Parse, TextBox};
use druid::{Env, RenderContext, Widget, WidgetExt};
use druid::lens::{self, LensExt};
use druid::{Lens, Data};

use crate::states::{WorkerStateMonth};
use crate::theme;

pub fn build_name_label() -> impl Widget<(WorkerStateMonth, Option<f32>)> {
    Label::new(|data: &WorkerStateMonth, _env: &Env| {
        format!("{} {}", data.name.0, data.name.1)
    })
    .with_text_size(20.0)
    .padding(5.0)
    .center()
    .lens(lens::Id.map(
        |d: &(WorkerStateMonth, Option<f32>)| d.0.clone(),
        |d: &mut (WorkerStateMonth, Option<f32>), v: WorkerStateMonth| d.0 = v
    ))
}

pub fn build_painter() -> Painter<(WorkerStateMonth, Option<f32>)> {
    Painter::new(|ctx, data: &(WorkerStateMonth, Option<f32>), _env| {
        let bounds = ctx.size().to_rect().inset(-theme::STROKE_WIDTH / 2.0);
        let rounded = bounds.to_rounded_rect(theme::CORNER_RADIUS);
        if data.0.has_to_work.is_some() && data.0.worked.is_some() && data.0.paid_out.is_some() {
            ctx.stroke(rounded, &theme::COLOR_DONE, theme::STROKE_WIDTH);
        } else {
            ctx.stroke(rounded, &theme::COLOR_NOT_DONE, theme::STROKE_WIDTH);
        }
    })
}

pub fn build_label_with_input<L: Lens<WorkerStateMonth, Option<f32>> + 'static>(label: Label<WorkerStateMonth>, lens: L) -> impl Widget<WorkerStateMonth> {
    let input = Parse::new(TextBox::new()).lens(lens);

    Flex::column()
        .with_child(label)
        .with_spacer(theme::SPACER_SIZE)
        .with_child(input)
}

pub fn build_flex_column<T: Data>(widgets: Vec<Box<dyn Widget<T>>>) -> impl Widget<T> {
    let mut result_widget = Flex::row().with_spacer(theme::SPACER_SIZE);

    for widget in widgets {
        result_widget = result_widget.with_child(widget).with_spacer(theme::SPACER_SIZE);
    }

    return result_widget;
}

pub fn build_widget_with_label_row<T: Data>(label1: &str, label2: impl Widget<T> + 'static) -> impl Widget<T> {
    return Flex::column().with_child(Label::new(label1)).with_spacer(theme::SPACER_SIZE).with_child(label2).with_spacer(theme::SPACER_SIZE);

}