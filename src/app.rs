use egui::{ColorImage, Label, Layout, RichText, Style};
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use std::{fs::File, path::PathBuf};
use tinyfiledialogs::open_file_dialog;

#[derive()]
pub struct DinnerApp {
    path: PathBuf,
    table: DataFrame,
}

impl Default for DinnerApp {
    fn default() -> Self {
        Self {
            path: PathBuf::default(),
            table:  DataFrame::default(),
        }
    }
}

impl DinnerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for DinnerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            ref mut path,
            table: dataframe,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        *path = open_file_dialog(
                            "Select a file",
                            std::env::current_dir().unwrap().as_path().to_str().unwrap(),
                            Some((&["*.parquet"], "")), // this might be broken? I have no idea what the second string is for.
                        )
                        .unwrap_or("".to_string())
                        .into();
                        let file = File::open(path).unwrap();
                        *dataframe = ParquetReader::new(file).finish().unwrap();

                        //  ParquetReader::new(file).finish().unwrap();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            egui::containers::ScrollArea::horizontal().show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's

                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::default().with_main_wrap(false))
                    .columns(
                        Column::auto().resizable(true).clip(true).at_least(20.0),
                        dataframe.schema().len(),
                    )
                    .header(18.0, |mut header| {
                        for (col_name, dtype) in dataframe.schema().iter() {
                            header.col(|ui| {
                                ui.add(
                                    Label::new(RichText::new(col_name.to_string()).strong().heading())
                                        .truncate(true),
                                );
                                ui.heading(dtype.to_string());
                            });
                        }
                    })
                    .body(|body| {
                        let widths = body.widths();
                        let row_count = dataframe.height();
                        let columns = dataframe.get_columns();
                        let len = columns.len();
                        body.rows(18.0, row_count, |row_index, mut row| {
                            let df_row = dataframe.get(row_index).unwrap();
                            for val in df_row {
                                row.col(|ui| {
                                    // let val_string = if val.dtype().is_float() {
                                    //     format!("{:.2}", val.try_extract::<f32>().unwrap())
                                    // } else {
                                    //     val.to_string()
                                    // };
                                    ui.add(
                                        Label::new(RichText::new(val.to_string())).truncate(true),
                                    );
                                });
                            }
                        })
                    });
            });
        });
    }
}
