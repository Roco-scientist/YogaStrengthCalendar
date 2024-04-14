use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum ActivityType {
    Yoga(&'static str),
    Strength(&'static str),
    StrengthEmpty,
    Skip,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum YogaLevel {
    NotSet,
    Beginer,
    Intermediate,
    Advanced,
}

impl fmt::Display for YogaLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            YogaLevel::NotSet => "None",
            YogaLevel::Beginer => "Beginer",
            YogaLevel::Intermediate => "Intermediate",
            YogaLevel::Advanced => "Advanced",
        };
        write!(f, "{}", text)
    }
}

impl YogaLevel {
    pub fn list() -> [String; 3] {
        [
            "Beginer".to_string(),
            "Intermediate".to_string(),
            "Advanced".to_string(),
        ]
    }

    pub fn advance(&mut self) -> () {
        *self = match *self {
            YogaLevel::NotSet => YogaLevel::NotSet,
            YogaLevel::Beginer => YogaLevel::Intermediate,
            YogaLevel::Intermediate => YogaLevel::Advanced,
            YogaLevel::Advanced => YogaLevel::Advanced,
        }
    }
    pub fn activities(&self) -> [[ActivityType; 6]; 9] {
        match *self {
            YogaLevel::NotSet => [
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
            ],
            YogaLevel::Beginer => [
                [
                    ActivityType::Yoga("Total Newbie Yoga"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Basic Yoga Poses"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Super Easy Stretch Routine I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Loosen Neck and Shoulders"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Shoulder Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Shake Off the Day"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Neck and upper Back Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Recovery Booster"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Hip Flexor and Groin Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Super Easy Stretch Routine I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Foot and Ankle Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Super Easy Stretch Routine II"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Mobilize the Joints"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Lower Back Recovery II"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
            ],
            YogaLevel::Intermediate => [
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Improving Posture I"),
                    ActivityType::Yoga("Basic Yoga Poses"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Mobilize and Activate I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Super Easy Stretch I"),
                    ActivityType::Yoga("Side Bends"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Full Body Mobility"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Yoga for Travel"),
                    ActivityType::Yoga("Super Easy Stretch III"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Hamstring and Calf Flexibility"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Scapular Stability"),
                    ActivityType::Yoga("Core Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Shoulder Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Pre-Ride Warm-Up"),
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Foot and Ankle Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Loosen Neck and Shoulders"),
                    ActivityType::Yoga("Hips and Hamstrings"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Mobilize the Joints"),
                    ActivityType::Yoga("Recovery Booster"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Mobilize and Activate II"),
                    ActivityType::Yoga("Improving Posture II"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Core Strengtheners III"),
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Lower Back Recovery II"),
                ],
            ],
            YogaLevel::Advanced => [
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Improving Posture I"),
                    ActivityType::Yoga("Balance and Agility I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Super Easy Stretch Routin III"),
                ],
                [
                    ActivityType::Yoga("Mobilize and Activate I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Yoga for Travel"),
                    ActivityType::Yoga("Side Bends"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip Openers I"),
                ],
                [
                    ActivityType::Yoga("Full Body Mobility"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Shake Off the Day"),
                    ActivityType::Yoga("Scapular Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip Flexor and Groin Recovery"),
                ],
                [
                    ActivityType::Yoga("Pre-Ride Warm-Up"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Shake Off the Day"),
                    ActivityType::Yoga("Core Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hips and Hamstrings"),
                ],
                [
                    ActivityType::Yoga("Knee and Ankle Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip Openers II"),
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Side Bends"),
                ],
                [
                    ActivityType::Yoga("Foot and Ankle Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Loosen Neck and Shoulders"),
                    ActivityType::Yoga("Hips and Hamstrings"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Lower Back Recovery I"),
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Balance and Agility II"),
                    ActivityType::Yoga("Recovery Booster"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Mobilize the Joints"),
                ],
                [
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Mobilize and Activate II"),
                    ActivityType::Yoga("Improving Posture II"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Super Easy Stretch Routine III"),
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Core Strengtheners III"),
                    ActivityType::Yoga("Post-Workout Cool-Down"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Neck and upper Back Recovery"),
                ],
            ],
        }
    }
    pub fn recovery(&self) -> [[ActivityType; 6]; 3] {
        match *self {
            YogaLevel::NotSet => [
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Skip,
                ],
            ],
            YogaLevel::Beginer => [
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Basic Yoga Poses"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Lower Back Recovery I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Thoracic Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
                [
                    ActivityType::Yoga("Morning Yoga Routine"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                    ActivityType::Yoga("Recovery Booster"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
            ],
            YogaLevel::Intermediate => [
                [
                    ActivityType::Yoga("Upper Body Strength"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip Openers I"),
                    ActivityType::Yoga("Core Strengtheners I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Shake Off the Day"),
                ],
                [
                    ActivityType::Yoga("Lower Back Recovery I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Core Strengtheners II"),
                    ActivityType::Yoga("Hip and Pelvis Stability"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Thoracic Recovery"),
                ],
                [
                    ActivityType::Yoga("Hip Flexor and Groin Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Recovery Booster"),
                    ActivityType::Yoga("Neck and upper Back Recovery"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Skip,
                ],
            ],
            YogaLevel::Advanced => [
                [
                    ActivityType::Yoga("Core Strengtheners I"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hamstring and Calf Flexibility"),
                    ActivityType::Yoga("Back Strengtheners"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Shoulder Stability"),
                ],
                [
                    ActivityType::Yoga("Upper Body Strength"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip and Pelvis Stability"),
                    ActivityType::Yoga("Core Strengtheners III"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Thoracic Recovery"),
                ],
                [
                    ActivityType::Yoga("Lower Back Recovery II"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Hip Flexor and Groin Recovery"),
                    ActivityType::Yoga("Shake Off the Day"),
                    ActivityType::StrengthEmpty,
                    ActivityType::Yoga("Recovery Booster"),
                ],
            ],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StrengthLevel {
    NotSet,
    Strength1,
    Strength2,
    Strength3,
    Strength4,
    Strength5,
}

impl fmt::Display for StrengthLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            StrengthLevel::NotSet => "None",
            StrengthLevel::Strength1 => "1",
            StrengthLevel::Strength2 => "2",
            StrengthLevel::Strength3 => "3",
            StrengthLevel::Strength4 => "4",
            StrengthLevel::Strength5 => "5",
        };
        write!(f, "{}", text)
    }
}

impl StrengthLevel {
    pub fn advance(&mut self) -> () {
        *self = match *self {
            StrengthLevel::NotSet => StrengthLevel::NotSet,
            StrengthLevel::Strength1 => StrengthLevel::Strength2,
            StrengthLevel::Strength2 => StrengthLevel::Strength3,
            StrengthLevel::Strength3 => StrengthLevel::Strength4,
            StrengthLevel::Strength4 => StrengthLevel::Strength5,
            StrengthLevel::Strength5 => StrengthLevel::Strength5,
        }
    }

    pub fn activities(&self) -> [[ActivityType; 2]; 9] {
        match *self {
            StrengthLevel::NotSet => [
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
            ],
            StrengthLevel::Strength1 => [
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 03"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 03"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 04"),
                    ActivityType::Strength("Full Body 03"),
                ],
                [
                    ActivityType::Strength("Full Body 04"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 04"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 05"),
                    ActivityType::Strength("Full Body 06"),
                ],
                [
                    ActivityType::Strength("Full Body 06"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
            ],
            StrengthLevel::Strength2 => [
                [
                    ActivityType::Strength("Full Body 05"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 05"),
                    ActivityType::Strength("Full Body 06"),
                ],
                [
                    ActivityType::Strength("Full Body 07"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 06"),
                    ActivityType::Strength("Full Body 07"),
                ],
                [
                    ActivityType::Strength("Full Body 08"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 07"),
                    ActivityType::Strength("Posterior Chain Focus 02"),
                ],
                [
                    ActivityType::Strength("Full Body 07"),
                    ActivityType::Strength("Full Body 08"),
                ],
                [
                    ActivityType::Strength("Core Focus 01"),
                    ActivityType::Strength("Full Body 09"),
                ],
                [
                    ActivityType::Strength("Full Body 08"),
                    ActivityType::Strength("Posterior Chain Focus 02"),
                ],
            ],
            StrengthLevel::Strength3 => [
                [
                    ActivityType::Strength("Full Body 09"),
                    ActivityType::Strength("Posterior Chain Focus 01"),
                ],
                [
                    ActivityType::Strength("Core Focus 01"),
                    ActivityType::Strength("Full Body 09"),
                ],
                [
                    ActivityType::Strength("Full Body 10"),
                    ActivityType::Strength("Posterior Chain Focus 02"),
                ],
                [
                    ActivityType::Strength("Full Body 11"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
                [
                    ActivityType::Strength("Core Focus 02"),
                    ActivityType::Strength("Lower Body Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 11"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
                [
                    ActivityType::Strength("Full Body 11"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
                [
                    ActivityType::Strength("Core Focus 02"),
                    ActivityType::Strength("Lower Body Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 12"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
            ],
            StrengthLevel::Strength4 => [
                [
                    ActivityType::Strength("Full Body 12"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
                [
                    ActivityType::Strength("Core Focus 02"),
                    ActivityType::Strength("Lower Body Focus 01"),
                ],
                [
                    ActivityType::Strength("Full Body 13"),
                    ActivityType::Strength("Posterior Chain Focus 03"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 02"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Core Focus 03"),
                    ActivityType::Strength("Full Body 15"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 02"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 03"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Core Focus 04"),
                    ActivityType::Strength("Full Body 16"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 03"),
                    ActivityType::Strength("Posterior Chain Focus 05"),
                ],
            ],
            StrengthLevel::Strength5 => [
                [
                    ActivityType::Strength("Full Body 15"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Core Focus 03"),
                    ActivityType::Strength("Lower Body Focus 03"),
                ],
                [
                    ActivityType::Strength("Full Body 16"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 03"),
                    ActivityType::Strength("Posterior Chain Focus 04"),
                ],
                [
                    ActivityType::Strength("Core Focus 04"),
                    ActivityType::Strength("Full Body 17"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 04"),
                    ActivityType::Strength("Posterior Chain Focus 05"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 04"),
                    ActivityType::Strength("Posterior Chain Focus 05"),
                ],
                [
                    ActivityType::Strength("Core Focus 02"),
                    ActivityType::Strength("Full Body 10"),
                ],
                [
                    ActivityType::Strength("Lower Body Focus 01"),
                    ActivityType::Strength("Posterior Chain Focus 02"),
                ],
            ],
        }
    }
    pub fn recovery(&self) -> [[ActivityType; 2]; 3] {
        match *self {
            StrengthLevel::NotSet => [
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
                [ActivityType::Skip, ActivityType::Skip],
            ],
            StrengthLevel::Strength1 => [
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
                [
                    ActivityType::Strength("Recovery Session B"),
                    ActivityType::Strength("Recovery Session A"),
                ],
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
            ],
            StrengthLevel::Strength2 => [
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
            ],
            StrengthLevel::Strength3 => [
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Recovery Session B"),
                ],
            ],
            StrengthLevel::Strength4 => [
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Recovery Session B"),
                ],
            ],
            StrengthLevel::Strength5 => [
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Full Body 01"),
                    ActivityType::Strength("Full Body 02"),
                ],
                [
                    ActivityType::Strength("Recovery Session A"),
                    ActivityType::Strength("Recovery Session B"),
                ],
            ],
        }
    }
}

// STRENGTH_ANNOTATION = {
//     "Core Focus 01": "",
//     "Core Focus 02": "",
//     "Core Focus 03": "",
//     "Core Focus 04": "Exercise band",
//     "Full Body 01": "Empty drink bottle",
//     "Full Body 02": "Empty drink bottle",
//     "Full Body 05": "Filled drink bottle",
//     "Full Body 06": "2 Filled drink bottles",
//     "Full Body 07": "2 Filled drink bottles",
//     "Full Body 08": "2 Filled drink bottles",
//     "Full Body 09": "2 Filled drink bottles",
//     "Full Body 10": "2 Filled drink bottles",
//     "Full Body 11": "2 Filled drink bottles",
//     "Full Body 12": "2 Filled drink bottles",
//     "Full Body 13": "2 Filled drink bottles",
//     "Full Body 15": "2 Filled drink bottles",
//     "Full Body 16": "2 Filled drink bottles",
//     "Full Body 17": "2 Filled drink bottles",
//     "Lower Body Focus 01": "Chair",
//     "Lower Body Focus 02": "Chair",
//     "Lower Body Focus 03": "Filled drink bottle",
//     "Lower Body Focus 04": "Chair, drink bottle, exercise band",
//     "Posterior Chain Focus 01": "",
//     "Posterior Chain Focus 02": "",
//     "Posterior Chain Focus 03": "Stick, filled drink bottle",
//     "Posterior Chain Focus 04": "Exercise band, filled drink bottle",
//     "Posterior Chain Focus 05": "Exercise band",
//     "Recovery Session A": "Empty drink bottle",
//     "Recovery Session B": "",
// }
//
pub enum WeekType {
    Active,
    Recovery,
}

#[derive(Copy, Clone)]
pub struct WeeklyActivities {
    pub yoga_level: YogaLevel,
    pub progress_yoga: bool,
    pub strength_level: StrengthLevel,
    pub progress_strength: bool,
    yoga_activities: [[ActivityType; 6]; 9],
    yoga_recovery: [[ActivityType; 6]; 3],
    strength_activities: [[ActivityType; 2]; 9],
    strength_recovery: [[ActivityType; 2]; 3],
    week_index: usize,
    recovery_index: usize,
}

impl Default for WeeklyActivities {
    fn default() -> WeeklyActivities {
        WeeklyActivities {
            yoga_level: YogaLevel::NotSet,
            progress_yoga: true,
            strength_level: StrengthLevel::NotSet,
            progress_strength: true,
            yoga_activities: YogaLevel::NotSet.activities(),
            yoga_recovery: YogaLevel::NotSet.recovery(),
            strength_activities: StrengthLevel::NotSet.activities(),
            strength_recovery: StrengthLevel::NotSet.recovery(),
            week_index: 0,
            recovery_index: 0,
        }
    }
}

impl WeeklyActivities {
    pub fn begining(self) -> bool {
        self.week_index == 0
    }

    pub fn begining_description(self) -> String {
        let yoga_text = match self.yoga_level {
            YogaLevel::NotSet => "",
            YogaLevel::Beginer => "Beginer yoga and ",
            YogaLevel::Intermediate => "Intermediate yoga and ",
            YogaLevel::Advanced => "Advanced yoga and ",
        };

        let strength_text = match self.strength_level {
            StrengthLevel::NotSet => "",
            StrengthLevel::Strength1 => "strength level 1",
            StrengthLevel::Strength2 => "strength level 2",
            StrengthLevel::Strength3 => "strength level 3",
            StrengthLevel::Strength4 => "strength level 4",
            StrengthLevel::Strength5 => "strength level 5",
        };

        format!("Starting: {}{}", yoga_text, strength_text)
    }

    pub fn update_activities(&mut self) -> () {
        self.yoga_activities = self.yoga_level.activities();
        self.yoga_recovery = self.yoga_level.recovery();
        self.strength_activities = self.strength_level.activities();
        self.strength_recovery = self.strength_level.recovery();
    }

    pub fn next_week_activities(&mut self, week_type: WeekType) -> Vec<ActivityType> {
        let week_activities;
        match week_type {
            WeekType::Active => {
                week_activities = self.combine_activities(
                    &self.yoga_activities[self.week_index],
                    &self.strength_activities[self.week_index],
                );
                self.advance_week_index()
            }
            WeekType::Recovery => {
                week_activities = self.combine_activities(
                    &self.yoga_recovery[self.recovery_index],
                    &self.strength_recovery[self.recovery_index],
                );
                self.advance_recovery_index()
            }
        }
        week_activities
    }

    fn advance_week_index(&mut self) -> () {
        self.week_index += 1;
        if self.week_index == 9 {
            self.week_index = 0;
            if self.progress_yoga {
                self.yoga_level.advance();
            }
            if self.progress_strength {
                self.strength_level.advance();
            }
            if self.progress_yoga || self.progress_strength {
                self.update_activities();
            }
        }
    }

    fn advance_recovery_index(&mut self) -> () {
        self.recovery_index += 1;
        if self.recovery_index == 3 {
            self.recovery_index = 0;
        }
    }

    fn combine_activities(
        self,
        yoga_week: &[ActivityType; 6],
        strength_week: &[ActivityType; 2],
    ) -> Vec<ActivityType> {
        let mut weeks_activities: Vec<ActivityType> = Vec::new();
        let mut strength_index = 0;
        for activity in yoga_week {
            match activity {
                ActivityType::StrengthEmpty => {
                    weeks_activities.push(strength_week[strength_index]);
                    strength_index += 1;
                }
                _ => weeks_activities.push(activity.clone()),
            }
        }
        weeks_activities
    }
}
