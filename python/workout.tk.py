#!/usr/bin/env python

from argparse import ArgumentParser
from pandas import read_excel
from datetime import date, datetime
from dateutil.relativedelta import relativedelta
from ics import Calendar, Event
import tkinter as tk
from tkinter.filedialog import asksaveasfile
import tkcalendar


YOGA = {
    "Beginer": [
        ["Total Newbie Yoga", "Strength", "Basic Yoga Poses", "Strength"],
        ["Super Easy Stretch Routine I", "Strength", "Loosen Neck and Shoulders", "Strength"],
        ["Shoulder Stability", "Strength", "Shake Off the Day", "Strength"],
        ["Neck and upper Back Recovery", "Strength", "Recovery Booster", "Strength"],
        ["Hip Flexor and Groin Recovery", "Strength", "Super Easy Stretch Routine I", "Strength"],
        ["Foot and Ankle Recovery", "Strength", "Post-Workout Cool-Down", "Strength"],
        ["Morning Yoga Routine", "Strength", "Super Easy Stretch Routine II", "Strength"],
        ["Post-Workout Cool-Down", "Strength", "Mobilize the Joints", "Strength"],
        ["Lower Back Recovery II", "Strength", "Post-Workout Cool-Down", "Strength"],
    ],
    "Beginer_recovery": [
        ["Morning Yoga Routine", "Strength", "Basic Yoga Poses", "Strength"],
        ["Lower Back Recovery I", "Strength", "Thoracic Recovery", "Strength"],
        ["Morning Yoga Routine", "Strength", "Recovery Booster", "Strength"],
    ],
    "Intermediate": [
        ["Morning Yoga Routine", "Strength", "Improving Posture I", "Basic Yoga Poses", "Strength"],
        ["Mobilize and Activate I", "Strength", "Super Easy Stretch I", "Side Bends", "Strength"],
        ["Full Body Mobility", "Strength", "Yoga for Travel", "Super Easy Stretch III", "Strength"],
        ["Hamstring and Calf Flexibility", "Strength", "Scapular Stability", "Core Stability", "Strength"],
        ["Shoulder Stability", "Strength", "Pre-Ride Warm-Up", "Post-Workout Cool-Down", "Strength"],
        ["Foot and Ankle Recovery", "Strength", "Loosen Neck and Shoulders", "Hips and Hamstrings", "Strength"],
        ["Morning Yoga Routine", "Strength", "Mobilize the Joints", "Recovery Booster", "Strength"],
        ["Post-Workout Cool-Down", "Strength", "Mobilize and Activate II", "Improving Posture II", "Strength"],
        ["Morning Yoga Routine", "Strength", "Core Strengtheners III", "Post-Workout Cool-Down", "Strength", "Lower Back Recovery II"],
    ],
    "Intermediate_recovery": [
        ["Upper Body Strength", "Strength", "Hip Openers I", "Core Strengtheners I", "Strength", "Shake Off the Day"],
        ["Lower Back Recovery I", "Strength", "Core Strengtheners II", "Hip and Pelvis Stability", "Strength", "Thoracic Recovery"],
        ["Hip Flexor and Groin Recovery", "Strength", "Recovery Booster", "Neck and upper Back Recovery", "Strength"],
    ],
    "Advanced": [
        ["Morning Yoga Routine", "Strength", "Improving Posture I", "Balance and Agility I", "Strength", "Super Easy Stretch Routin III"],
        ["Mobilize and Activate I", "Strength", "Yoga for Travel", "Side Bends", "Strength", "Hip Openers I"],
        ["Full Body Mobility", "Strength", "Shake Off the Day", "Scapular Stability", "Strength", "Hip Flexor and Groin Recovery"],
        ["Pre-Ride Warm-Up", "Strength", "Shake Off the Day", "Core Stability", "Strength", "Hips and Hamstrings"],
        ["Knee and Ankle Stability", "Strength", "Hip Openers II", "Post-Workout Cool-Down", "Strength", "Side Bends"],
        ["Foot and Ankle Recovery", "Strength", "Loosen Neck and Shoulders", "Hips and Hamstrings", "Strength", "Lower Back Recovery I"],
        ["Morning Yoga Routine", "Strength", "Balance and Agility II", "Recovery Booster", "Strength", "Mobilize the Joints"],
        ["Post-Workout Cool-Down", "Strength", "Mobilize and Activate II", "Improving Posture II", "Strength", "Super Easy Stretch Routine III"],
        ["Morning Yoga Routine", "Strength", "Core Strengtheners III", "Post-Workout Cool-Down", "Strength", "Neck and upper Back Recovery"],
    ],
    "Advanced_recovery": [
        ["Core Strengtheners I", "Strength", "Hamstring and Calf Flexibility", "Back Strengtheners", "Strength", "Shoulder Stability"],
        ["Upper Body Strength", "Strength", "Hip and Pelvis Stability", "Core Strengtheners III", "Strength", "Thoracic Recovery"],
        ["Lower Back Recovery II", "Strength", "Hip Flexor and Groin Recovery", "Shake Off the Day", "Strength", "Recovery Booster"],
    ],
}

