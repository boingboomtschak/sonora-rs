
// Reminder types

pub struct Reminder {
    pub time: String,
    pub message: String,
}


// everything below here is overengineering and is only here so I don't forget rust syntax
/* 
pub enum ReminderReturn {
    User,
    Channel
}

pub struct ReminderDelay {
    years: u32,
    months: u32,
    weeks: u32,
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

pub struct Reminder {
    message: String,
    return_type: ReminderReturn,
    return_id: String,
    expires: ReminderDelay,
}

// Reminder method implementations

impl ReminderDelay {
    fn to_milliseconds(&self) -> u64 {
        return u64::from(self.years) * 31536000000 
            + u64::from(self.months) * 2592000000 
            + u64::from(self.weeks) * 604800000 
            + u64::from(self.days) * 86400000 
            + u64::from(self.hours) * 3600000 
            + u64::from(self.minutes) * 60000 
            + u64::from(self.seconds) * 1000;
    }
    fn to_string(&self) -> String {
        return format!("{}y{}mo{}wk{}d{}h{}m{}s", 
            self.years, 
            self.months, 
            self.weeks, 
            self.days, 
            self.hours, 
            self.minutes, 
            self.seconds
        );
    }
}


impl Reminder {
    
} */
