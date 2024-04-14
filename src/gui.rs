use crate::activities;
use crate::calendar;
use chrono::{Local, NaiveDate};
use eframe::egui;
use egui_extras::DatePickerButton;

pub struct StrengthYogaApp {
    weekly_activities: activities::WeeklyActivities,
    total_weeks: u32,
    start_date: NaiveDate,
    recovery_weeks_bools: Vec<bool>,
    recovery_weeks: Vec<NaiveDate>,
}

impl Default for StrengthYogaApp {
    fn default() -> Self {
        Self {
            weekly_activities: activities::WeeklyActivities::default(),
            total_weeks: 12,
            start_date: Local::now().date_naive(),
            recovery_weeks_bools: std::iter::repeat(false).take(120).collect::<Vec<bool>>(),
            recovery_weeks: calendar::recovery_days(Local::now().date_naive(), 120),
        }
    }
}

impl StrengthYogaApp {
    pub fn reset_recovery_bool(&mut self) -> () {
        self.recovery_weeks_bools = std::iter::repeat(false).take(120).collect::<Vec<bool>>()
    }

    pub fn set_recovery_bool_repeat(&mut self, step: usize) -> () {
        self.reset_recovery_bool();
        for x in self
            .recovery_weeks
            .iter()
            .enumerate()
            .filter(|(_, d)| d >= &&calendar::monday_start(self.start_date))
            .map(|(x, _)| x)
            .skip(step)
            .take(self.total_weeks as usize)
            .step_by(step)
        {
            self.recovery_weeks_bools[x - 1] = true
        }
    }
}

impl eframe::App for StrengthYogaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Yoga Level") // When created from a label the text will b shown on the side of the combobox
                    .selected_text(self.weekly_activities.yoga_level.to_string()) // This is the currently selected option (in text form)
                    .show_ui(ui, |ui| {
                        // In this closure the various options can be added
                        for yoga_level in [
                            activities::YogaLevel::NotSet,
                            activities::YogaLevel::Beginer,
                            activities::YogaLevel::Intermediate,
                            activities::YogaLevel::Advanced,
                        ] {
                            ui.selectable_value(
                                &mut self.weekly_activities.yoga_level,
                                yoga_level,
                                yoga_level.to_string(),
                            );
                        }
                    });
                ui.checkbox(
                    &mut self.weekly_activities.progress_yoga,
                    "Progress when done",
                );
            });

            ui.add_space(16.);

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Strength Level") // When created from a label the text will b shown on the side of the combobox
                    .selected_text(self.weekly_activities.strength_level.to_string()) // This is the currently selected option (in text form)
                    .show_ui(ui, |ui| {
                        // In this closure the various options can be added
                        for strength_level in [
                            activities::StrengthLevel::NotSet,
                            activities::StrengthLevel::Strength1,
                            activities::StrengthLevel::Strength2,
                            activities::StrengthLevel::Strength3,
                            activities::StrengthLevel::Strength4,
                            activities::StrengthLevel::Strength5,
                        ] {
                            ui.selectable_value(
                                &mut self.weekly_activities.strength_level,
                                strength_level,
                                strength_level.to_string(),
                            );
                        }
                    });
                ui.checkbox(
                    &mut self.weekly_activities.progress_strength,
                    "Progress when done",
                );
            });

            ui.add_space(16.);

            ui.add(egui::Slider::new(&mut self.total_weeks, 1..=52).text("Total weeks"));

            ui.add_space(16.);

            ui.add(DatePickerButton::new(&mut self.start_date));

            ui.add_space(16.);

            ui.horizontal(|ui| {
                ui.label("Select recovery weeks:");
                if ui.button("3:1").clicked() {
                    self.set_recovery_bool_repeat(4)
                }
                if ui.button("2:1").clicked() {
                    self.set_recovery_bool_repeat(3)
                }
                if ui.button("Reset").clicked() {
                    self.reset_recovery_bool()
                }
            });

            ui.add_space(6.);

            ui.separator();
            ui.vertical(|ui| {
                let last_monday = calendar::last_monday(self.start_date, self.total_weeks);
                let monday_start = calendar::monday_start(self.start_date);
                for x in self
                    .recovery_weeks
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d >= &&monday_start)
                    .map(|(x, _)| x)
                    .take(self.total_weeks as usize)
                    .step_by(4)
                {
                    ui.horizontal(|ui| {
                        for y in 0..4 {
                            if (x + y) < self.recovery_weeks.len()
                                && self.recovery_weeks[x + y] < last_monday
                            {
                                ui.checkbox(
                                    &mut self.recovery_weeks_bools[x + y],
                                    format!("{:?}", self.recovery_weeks[x + y]),
                                );
                            }
                        }
                    });
                }
            });

            ui.separator();
            ui.add_space(16.);
            if ui.button("Save").clicked() {
                self.weekly_activities.update_activities();
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name("workout.ics")
                    .save_file()
                {
                    let recovery_weeks = self
                        .recovery_weeks
                        .iter()
                        .zip(&self.recovery_weeks_bools)
                        .filter_map(|(d, b)| if *b { Some(*d) } else { None })
                        .collect::<Vec<NaiveDate>>();
                    let _ = calendar::create_ics(
                        self.start_date,
                        self.total_weeks,
                        recovery_weeks,
                        self.weekly_activities,
                        path,
                    );
                }
            }
        });
    }
}