STRENGTH = {
    "1": [
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Posterior Chain Focus 01"],
        ["Full Body 03", "Full Body 02"],
        ["Full Body 03", "Posterior Chain Focus 01"],
        ["Full Body 04", "Full Body 03"],
        ["Full Body 04", "Posterior Chain Focus 01"],
        ["Full Body 04", "Posterior Chain Focus 01"],
        ["Full Body 05", "Full Body 06"],
        ["Full Body 06", "Posterior Chain Focus 01"],
    ],
    "1_recovery": [
        ["Recovery Session A", "Recovery Session B"],
        ["Recovery Session B", "Recovery Session A"],
        ["Recovery Session A", "Recovery Session B"],
    ],
    "2": [
        ["Full Body 05", "Posterior Chain Focus 01"],
        ["Full Body 05", "Full Body 06"],
        ["Full Body 07", "Posterior Chain Focus 01"],
        ["Full Body 06", "Full Body 07"],
        ["Full Body 08", "Posterior Chain Focus 01"],
        ["Full Body 07", "Posterior Chain Focus 02"],
        ["Full Body 07", "Full Body 08"],
        ["Core Focus 01", "Full Body 09"],
        ["Full Body 08", "Posterior Chain Focus 02"],
    ],
    "2_recovery": [
        ["Recovery Session A", "Recovery Session B"],
        ["Recovery Session A", "Recovery Session B"],
        ["Recovery Session A", "Recovery Session B"],
    ],
    "3": [
        ["Full Body 09", "Posterior Chain Focus 01"],
        ["Core Focus 01", "Full Body 09"],
        ["Full Body 10", "Posterior Chain Focus 02"],
        ["Full Body 11", "Posterior Chain Focus 03"],
        ["Core Focus 02", "Lower Body Focus 01"],
        ["Full Body 11", "Posterior Chain Focus 03"],
        ["Full Body 11", "Posterior Chain Focus 03"],
        ["Core Focus 02", "Lower Body Focus 01"],
        ["Full Body 12", "Posterior Chain Focus 03"],
    ],
    "3_recovery": [
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Recovery Session B"],
    ],
    "4": [
        ["Full Body 12", "Posterior Chain Focus 03"],
        ["Core Focus 02", "Lower Body Focus 01"],
        ["Full Body 13", "Posterior Chain Focus 03"],
        ["Lower Body Focus 02", "Posterior Chain Focus 04"],
        ["Core Focus 03", "Full Body 15"],
        ["Lower Body Focus 02", "Posterior Chain Focus 04"],
        ["Lower Body Focus 03", "Posterior Chain Focus 04"],
        ["Core Focus 04", "Full Body 16"],
        ["Lower Body Focus 03", "Posterior Chain Focus 05"],
    ],
    "4_recovery": [
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Recovery Session B"],
    ],
    "5": [
        ["Full Body 15", "Posterior Chain Focus 04"],
        ["Core Focus 03", "Lower Body Focus 03"],
        ["Full Body 16", "Posterior Chain Focus 04"],
        ["Lower Body Focus 03", "Posterior Chain Focus 04"],
        ["Core Focus 04", "Full Body 17"],
        ["Lower Body Focus 04", "Posterior Chain Focus 05"],
        ["Lower Body Focus 04", "Posterior Chain Focus 05"],
        ["Core Focus 02", "Full Body 10"],
        ["Lower Body Focus 01", "Posterior Chain Focus 02"],
    ],
    "5_recovery": [
        ["Full Body 01", "Full Body 02"],
        ["Full Body 01", "Full Body 02"],
        ["Recovery Session A", "Recovery Session B"],
    ],
}

