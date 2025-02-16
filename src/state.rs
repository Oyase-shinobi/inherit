#[derive(Debug, PartialEq, Clone)]

pub enum Page {
    FirstCreateNewPlanPage,
    SecondCreateNewPlanPage,
    ThirdCreateNewPlanPage,
    ForthCreateNewPlanPage,
    FifthCreateNewPlanPage,
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
    pub is_loading_page: bool
}

impl Default for State {
    fn default() -> Self {
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
            is_loading_page: false
        }
    }
}