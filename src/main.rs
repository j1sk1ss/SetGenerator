use eframe::{egui, App, CreationContext, Frame};
mod setter;

struct SetterApp {
    line: usize,
    selected: Vec<setter::series::Series>,
    src_table: setter::table::Table,
    mode: AppMode,
    result_table: Option<setter::table::Table>,
    sets_table: Option<setter::table::Table>,
}

#[derive(PartialEq)]
enum AppMode {
    Welcome,
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
            mode: AppMode::Welcome,
            result_table: None,
            sets_table: None,
        }
    }
}

impl App for SetterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                AppMode::Welcome => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Welcome!");
                        ui.label("Press below button to continue...");
                        if ui.button("Continue").clicked() {
                            self.mode = AppMode::Selection;
                        }
                    });
                }

                AppMode::Selection => {
                    ui.heading("Select Gradations");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, item) in self.src_table.body.iter().enumerate() {
                            let is_selected: bool = self.selected.iter().any(|s| s.equal(item));
                            if ui.selectable_label(self.line == i, format!(
                                    "{}{}",
                                    if is_selected { "[x] " } else { "[ ] " },
                                    item.name()
                                )).clicked() {
                                self.line = i;
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        let current = &self.src_table.body[self.line];
                        let is_already_selected = self.selected.iter().any(|s| s.equal(current));
                    
                        if is_already_selected {
                            if ui.button("Unselect").clicked() {
                                self.selected.retain(|s| !s.equal(current));
                            }
                        } 
                        else {
                            if ui.button("Select").clicked() {
                                self.selected.push(current.clone());
                            }
                        }
                    
                        if ui.button("Generate Series").clicked() {
                            let mut cons_tb: setter::table::Table = setter::generate_series(&self.selected);
                            cons_tb.to_uniq();
                            self.result_table = Some(cons_tb);
                            self.mode = AppMode::Result;
                        }
                    });                    
                }

                AppMode::Result => {
                    ui.heading("Generated Series");
                    if let Some(tb) = &self.result_table {
                        for s in &tb.body {
                            ui.label(s.name());
                        }

                        if ui.button("Generate Sets").clicked() {
                            let mut sets: setter::table::Table = setter::generate_sets(tb).unwrap_or_else(setter::table::Table::empty);
                            sets.sort_series();
                            sets.filter_series_by_range(0., 100.);
                            self.sets_table = Some(sets);
                            self.mode = AppMode::Final;
                        }
                    }
                }

                AppMode::Final => {
                    ui.heading("Possible Sets");
                
                    if let Some(tb) = &self.sets_table {
                        egui::ScrollArea::both().show(ui, |ui| {
                            egui::Grid::new("sets_table_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("#");
                                    ui.label("Count");
                                    ui.label("Values");
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
                
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
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
        Box::new(|_cc: &CreationContext| Box::new(SetterApp::default())),
    )
}
