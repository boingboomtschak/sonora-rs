use std::time::SystemTime;

// Reminder types

pub struct Reminder {
    pub time: Option<u64>,
    pub message: Option<String>,
    pub recip: Option<String>,
}

impl Reminder {
    pub fn parse_time(time: String) -> Result<u64, &'static str> {
        let mut ts : u64 = 0;
        let sys_time : u64;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => sys_time = n.as_secs(),
            Err(_) => return Err("Failed to load system time!"),
        }
        // Group chars together by groups of either numbers or valid letters
        // i.e., "4y5mo2d" becomes ["4", "y", "5", "mo", "2", "d"]
        let mut queue : Vec<char> = Vec::new();
        let mut pairs : Vec<String> = Vec::new();
        let mut num = true;
        for c in time.chars() {
            if c.is_digit(10) {
                if num {
                    queue.push(c);
                } else {
                    pairs.push(queue.iter().collect());
                    queue.clear();
                    num = true;
                    queue.push(c);
                }
            } else if c.is_alphabetic() {
                if num {
                    pairs.push(queue.iter().collect());
                    queue.clear();
                    num = false;
                    queue.push(c);
                } else {
                    queue.push(c);
                }
            } else {
                return Err("Failed to parse time string!");
            }
        }
        if !queue.is_empty() {
            pairs.push(queue.iter().collect());
            queue.clear();
        }
        
        // Match each "pair" of chars to either numeric set of digits or valid time symbol, error otherwise
        // Valid numeric digits set the current counter ("unit") equal, valid time symbols multiply by the last passed
        //   unit and add to timestamp (i.e., if last # digits was 4 and current symbols are "yr", add 4 years in seconds to ts)
        let mut unit = 0;
        for p in pairs.iter() {
            match p.as_str() {
                _ if p.chars().all(char::is_numeric) => unit = p.parse::<u64>().unwrap(),
                "yr" | "year" | "years" => { unit *= 31536000; ts += unit },
                "mo" | "month" | "months" => { unit *= 2592000; ts += unit },
                "w" | "wk" | "week" | "weeks" => { unit *= 604800; ts += unit },
                "d" | "day" | "days" => { unit *= 86400; ts += unit },
                "h" | "hr" | "hour" | "hours" => { unit *= 3600; ts += unit },
                "m" | "min" | "mins" | "minute" | "minutes" => { unit *= 60; ts += unit },
                "s" | "sec" | "secs" | "second" | "seconds" => { unit *= 1; ts += unit },
                _ => return Err("Failed to parse time string!"),
            }
        }
        return Ok(sys_time + ts);
    }
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
