use crate::states::WorkData;
use crate::write_to_pdf::write_to_pdf;

use druid::{commands, AppDelegate, Command, DelegateCtx, Env, Target, Widget, FileSpec, FileDialogOptions};
use druid::widget::Button;

use ron;
use crate::strings::{STR_SAVE_PDF, STR_OPEN_RON, STR_SAVE_RON};

const RON_FILETYPE: FileSpec = FileSpec::new("Rust Object Notation", &["ron"]);
const PDF_FILETYPE: FileSpec = FileSpec::new("Printable Document Format", &["pdf"]);

pub struct Delegate;

impl AppDelegate<WorkData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut WorkData,
        _env: &Env,
    ) -> bool {
        if let Some(Some(file_info)) = cmd.get(commands::SAVE_FILE) {
            let file_extension = file_info.path().extension();
            if file_extension.is_none() {
                println!("Cannot identify type of file: {:?}", file_info.path());
                return false;

            }
            match file_extension.unwrap().to_str().unwrap() {
                "ron" => {
                    let serialized = ron::to_string(data).unwrap();
                    if let Err(e) = std::fs::write(file_info.path(), &serialized) {
                        println!("Error writing file: {}", e);
                    }
                },
                "pdf" => {
                    write_to_pdf(&data, file_info.path());
                },
                _ => {
                }

            }

            return false;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    let deserialized = ron::from_str(&s);
                    if deserialized.is_ok() {
                        *data = deserialized.unwrap();
                    } else {
                        println!("Error interpreting file: {:?}", file_info.path());
                    }
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return false;
        }
        true
    }
}

pub fn build_save_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![RON_FILETYPE];
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(RON_FILETYPE);

    let save_widget = Button::new(STR_SAVE_RON).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()), Target::Window(ctx.window_id()));
    });

    return save_widget;
}

pub fn build_open_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![RON_FILETYPE];
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(RON_FILETYPE);

    let open_widget = Button::new(STR_OPEN_RON).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()), Target::Window(ctx.window_id()));
    });

    return open_widget;
}

pub fn build_save_to_pdf_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![PDF_FILETYPE];
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(PDF_FILETYPE);

    let save_widget = Button::new(STR_SAVE_PDF).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()), Target::Window(ctx.window_id()));
    });

    return save_widget;

}