STRENGTH_ANNOTATION = {
    "Core Focus 01": "",
    "Core Focus 02": "",
    "Core Focus 03": "",
    "Core Focus 04": "Exercise band",
    "Full Body 01": "Empty drink bottle",
    "Full Body 02": "Empty drink bottle",
    "Full Body 05": "Filled drink bottle",
    "Full Body 06": "2 Filled drink bottles",
    "Full Body 07": "2 Filled drink bottles",
    "Full Body 08": "2 Filled drink bottles",
    "Full Body 09": "2 Filled drink bottles",
    "Full Body 10": "2 Filled drink bottles",
    "Full Body 11": "2 Filled drink bottles",
    "Full Body 12": "2 Filled drink bottles",
    "Full Body 13": "2 Filled drink bottles",
    "Full Body 15": "2 Filled drink bottles",
    "Full Body 16": "2 Filled drink bottles",
    "Full Body 17": "2 Filled drink bottles",
    "Lower Body Focus 01": "Chair",
    "Lower Body Focus 02": "Chair",
    "Lower Body Focus 03": "Filled drink bottle",
    "Lower Body Focus 04": "Chair, drink bottle, exercise band",
    "Posterior Chain Focus 01": "",
    "Posterior Chain Focus 02": "",
    "Posterior Chain Focus 03": "Stick, filled drink bottle",
    "Posterior Chain Focus 04": "Exercise band, filled drink bottle",
    "Posterior Chain Focus 05": "Exercise band",
    "Recovery Session A": "Empty drink bottle",
    "Recovery Session B": "",
}
YOGA_PROGRESS = ("Beginer", "Intermediate", "Advanced")
STRENGTH_PROGRESS = ("1", "2", "3", "4", "5")


def monday(current_date: date, previous: bool) -> date:
    """
    Yoga starts on Monday, so this makes sure every week starts with a Monday
    """
    while current_date.weekday() != 0:
        if previous:
            current_date -= relativedelta(days=1)
        else:
            current_date += relativedelta(days=1)
    return current_date


def combine_yoga_strength(yoga_activities_start: list, strength_activities_start: list):
    """
    Creates the week by combing the yoga and strength activities chosen
    """
    yoga_activities = yoga_activities_start.copy()
    strength_activities = strength_activities_start.copy()
    final_activities = []
    activity_text = []
    for activity in yoga_activities:
        if activity == "Strength":
            strength_activity = strength_activities.pop(0)
            final_activities.append(f"Strength: {strength_activity}")
            activity_text.append(STRENGTH_ANNOTATION[strength_activity])
        else:
            final_activities.append(f"Yoga: {activity}")
            activity_text.append("")
    return zip(final_activities, activity_text)


def create():
    recovery_weeks = [
        date.fromisoformat(recovery_weeks_box.get(i))
        for i in recovery_weeks_box.curselection()
    ] + [ 
            date.fromisoformat(recovery_weeks_box_2.get(i)) 
            for i in recovery_weeks_box_2.curselection() 
            ]
    
    current_yoga = yoga_variable.get()
    current_strength = strength_variable.get()
    total_weeks_str = weeks.get()
    if total_weeks_str == "":
        total_weeks = 0
    else:
        total_weeks = int(total_weeks_str)

    progress_yoga = yoga_progress_bool.get()
    progress_strength = strength_progress_bool.get()

    current_yoga_index = YOGA_PROGRESS.index(current_yoga)
    current_strength_index = STRENGTH_PROGRESS.index(current_strength)

    start_date = start_date_calendar.get_date()
    current_date = datetime.strptime(start_date, "%m/%d/%y").date()
    current_date = monday(current_date, previous=True)

    calendar = Calendar()

    week = 0
    week_activity = 0
    week_recovery = 0
    while week < total_weeks:
        current_date = monday(current_date, previous=False)
        if week_activity == 0:
            event = Event()
            event.begin = str(current_date)
            event.name = f"Starting: {current_yoga} Yoga and Strength {current_strength}"
            event.make_all_day()
            calendar.events.add(event)

        if current_date in recovery_weeks:
            yoga_activities = YOGA[f"{current_yoga}_recovery"][week_recovery]
            strength_activities = STRENGTH[f"{current_strength}_recovery"][
                week_recovery
            ]
            week_recovery += 1
            if week_recovery == len(YOGA[f"{current_yoga}_recovery"]):
                week_recovery = 0
        else:
            yoga_activities = YOGA[current_yoga][week_activity]
            strength_activities = STRENGTH[current_strength][week_activity]
            week_activity += 1
            if week_activity == len(YOGA[current_yoga]):
                week_recovery = 0
                week_activity = 0
                if progress_strength:
                    current_strength_index += 1
                    if current_strength_index == len(STRENGTH_PROGRESS):
                        current_strength_index -= 1
                if progress_yoga:
                    current_yoga_index += 1
                    if current_yoga_index == len(YOGA_PROGRESS):
                        current_yoga_index -= 1
                current_yoga = YOGA_PROGRESS[current_yoga_index]
                current_strength = STRENGTH_PROGRESS[current_strength_index]
        for activity, activity_text in combine_yoga_strength(
            yoga_activities, strength_activities
        ):
            event = Event()
            event.begin = str(current_date)
            event.name = activity
            event.description = activity_text
            event.make_all_day()
            calendar.events.add(event)
            current_date += relativedelta(days=1)
            if current_yoga == "Beginer" and current_date.weekday() == 2:
                current_date += relativedelta(days=1)
        week += 1
    files = [("calendar file", "*.ics")]
    workout_file = asksaveasfile(mode="w", filetypes=files, defaultextension=files)

    # with open(file, 'w') as workout_file:
    workout_file.writelines(calendar.serialize_iter())
    workout_file.close()


