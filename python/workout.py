from argparse import ArgumentParser
from pandas import read_excel
from datetime import date
from dateutil.relativedelta import relativedelta
from ics import Calendar, Event

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
            ["Lower Back Recovery II", "Strength", "Post-Workout Cool-Down", "Strength"]
            ],
        "Beginer_recovery": [
            ["Morning Yoga Routine", "Strength", "Basic Yoga Poses", "Strength"],
            ["Lower Back Recovery I", "Strength", "Thoracic Recovery", "Strength"],
            ["Morning Yoga Routine", "Strength", "Recovery Booster", "Strength"]
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
            ["Morning Yoga Routine", "Strength", "Core Strengtheners III", "Post-Workout Cool-Down", "Strength", "Lower Back Recovery II"]
            ],
        "Intermediat_recovery": [
            ["Upper Body Strength", "Strength", "Hip Openers I", "Core Strengtheners I", "Strength", "Shake Off the Day"],
            ["Lower Back Recovery I", "Strength", "Core Strengtheners II", "Hip and Pelvis Stability", "Strength", "Thoracic Recovery"],
            ["Hip Flexor and Groin Recovery", "Strength", "Recovery Booster", "Neck and upper Back Recovery", "Strength"]
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
            ["Morning Yoga Routine", "Strength", "Core Strengtheners III", "Post-Workout Cool-Down", "Strength", "Neck and upper Back Recovery"]
            ],
        "Advanced_recovery": [
            ["Core Strengtheners I", "Strength", "Hamstring and Calf Flexibility", "Back Strengtheners", "Strength", "Shoulder Stability"],
            ["Upper Body Strength", "Strength", "Hip and Pelvis Stability", "Core Strengtheners III", "Strength", "Thoracic Recovery"],
            ["Lower Back Recovery II", "Strength", "Hip Flexor and Groin Recovery", "Shake Off the Day", "Strength", "Recovery Booster"]
            ]
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
            ["Full Body 06", "Posterior Chain Focus 01"]
            ],
        "1_recovery": [
            ["Recovery Session A", "Recovery Session B"],
            ["Recovery Session B", "Recovery Session A"],
            ["Recovery Session A", "Recovery Session B"]
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
            ["Full Body 08", "Posterior Chain Focus 02"]
            ],
        "2_recovery": [
            ["Recovery Session A", "Recovery Session B"],
            ["Recovery Session A", "Recovery Session B"],
            ["Recovery Session A", "Recovery Session B"]
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
            ["Full Body 12", "Posterior Chain Focus 03"]
            ],
        "3_recovery": [
            ["Full Body 01", "Full Body 02"],
            ["Full Body 01", "Full Body 02"],
            ["Full Body 01", "Recovery Session B"]
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
            ["Lower Body Focus 03", "Posterior Chain Focus 05"]
            ],
        "4_recovery": [
                ["Full Body 01", "Full Body 02"],
                ["Full Body 01", "Full Body 02"],
                ["Full Body 01", "Recovery Session B"]
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
                ["Lower Body Focus 01", "Posterior Chain Focus 02"]
                ],
        "5_recovery": [
                ["Full Body 01", "Full Body 02"],
                ["Full Body 01", "Full Body 02"],
                ["Recovery Session A", "Recovery Session B"]
                ]
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
        "Recovery Session B": ""
        }


def arguments():
    parser = ArgumentParser()
    parser.add_argument("--yoga", dest="yoga", required=True, choices=["Beginer", "Intermediate", "Advanced"])
    parser.add_argument("--strength", dest="strength", required=True, choices=["1", "2", "3", "4", "5"])
    parser.add_argument("--start", dest="start", default=str(date.today()), help="Start date in the format of yyyy-mm-dd")
    parser.add_argument("--schedule", dest="schedule", default="./Workout schedule.xlsx", help="Schedule compainion xlsx file")
    parser.add_argument("--recovery", dest="recovery", default="4,8,12", help="Weeks for recovery")
    parser.add_argument("--weeks", dest="weeks", default="12", help="Total weeks")
    return parser.parse_args()


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

def combine_yoga_strength(yoga_activities: list, strength_activities: list):
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


def main():
    yoga_progress = ("Beginer", "Intermediate", "Advanced")
    strength_progress = ("1", "2", "3", "4", "5")
    args = arguments()

    recovery_weeks = args.recovery.split(",")

    current_yoga = args.yoga
    current_yoga_index = yoga_progress.index(current_yoga)
    current_strength = args.strength
    current_strength_index = strength_progress.index(current_strength)

    total_weeks = int(args.weeks)

    current_date = date.fromisoformat(args.start)
    current_date = monday(current_date, previous=True)

    calendar = Calendar()
    event = Event()
    event.begin = str(current_date)
    event.name = f"Starting: {current_yoga} Yoga and Strength {current_strength}"
    event.make_all_day()
    calendar.events.add(event)

    week = 0
    week_activity = 0
    week_recovery = 0
    while week < total_weeks:
        if week + 1 in recovery_weeks:
            yoga_activities = YOGA[f"{current_yoga}_recovery"][week_recovery]
            strength_activities = STRENGTH[f"{current_strength}_recovery"][week_recovery]
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
                current_strength_index += 1
                if current_strength_index == len(strength_progress):
                    current_strength_index -= 1
                current_yoga_index += 1
                if current_yoga_index == len(yoga_progress):
                    current_yoga_index -= 1
                current_yoga = yoga_progress[current_yoga_index]
                current_strength = strength_progress[current_strength_index]

                event = Event()
                event.begin = str(current_date)
                event.name = f"Starting: {current_yoga} Yoga and Strength {current_strength}"
                event.make_all_day()
                calendar.events.add(event)
        current_date = monday(current_date, previous=False)
        for activity, activity_text in combine_yoga_strength(yoga_activities, strength_activities):
            event = Event()
            event.begin = str(current_date)
            event.name = activity
            event.description = activity_text
            event.make_all_day()
            calendar.events.add(event)
            current_date += relativedelta(days=1)
            if args.yoga == "Beginer" and current_date.weekday() == 2:
                current_date += relativedelta(days=1)
        week += 1
    with open('workout.ics', 'w') as workout_file:
        workout_file.writelines(calendar.serialize_iter())


if __name__ == "__main__":
    main()
