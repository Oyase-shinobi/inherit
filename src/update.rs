use crate::messages::MyAppMessage;
use crate::state::{State, Page};

pub fn update(state: &mut State, message: MyAppMessage) {
    // self.page = match message {
    //     MyAppMessage::GoToBButtonPressed => Page::B,
    //     MyAppMessage::GoToAButtonPressed => Page::A,
    // }
    match message {
        MyAppMessage::TimeSafePressed => {
            state.time_safe_selected = !state.time_safe_selected;
            if state.time_safe_selected {
                state.time_safe_alert_visible = true;
            } else {
                state.time_safe_alert_visible = false;
            }
            state.fail_safe_selected = false;
        }
        MyAppMessage::AlertCloseBtnPressed => {
            state.time_safe_alert_visible = false;
        }
        MyAppMessage::GoToFirstCreateNewPlanBtnPressed => {
            state.current_page = Page::FirstCreateNewPlanPage;
        }
        MyAppMessage::GoToSecondCreateNewPlanBtnPressed => {
            state.current_page = Page::SecondCreateNewPlanPage;
        }
        MyAppMessage::GoToThirdCreateNewPlanBtnPressed => {
            state.current_page = Page::ThirdCreateNewPlanPage;
        }
        MyAppMessage::GoToForthCreateNewPlanBtnPressed => {
            state.current_page = Page::ForthCreateNewPlanPage;
            state.trigger_time_tick = true; // Start countdown
            state.tick_count = 0; // Reset tick counter
        }
        MyAppMessage::PlanNameContentChanged(content) => {
            state.plan_name = content;
        }
        MyAppMessage::TogglerUxtos(is_checked) => {
            state.is_selected_uxtos = is_checked;
        }
        MyAppMessage::RecommendAlertCloseBtnPressed => {
            state.recommend_alert_visible = false;
        }
        MyAppMessage::FailSafePressed => {
            state.time_safe_selected = false;
            state.time_safe_alert_visible = false;
            state.fail_safe_selected = !state.fail_safe_selected;
        }
        MyAppMessage::ToggleCheckbox1(is_checked) => {
            state.first_is_checked = is_checked
        }
        MyAppMessage::ToggleCheckbox2(is_checked) => {
            state.second_is_checked = is_checked
        }
        MyAppMessage::ToggleCheckbox3(is_checked) => {
            state.third_is_checked = is_checked
        }
        MyAppMessage::ToggleCheckbox4(is_checked) => {
            state.fifth_is_checked = is_checked
        }
        MyAppMessage::ToggleCheckbox5(is_checked) => {
            state.sixth_is_checked = is_checked
        }
        MyAppMessage::TogglePrivacyCheckbox(is_checked) => {
            state.privacy_is_checked = is_checked
        }
        MyAppMessage::ToggleUnderstandCheckbox(is_checked) => {
            state.understand_is_checked = is_checked
        }
    }
}