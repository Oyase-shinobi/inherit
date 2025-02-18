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
    pub calendars: Vec<Calendar>,
    pub filter: Filter,
    pub countdown: CountdownTimer,
    pub countdown1: CountdownTimer,
    pub countdown2: CountdownTimer,
}

impl Default for State {
    fn default() -> Self {
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
            calendars,
            filter: Filter::Owner,
            countdown: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Unlock date", "purple"),
            countdown1: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Check-in time remaining", "purple"),
            countdown2: CountdownTimer::new(SystemTime::now() + Duration::from_secs(321 * 24 * 3600 + 23 * 3600 + 12 * 60 + 9), "Grace period", "gold"),
        }
    }
}