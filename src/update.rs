use crate::messages::MyAppMessage;
use crate::state::{State, Page};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

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
            state.is_loading_page = true;
        }
        MyAppMessage::GoToFifthCreateNewPlanPage => {
            state.current_page = Page::FifthCreateNewPlanPage;
            state.is_loading_page = false;
        }
        MyAppMessage::GoToDashboardPage => {
            state.current_page = Page::DashboardPage;
        }
        MyAppMessage::PlanNameContentChanged(content) => {
            state.plan_name = content;
        }
        MyAppMessage::BeneficiaryNameContentChanged(content) => {
            state.beneficiary_name = content;
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
        MyAppMessage::TimeSelected(time) => {
            state.selected_time = time
        }
        MyAppMessage::TimeZoneSelected(timezone) => {
            state.timezone = timezone
        }
        MyAppMessage::FilterSelected(filter) => state.filter = filter,
        MyAppMessage::Tick => {
            state.countdown.update();
            state.countdown1.update();
            state.countdown2.update();
        }
        MyAppMessage::TimePickListPressed => {
            state.is_time_pick_list_visible = !state.is_time_pick_list_visible
        }
        MyAppMessage::DatePickListPressed => {
            state.is_date_pick_list_visible = !state.is_date_pick_list_visible
        }
        MyAppMessage::TimeZonePickListPressed => {
            state.is_timezone_pick_list_visible = !state.is_timezone_pick_list_visible
        }
        MyAppMessage::PreviousMonth => {
            if state.month <= 1 {
                state.month = 12;
                state.year -= 1;
            } else {
                state.month -= 1
            }
        }
        MyAppMessage::NextMonth => {
            if state.month >= 12 {
                state.month = 1;
                state.year += 1;
            } else {
                state.month += 1;
            }
        }
        MyAppMessage::SelectDay(day) => {
            state.day = day
        }
        MyAppMessage::SetLockBtcAmount(amount) => {
            state.lock_btc_amount = amount
        }
        MyAppMessage::TxLinkCopyBtnPressed(tx_link) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            println!("{:?}", ctx.get_contents());
            ctx.set_contents(tx_link).unwrap();
            state.is_tx_link_copied = !state.is_tx_link_copied
        }
    }
}