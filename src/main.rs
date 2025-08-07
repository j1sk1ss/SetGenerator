/* rustup run stable cargo rustc --release --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc */
use eframe::{egui, App, CreationContext, Frame};
use chrono;
mod setter;

struct SetterApp {
    line: usize,
    selected: Vec<setter::series::Series>,
    src_table: setter::table::Table,
    mode: AppMode,
    result_table: Option<setter::table::Table>,
    sets_table: Option<setter::table::Table>,
    manual_input: String,
    min: f64,
    max: f64
}

#[derive(PartialEq)]
enum AppMode {
    Selection,
    Result,
    Final,
}

impl Default for SetterApp {
    fn default() -> Self {
        Self {
            line: 0,
            selected: vec![],
            src_table: setter::table::Table::default(),
            mode: AppMode::Selection,
            result_table: None,
            sets_table: None,
            manual_input: String::new(),
            min: 0f64,
            max: 100f64
        }
    }
}

impl App for SetterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                AppMode::Selection => {
                    ui.heading("Выберите градации");
                
                    let mut manual_input = self.manual_input.clone();
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Ввести градации:");
                            if ui.text_edit_singleline(&mut manual_input).changed() {
                                self.manual_input = manual_input.clone();
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Минимум:");
                            ui.add(egui::DragValue::new(&mut self.min).speed(0.1));
                        
                            ui.label("Максимум:");
                            ui.add(egui::DragValue::new(&mut self.max).speed(0.1));
                        });                        

                        if ui.button("Выбрать вручную").clicked() {
                            let names: Vec<&str> = manual_input.split_whitespace().collect();
                            self.selected.clear();
                            for name in names {
                                if let Some(item) = self.src_table.body.iter().find(|i| i.gradation == name.parse::<f64>().unwrap()) {
                                    if !self.selected.iter().any(|s| s.equal(item)) {
                                        self.selected.push(item.clone());
                                    }
                                }
                            }
                        }
                    });
                
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, item) in self.src_table.body.iter().enumerate() {
                            let is_selected: bool = self.selected.iter().any(|s| s.equal(item));
                            let response = ui.selectable_label(
                                self.line == i,
                                format!(
                                    "{}{}",
                                    if is_selected { "[x] " } else { "[ ] " },
                                    item.name()
                                )
                            );
                
                            if response.clicked() {
                                self.line = i;
                            }
                
                            if response.double_clicked() {
                                self.line = i;
                                if is_selected {
                                    self.selected.retain(|s| !s.equal(item));
                                } else {
                                    self.selected.push(item.clone());
                                }
                            }
                        }
                    });
                
                    ui.horizontal(|ui| {
                        let current = &self.src_table.body[self.line];
                        let is_already_selected = self.selected.iter().any(|s| s.equal(current));
                
                        if is_already_selected {
                            if ui.button("Снять выбор").clicked() {
                                self.selected.retain(|s| !s.equal(current));
                            }
                        } else {
                            if ui.button("Выбрать").clicked() {
                                self.selected.push(current.clone());
                            }
                        }
                
                        if ui.button("Сгенерировать серии").clicked() {
                            let mut cons_tb: setter::table::Table = setter::generate_series(&self.selected);
                            cons_tb.to_uniq();
                            self.result_table = Some(cons_tb);
                            self.mode = AppMode::Result;
                        }
                    });
                }                

                AppMode::Result => {
                    ui.heading("Сгенерированные серии");
                    if let Some(tb) = &self.result_table {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for s in &tb.body {
                                ui.label(s.name());
                            }
                        });

                        if ui.button("Сгенерировать наборы").clicked() {
                            let mut sets: setter::table::Table = setter::generate_sets(tb).unwrap_or_else(setter::table::Table::empty);
                            sets.sort_series();
                            sets.filter_series_by_range(self.min, self.max);
                            self.sets_table = Some(sets);
                            self.mode = AppMode::Final;
                        }
                    }
                }

                AppMode::Final => {
                    ui.heading("Возможные наборы");
                    ui.horizontal(|ui| {
                        if ui.button("Начать заново").clicked() {
                            self.mode = AppMode::Selection;
                            self.selected.clear();
                            self.result_table = None;
                            self.sets_table = None;
                            self.line = 0;
                        }

                        if ui.button("Сохранить в Word").clicked() {
                            if let Some(tb) = &self.sets_table {
                                let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
                                let filename: String = format!("наборы_{}.rtf", now.format("%Y-%m-%d_%H-%M-%S"));

                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Сохранить файл Word")
                                    .set_directory(".")
                                    .set_file_name(&filename)
                                    .save_file()
                                {
                                    match tb.save_table_as_rtf(path.to_str().unwrap()) {
                                        Ok(_) => {
                                            println!("Файл сохранен: {}", path.display());
                                        }
                                        Err(e) => {
                                            eprintln!("Ошибка сохранения: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                    
                        if ui.button("Выход").clicked() {
                            std::process::exit(0);
                        }
                    });

                    if let Some(tb) = &self.sets_table {
                        egui::ScrollArea::both().show(ui, |ui| {
                            egui::Grid::new("sets_table_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("#");
                                    ui.label("Количество");
                                    ui.label("Значения");
                                    ui.end_row();
                
                                    for (i, s) in tb.body.iter().enumerate() {
                                        let values: String = s.series.iter()
                                            .map(|v| format!("{:.3}", v))
                                            .collect::<Vec<_>>()
                                            .join(" ");
                
                                        ui.label((i + 1).to_string());
                                        ui.label(s.series.len().to_string());
                                        ui.label(values);
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                }                                     
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options: eframe::NativeOptions = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Setter",
        options,
        Box::new(|_cc: &CreationContext| {
            Box::new(SetterApp::default())
        }),
    )
}