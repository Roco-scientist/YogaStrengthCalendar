use crate::activities;
use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use icalendar::{Calendar, Component, Event, EventLike};
use std::{fs, path::PathBuf};

pub fn create_ics(
    start: NaiveDate,
    weeks: u32,
    recovery_weeks: Vec<NaiveDate>,
    mut weekly_activities: activities::WeeklyActivities,
    destination: PathBuf,
) -> Result<()> {
    let mut calendar = Calendar::new();
    let mut current_date = monday(start, MondayType::Previous);
    for _ in 0..weeks {
        if weekly_activities.begining() {
            calendar.push(
                Event::new()
                    .all_day(current_date)
                    .summary(&weekly_activities.begining_description())
                    .done(),
            );
        }
        let week_type;
        if recovery_weeks.contains(&current_date) {
            week_type = activities::WeekType::Recovery;
        } else {
            week_type = activities::WeekType::Active;
        }
        for activity in weekly_activities.next_week_activities(week_type) {
            println!("{:?}", activity);
            if let activities::ActivityType::Yoga(yoga_name) = activity {
                calendar.push(
                    Event::new()
                        .all_day(current_date)
                        .summary(&format!("Yoga: {}", yoga_name))
                        .done(),
                );
            } else if let activities::ActivityType::Strength(strength_name) = activity {
                calendar.push(
                    Event::new()
                        .all_day(current_date)
                        .summary(&format!("Strength: {}", strength_name))
                        .done(),
                );
            }
            current_date += Duration::days(1);
        }
        current_date = monday(current_date, MondayType::Next);
    }
    save_ics(calendar, destination)?;
    Ok(())
}

fn save_ics(calendar: Calendar, destination: PathBuf) -> Result<()> {
    fs::write(destination, format!("{}", calendar))?;
    Ok(())
}

enum MondayType {
    Previous,
    Next,
}

fn monday(mut day: NaiveDate, monday_type: MondayType) -> NaiveDate {
    while day.weekday() != Weekday::Mon {
        match monday_type {
            MondayType::Next => day += Duration::days(1),
            MondayType::Previous => day -= Duration::days(1),
        }
    }
    day
}

pub fn monday_start(start: NaiveDate) -> NaiveDate {
    monday(start, MondayType::Previous)
}

pub fn last_monday(start: NaiveDate, weeks: u32) -> NaiveDate {
    let current_monday = monday_start(start);
    current_monday + Duration::days(weeks as i64 * 7)
}

pub fn recovery_days(start: NaiveDate, weeks: u32) -> Vec<NaiveDate> {
    let mut first_day_of_week = monday(start, MondayType::Previous);
    let mut recovery_weeks = vec![first_day_of_week.clone()];
    for _ in 0..(weeks - 1) {
        first_day_of_week += Duration::days(7);
        recovery_weeks.push(first_day_of_week.clone());
    }
    recovery_weeks
}
