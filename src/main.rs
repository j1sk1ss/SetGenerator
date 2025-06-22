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
                            let is_selected = self.selected.iter().any(|s| s.equal(item));

                            if ui
                                .selectable_label(self.line == i, format!(
                                    "{}{}",
                                    if is_selected { "[x] " } else { "[ ] " },
                                    item.name()
                                ))
                                .clicked() {
                                self.line = i;
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Select").clicked() {
                            let current = &self.src_table.body[self.line];
                            if !self.selected.iter().any(|s| s.equal(current)) {
                                self.selected.push(current.clone());
                            }
                        }

                        if ui.button("Generate Series").clicked() {
                            let mut cons_tb = setter::generate_series(&self.selected).unwrap_or_else(setter::table::Table::empty);
                            cons_tb.to_uniq();
                            cons_tb.sort_series();
                            cons_tb.filter_series_by_range(0., 100.);
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
                            let sets = setter::generate_sets(tb).unwrap_or_else(setter::table::Table::empty);
                            self.sets_table = Some(sets);
                            self.mode = AppMode::Final;
                        }
                    }
                }

                AppMode::Final => {
                    ui.heading("Possible Sets");
                    if let Some(tb) = &self.sets_table {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for s in &tb.body {
                                ui.label(s.name());
                            }
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
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Setter",
        options,
        Box::new(|_cc: &CreationContext| Box::new(SetterApp::default())),
    )
}
