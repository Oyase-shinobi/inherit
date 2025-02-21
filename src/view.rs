use crate::messages::MyAppMessage;
use crate::pages::dashboard::dashboard;
use crate::pages::fs_plan_created_page::fs_plan_created_page;
use crate::pages::generation::generation;
use crate::pages::review_confirm_page::review_confirm_page;
use crate::pages::select_plan_page::select_plan_page;
use crate::pages::selected_setup_page::selected_setup_page;
use crate::pages::ts_plan_details_locked_page::ts_plan_details_locked_page;
use crate::state::{State, Page};

pub fn view(state: &State) -> iced::Element<MyAppMessage> {
    match state.current_page {        
        Page::FirstCreateNewPlanPage => select_plan_page(state),
        Page::SecondCreateNewPlanPage => selected_setup_page(state),
        Page::ThirdCreateNewPlanPage => review_confirm_page(state),
        Page::ForthCreateNewPlanPage => generation(),
        Page::FifthCreateNewPlanPage => fs_plan_created_page(state),
        Page::DashboardPage => dashboard(state),
        Page::TimeSafePlanDetailsLockedPage => ts_plan_details_locked_page(state)
    }
}
