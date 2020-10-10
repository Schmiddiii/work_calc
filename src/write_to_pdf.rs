use std::path::Path;
use std::io::BufWriter;

use printpdf::*;

use crate::states::{WorkedMonth, WorkerStateMonth, WorkData};
use std::fs::OpenOptions;
use chrono::NaiveDate;

const PDF_WIDTH: f64 = 210.0;
const PDF_HEIGHT: f64 = 297.0;
const PDF_PADDING_X: f64 = 20.0;
const PDF_PADDING_Y: f64 = 20.0;

const PDF_FONT_SIZE: i64 = 12;
const PDF_FONT_OFFSET_X: f64 = 1.0;
const PDF_FONT_OFFSET_Y: f64 = 1.0;


const CELL_SIZE_Y: f64 = 6.0;

const CELL_SIZE_NAME_X: f64 = 30.0;
const CELL_SIZE_HASTOWORK_X: f64 = 30.0;
const CELL_SIZE_WORKED_X: f64 = 20.0;
const CELL_SIZE_PAIDOUT_X: f64 = 20.0;
const CELL_SIZE_DELTA_X: f64 = 20.0;
const CELL_SIZE_LASTMONTH_X: f64 = 30.0;
const CELL_SIZE_OVERALL_X: f64 = 20.0;



pub fn write_to_pdf(data: &WorkData, path: &Path) {
    let (doc, page1, layer1) = PdfDocument::new("Title", Mm(PDF_WIDTH), Mm(PDF_HEIGHT), "");

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).expect("Cannot load font");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // DO STUFF
    data.months[data.index].workers.iter().enumerate().for_each(|(n, w)| write_single_worker(w, (*data).get_overall_from_name_previous(w.clone().name), *data.months[data.index].month, &current_layer, 2 * n, &font));

    doc.save(&mut BufWriter::new(OpenOptions::new().read(false).write(true).create(true).open(path).unwrap())).unwrap();
}

fn write_single_worker(data: &WorkerStateMonth, last_month: Option<f32>, month: NaiveDate, layer: &PdfLayerReference, number: usize, font: &IndirectFontRef) {
    let name_format = format!("{} {}", data.name.0, data.name.1);
    // layer.use_text(name_format, PDF_FONT_SIZE, Mm(PDF_PADDING_X), Mm(PDF_HEIGHT - PDF_PADDING_Y - 2.0 * (number as f64) * CELL_SIZE_Y), font);
    write_information_line(month, layer, PDF_HEIGHT - PDF_PADDING_Y - (number as f64) * CELL_SIZE_Y, font);
    // write_box(name_format, layer, PDF_PADDING_X, PDF_HEIGHT - PDF_PADDING_Y - (number + 1) as f64 * CELL_SIZE_Y, CELL_SIZE_NAME_X, CELL_SIZE_Y, font);
    write_data_line(data, last_month, layer, PDF_HEIGHT - PDF_PADDING_Y - (1.0+number as f64) * CELL_SIZE_Y, font);
}

fn write_information_line(month: NaiveDate, layer: &PdfLayerReference, y: f64, font: &IndirectFontRef) {
    let month_format = month.format("%B %Y").to_string();
    write_box(month_format, layer, PDF_PADDING_X, y, CELL_SIZE_NAME_X, CELL_SIZE_Y, font);
    write_box("Has to work", layer, PDF_PADDING_X + CELL_SIZE_NAME_X, y, CELL_SIZE_HASTOWORK_X, CELL_SIZE_Y, font);
    write_box("Worked", layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X, y, CELL_SIZE_WORKED_X, CELL_SIZE_Y, font);
    write_box("Paid Out", layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X, y, CELL_SIZE_PAIDOUT_X, CELL_SIZE_Y, font);
    write_box("Delta", layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X, y, CELL_SIZE_DELTA_X, CELL_SIZE_Y, font);
    write_box("Last Month", layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X + CELL_SIZE_DELTA_X, y, CELL_SIZE_LASTMONTH_X, CELL_SIZE_Y, font);
    write_box("Overall", layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X + CELL_SIZE_DELTA_X + CELL_SIZE_LASTMONTH_X, y, CELL_SIZE_OVERALL_X, CELL_SIZE_Y, font);
}

fn write_data_line(data: &WorkerStateMonth, last_month: Option<f32>, layer: &PdfLayerReference, y: f64, font: &IndirectFontRef) {
    let name_format = format!("{} {}", data.name.0, data.name.1);
    write_box(name_format, layer, PDF_PADDING_X, y, CELL_SIZE_NAME_X, CELL_SIZE_Y, font);
    write_box(data.has_to_work.unwrap_or(0.0).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X, y, CELL_SIZE_HASTOWORK_X, CELL_SIZE_Y, font);
    write_box(data.worked.unwrap_or(0.0).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X, y, CELL_SIZE_WORKED_X, CELL_SIZE_Y, font);
    write_box(data.paid_out.unwrap_or(0.0).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X, y, CELL_SIZE_PAIDOUT_X, CELL_SIZE_Y, font);
    write_box(data.get_delta().unwrap_or(0.0).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X, y, CELL_SIZE_DELTA_X, CELL_SIZE_Y, font);
    write_box(last_month.unwrap_or(0.0).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X + CELL_SIZE_DELTA_X, y, CELL_SIZE_LASTMONTH_X, CELL_SIZE_Y, font);
    write_box((last_month.unwrap_or(0.0) + data.get_delta().unwrap_or(0.0)).to_string(), layer, PDF_PADDING_X + CELL_SIZE_NAME_X + CELL_SIZE_HASTOWORK_X + CELL_SIZE_WORKED_X + CELL_SIZE_PAIDOUT_X + CELL_SIZE_DELTA_X + CELL_SIZE_LASTMONTH_X, y, CELL_SIZE_OVERALL_X, CELL_SIZE_Y, font);

}

fn write_box<S: Into<String>>(str: S, layer: &PdfLayerReference, x: f64, y: f64, w: f64, h: f64, font: &IndirectFontRef) {

    let point_nw: Point = Point::new(Mm(x), Mm(y));
    let point_ne: Point = Point::new(Mm(x + w), Mm(y));
    let point_sw: Point = Point::new(Mm(x), Mm(y + h));
    let point_se: Point = Point::new(Mm(x + w), Mm(y + h));

    let line_n: Line = vec![point_nw, point_ne].into_iter().map(|p| (p, true)).collect();
    let line_w: Line = vec![point_nw, point_sw].into_iter().map(|p| (p, true)).collect();
    let line_s: Line = vec![point_sw, point_se].into_iter().map(|p| (p, true)).collect();
    let line_e: Line = vec![point_ne, point_se].into_iter().map(|p| (p, true)).collect();

    let mut lines = vec![line_n, line_w, line_s, line_e];

    lines.iter_mut().for_each(|l| {
        l.set_closed(true);
        l.set_stroke(true);
    });


    layer.use_text(str, PDF_FONT_SIZE, Mm(x + PDF_FONT_OFFSET_X), Mm(y + PDF_FONT_OFFSET_Y), font);
    lines.into_iter().for_each(|l| layer.add_shape(l));
}