def create_recovery_weeks():
    """
    Creates the selection list for recovery weeks
    """
    start_date = start_date_calendar.get_date()
    current_date = datetime.strptime(start_date, "%m/%d/%y").date()
    current_date = monday(current_date, previous=True)
    total_weeks_str = weeks.get()
    if total_weeks_str == "":
        total_weeks = 0
    else:
        total_weeks = int(total_weeks_str)
    selection_list = [current_date]
    for _ in range(total_weeks - 1):
        current_date += relativedelta(days=7)
        selection_list.append(current_date)
    for index, week in enumerate(selection_list):
        if index <= int(len(selection_list)/2):
            recovery_weeks_box.insert(index, week)
        else:
            recovery_weeks_box_2.insert(index, week)
    recovery_weeks_box.grid(row=9, column=0, sticky="nw", padx=5, pady=(0, 5))
    recovery_weeks_box_2.grid(row=9, column=1, sticky="nw", padx=5, pady=(0, 5))
    save_button.grid(row=10, column=0, sticky="nw", padx=5, pady=5)


window = tk.Tk()
window.title("Cycling Workout Calendar Creator")
window.rowconfigure([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], minsize=10)
window.columnconfigure([0, 1], minsize=20)

yoga_label = tk.Label(window, text="Yoga level:")
yoga_label.grid(row=0, column=0, sticky="nw", padx=5)
progress_label = tk.Label(window, text="After completion:")
progress_label.grid(row=0, column=1, sticky="nw", padx=5)
yoga_variable = tk.StringVar(window)
yoga_variable.set(YOGA_PROGRESS[0])  # default value
yoga_option = tk.OptionMenu(window, yoga_variable, *YOGA_PROGRESS)
yoga_option.grid(row=1, column=0, sticky="nw", padx=5, pady=(0, 5))
yoga_progress_bool = tk.BooleanVar()
yoga_progress_button = tk.Checkbutton(window, text="Progress yoga", onvalue=True, offvalue=False, variable=yoga_progress_bool)
yoga_progress_button.grid(row=1, column=1, sticky="nsw", padx=5)

strength_label = tk.Label(window, text="Strength level:")
strength_label.grid(row=2, column=0, sticky="nw", padx=5)
strength_variable = tk.StringVar(window)
strength_variable.set(STRENGTH_PROGRESS[0])  # default value
strength_option = tk.OptionMenu(window, strength_variable, *STRENGTH_PROGRESS)
strength_option.grid(row=3, column=0, sticky="nw", padx=5, pady=(0, 5))
strength_progress_bool = tk.BooleanVar()
strength_progress_button = tk.Checkbutton(window, text="Progress strength", onvalue=True, offvalue=False, variable=strength_progress_bool)
strength_progress_button.grid(row=3, column=1, sticky="nsw", padx=5)

weeks_label = tk.Label(window, text="Total weeks: ")
weeks_label.grid(row=4, column=0, sticky="nw", padx=5)
weeks = tk.Entry(window, width=2)
weeks.grid(row=5, column=0, sticky="nw", padx=5, pady=(0, 5))

start_date_label = tk.Label(window, text="Start date:")
start_date_label.grid(row=6, column=0, sticky="nw", padx=5)
start_date_calendar = tkcalendar.Calendar(window, selectmode="day")
start_date_calendar.grid(row=7, column=0, sticky="nw", padx=5, pady=(0, 5))


recovery_weeks_box = tk.Listbox(window, height=13, selectmode="multiple", exportselection=0)
recovery_weeks_box_2 = tk.Listbox(window, height=13, selectmode="multiple", exportselection=0)

save_button = tk.Button(window, text="Save", command=create)

recovery_button = tk.Button(
    window, text="Select recovery weeks", command=create_recovery_weeks
)
recovery_button.grid(row=8, column=0, sticky="nw", padx=5)


window.mainloop()
