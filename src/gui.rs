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

    #[cfg(target_arch = "wasm32")]
    calendar_string: String,
}

impl Default for StrengthYogaApp {
    fn default() -> Self {
        Self {
            weekly_activities: activities::WeeklyActivities::default(),
            total_weeks: 12, // total number of weeks to create a schedule for
            start_date: Local::now().date_naive(), // when the workout schedule starts
            recovery_weeks_bools: std::iter::repeat(false).take(120).collect::<Vec<bool>>(), // list of bools for changing workout week selection
            recovery_weeks: calendar::recovery_days(Local::now().date_naive(), 120), // list of mondays for workout weeks

            #[cfg(target_arch = "wasm32")]
            calendar_string: String::default(),
        }
    }
}

impl StrengthYogaApp {
    // Changes all recovery_weeks_bools back to false
    pub fn reset_recovery_bool(&mut self) {
        self.recovery_weeks_bools = std::iter::repeat(false).take(120).collect::<Vec<bool>>()
    }

    // Function used to set every 3rd or 4th reovery week bool to true
    pub fn set_recovery_bool_repeat(&mut self, step: usize) {
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
    // The GUI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Yoga level area
            ui.label("Yoga Level:");
            ui.add_space(5.);
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("Yoga")
                    .selected_text(self.weekly_activities.yoga_level.to_string())
                    .show_ui(ui, |ui| {
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
            ui.add_space(10.);
            ui.separator();

            // Strength level area
            ui.add_space(10.);
            ui.label("Strength Level:");
            ui.add_space(5.);
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("Strength")
                    .selected_text(self.weekly_activities.strength_level.to_string())
                    .show_ui(ui, |ui| {
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
            ui.add_space(10.);
            ui.separator();

            // Total weeks selection
            ui.add_space(10.);
            ui.label("Total Weeks:");
            ui.add_space(5.);
            ui.add(egui::Slider::new(&mut self.total_weeks, 1..=52)); // set to max of 52 weeks
            ui.add_space(10.);
            ui.separator();

            // Start date selection
            ui.add_space(10.);
            ui.label("Start Date:");
            ui.add_space(5.);
            ui.add(DatePickerButton::new(&mut self.start_date));
            ui.add_space(10.);
            ui.separator();

            // Recovery weeks selection
            ui.add_space(10.);
            ui.horizontal(|ui| {
                ui.label("Recovery Weeks:");
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
            ui.add_space(5.);
            ui.vertical(|ui| {
                let last_monday = calendar::last_monday(self.start_date, self.total_weeks);
                let monday_start = calendar::monday_start(self.start_date);
                for x in self
                    .recovery_weeks
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d >= &&monday_start) // Only include Mondays after the Start Date
                    .map(|(x, _)| x) // Keep only the indexes
                    .take(self.total_weeks as usize) // Take as many as there are weeks
                    .step_by(4)
                // Take every 4th to create 4 columns later
                {
                    ui.horizontal(|ui| {
                        // Create 4 columns for the dates
                        for y in 0..4 {
                            // Only do so if it is within the range of the recovery weeks and stop
                            // when the last monday is reached
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
            ui.add_space(10.);
            ui.separator();

            // Save icalendar area
            ui.add_space(10.);

            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Save").clicked() {
                // Create a recovery weeks vec to feed into the create_ics function
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
                )
                .unwrap();
            };

            #[cfg(target_arch = "wasm32")]
            ui.vertical(|ui| {
                if ui.button("Update text").clicked() {
                    self.weekly_activities.update_activities(); // Update the yoga and strength activities as this has not been done yet

                    // Create a recovery weeks vec to feed into the create_ics function
                    let recovery_weeks = self
                        .recovery_weeks
                        .iter()
                        .zip(&self.recovery_weeks_bools)
                        .filter_map(|(d, b)| if *b { Some(*d) } else { None })
                        .collect::<Vec<NaiveDate>>();
                    self.calendar_string = calendar::create_ics(
                        self.start_date,
                        self.total_weeks,
                        recovery_weeks,
                        self.weekly_activities,
                    )
                    .unwrap();
                }
                ui.label("Copy text below into a workout.ics file:");
                egui::ScrollArea::vertical()
                    .id_source("ics_text_scroll")
                    .show(ui, |ui| {
                        ui.text_edit_multiline(&mut self.calendar_string);
                    })
            });
        });
    }
}
