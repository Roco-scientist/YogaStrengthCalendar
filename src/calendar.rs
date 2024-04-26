use crate::activities::{self, WeeklyActivities};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use icalendar::{Calendar, Component, Event, EventLike};
use std::future::Future;
use anyhow::Result;


/// Creates and saves the icalendar file with the input choices from the GUI
pub fn create_ics(
    start: NaiveDate,
    weeks: u32,
    recovery_weeks: Vec<NaiveDate>,
    mut weekly_activities: WeeklyActivities,
) -> Result<WeeklyActivities>{
    // Create  a new calendar to place events into
    let mut calendar = Calendar::new();

    // All excercise weeks start on a Monday
    let mut current_date = monday(start, MondayType::Previous);

    // Create as many weeks as were chosen
    for _ in 0..weeks {
        // If the workout cycle is at the begining, create an even indicating which level of yoga
        // and strength are starting.  This cycle repeats after every 9 active weeks, not including
        // recovery weeks.
        if weekly_activities.begining() {
            calendar.push(
                Event::new()
                    .all_day(current_date)
                    .summary(&weekly_activities.begining_description())
                    .done(),
            );
        }

        // Set as either recovery or active week, depending on what weeks were selected for
        // recovery
        let week_type = if recovery_weeks.contains(&current_date) {
            activities::WeekType::Recovery
        } else {
            activities::WeekType::Active
        };

        // Create daily calendar events for the week and preface the text with either strength or
        // yoga, depending on the activity type
        for activity in weekly_activities.next_week_activities(week_type) {
            if let activities::ActivityType::Yoga(yoga_name) = activity {
                calendar.push(
                    Event::new()
                        .all_day(current_date)
                        .summary(&format!("Yoga: {}", yoga_name))
                        .done(),
                );
            } else if let activities::ActivityType::Strength(strength_name) = activity {
                let strength_added_info = activities::strength_added_info(strength_name)?;
                calendar.push(
                    Event::new()
                        .all_day(current_date)
                        .summary(&format!("Strength: {}", strength_name))
                        .description(&strength_added_info)
                        .done(),
                );
            }
            // Advance by one day to start the next calendar day event
            current_date += Duration::days(1);
        }
        // Make sure the next week starts on a Monday
        current_date = monday(current_date, MondayType::Next);
    }
    let calendar_text = format!("{}", calendar);

    save_ics(calendar_text.clone());
    Ok(weekly_activities)
}

fn save_ics(calendar_text: String) {
    #[cfg(not(target_arch = "wasm32"))]
    let task = rfd::AsyncFileDialog::new().set_file_name("workout.ics").save_file();

    #[cfg(target_arch = "wasm32")]
    let task = rfd::AsyncFileDialog::new().save_file();

    execute(async move {
        let file = task.await;
        if let Some(file) = file {
            _ = file.write(calendar_text.as_bytes()).await;
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

// Used to determine if the monday() function advances to the next monday or goes to the previous
// monday
enum MondayType {
    Previous,
    Next,
}

// Finds the closest Monday
fn monday(mut day: NaiveDate, monday_type: MondayType) -> NaiveDate {
    while day.weekday() != Weekday::Mon {
        match monday_type {
            MondayType::Next => day += Duration::days(1),
            MondayType::Previous => day -= Duration::days(1),
        }
    }
    day
}

// Finds the previous Monday for the Start Date
pub fn monday_start(start: NaiveDate) -> NaiveDate {
    monday(start, MondayType::Previous)
}

// Finds the last Monday that corresponds to the number of weeks selected
pub fn last_monday(start: NaiveDate, weeks: u32) -> NaiveDate {
    let current_monday = monday_start(start);
    current_monday + Duration::days(weeks as i64 * 7)
}

// Creates a list of all Mondays which can be chosen for recovery weeks within the GUI
pub fn recovery_days(start: NaiveDate, weeks: u32) -> Vec<NaiveDate> {
    let mut first_day_of_week = monday(start, MondayType::Previous);
    let mut recovery_weeks = vec![first_day_of_week];
    for _ in 0..(weeks - 1) {
        first_day_of_week += Duration::days(7);
        recovery_weeks.push(first_day_of_week);
    }
    recovery_weeks
}
