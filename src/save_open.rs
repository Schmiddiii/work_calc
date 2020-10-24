use crate::states::WorkData;
use crate::write_to_pdf::write_to_pdf;
use crate::write_to_pdf::write_template_to_pdf;

use druid::{EventCtx, Selector, widget::Button};
use druid::{
    commands, AppDelegate, Command, DelegateCtx, Env, FileDialogOptions, FileSpec, Target, Widget,
};

use crate::strings::{STR_OPEN_RON, STR_SAVE_PDF, STR_SAVE_RON, STR_SAVE_PDF_TEMPLATE};
use ron;
use std::path::Path;

const RON_FILETYPE: FileSpec = FileSpec::new("Rust Object Notation", &["ron"]);
const PDF_FILETYPE: FileSpec = FileSpec::new("Printable Document Format", &["pdf"]);

const SAVEOPTIONS: Selector<String> = Selector::new("saveoptions");

#[derive(Default)]
pub struct Delegate {
    saveoption: String
}

impl AppDelegate<WorkData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut WorkData,
        _env: &Env,
    ) -> bool {

        if let Some(options) = cmd.get(SAVEOPTIONS) {
            self.saveoption = options.clone();
        }

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
                }
                "pdf" => {
                    if self.saveoption == "pdf_template" {
                        write_template_to_pdf(&data, file_info.path());
                    } else {
                        write_to_pdf(&data, file_info.path());
                    }
                }
                _ => {}
            }

            return false;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            let data_result = open_file(file_info.path());
            if data_result.is_ok() {
                *data = data_result.unwrap();
            }
            return false;
        }
        true
    }
}

pub fn open_file(path: &Path) -> Result<WorkData, &str> {
    match std::fs::read_to_string(path) {
        Ok(s) => {
            let deserialized = ron::from_str(&s);
            match deserialized {
                Ok(wd) => return Ok(wd),
                Err(_) => return Err("Cannot deserialize file")
            }
        }
        Err(_) => Err("Cannot read file"),
    }
}

pub fn build_save_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![RON_FILETYPE];
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(RON_FILETYPE);

    let save_widget = Button::new(STR_SAVE_RON).on_click(move |ctx, _, _| {
        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()),
            Target::Window(ctx.window_id()),
        );
    });

    return save_widget;
}

pub fn build_open_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![RON_FILETYPE];
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(RON_FILETYPE);

    let open_widget = Button::new(STR_OPEN_RON).on_click(move |ctx, _, _| {
        ctx.submit_command(
            druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()),
            Target::Window(ctx.window_id()),
        );
    });

    return open_widget;
}

pub fn build_save_to_pdf_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![PDF_FILETYPE];
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(PDF_FILETYPE);

    let save_widget = Button::new(STR_SAVE_PDF).on_click(move |ctx: &mut EventCtx, _, _| {
        ctx.submit_command(
            Command::new(SAVEOPTIONS, "pdf_save".to_string()), Target::Window(ctx.window_id()));
        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()),
            Target::Window(ctx.window_id()),
        );
    });

    return save_widget;
}
pub fn build_save_template_to_pdf_widget() -> impl Widget<WorkData> {
    let allowed_files = vec![PDF_FILETYPE];
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(allowed_files)
        .default_type(PDF_FILETYPE);

    let save_widget = Button::new(STR_SAVE_PDF_TEMPLATE).on_click(move |ctx: &mut EventCtx, _, _| {
        ctx.submit_command(
            Command::new(SAVEOPTIONS, "pdf_template".to_string()), Target::Window(ctx.window_id()));
        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()),
            Target::Window(ctx.window_id()),
        );
    });

    return save_widget;
}
