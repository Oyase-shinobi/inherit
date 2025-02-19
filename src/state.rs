use std::time::{Duration, SystemTime};

use crate::widgets::{calendar::Calendar, countdown_timer::CountdownTimer, filter_btn_group::Filter};

#[derive(Debug, PartialEq, Clone)]

pub enum Page {
    FirstCreateNewPlanPage,
    SecondCreateNewPlanPage,
    ThirdCreateNewPlanPage,
    ForthCreateNewPlanPage,
    FifthCreateNewPlanPage,
    DashboardPage,
}

#[derive(Debug)]
pub struct State {
    pub time_safe_selected: bool,
    pub fail_safe_selected: bool,
    pub time_safe_alert_visible: bool,
    pub current_page: Page,
    pub plan_name: String,
    pub is_selected_uxtos: bool,
    pub recommend_alert_visible: bool,
    pub first_is_checked: bool,
    pub second_is_checked: bool,
    pub third_is_checked: bool,
    pub fifth_is_checked: bool,
    pub sixth_is_checked: bool,
    pub privacy_is_checked: bool,
    pub understand_is_checked: bool,
    pub is_loading_page: bool,
    pub selected_time: String,
    pub times: Vec<String>,
    pub filter: Filter,
    pub countdown: CountdownTimer,
    pub countdown1: CountdownTimer,
    pub countdown2: CountdownTimer,
    pub is_time_pick_list_visible: bool,
    pub is_date_pick_list_visible: bool,
    pub day: u8,
    pub month: u8,
    pub year: i32,
    pub timezone: String,
    pub is_timezone_pick_list_visible: bool
}


impl Default for State {
    fn default() -> Self {
        let mut times = Vec::new();
        for hour in 0..24 {
            for minute in (0..60).step_by(30) {
                let time_str = format!("{:02}:{:02} {}", hour % 12, minute, if hour < 12 { "AM" } else { "PM" });
                times.push(time_str);
            }
        }
        let now = time::OffsetDateTime::now_utc().date();
        let day = now.day();
        let month = now.month() as u8;
        let year = now.year();
        let calendar = Calendar::new();
        let mut calendars = Vec::new();
        calendars.push(calendar);
        calendars.push(calendar);
        calendars.push(calendar);
        Self {
            time_safe_selected: false,
            fail_safe_selected: false,
            time_safe_alert_visible: false,
            current_page: Page::FirstCreateNewPlanPage,
            plan_name: "".to_string(),
            is_selected_uxtos: false,
            recommend_alert_visible: true,
            first_is_checked: false,
            second_is_checked: false,
            third_is_checked: true,
            fifth_is_checked: false,
            sixth_is_checked: true,
            privacy_is_checked: false,
            understand_is_checked: false,
            is_loading_page: false,
            selected_time: "00:00 AM".to_string(),
            times,
            filter: Filter::Owner,
            countdown: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Unlock date", "purple"),
            countdown1: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Check-in time remaining", "purple"),
            countdown2: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Grace period", "gold"),
            is_time_pick_list_visible: false,
            is_date_pick_list_visible: false,
            day,
            month,
            year,
            timezone: "PTC".to_string(),
            is_timezone_pick_list_visible: true
        }
    }
}