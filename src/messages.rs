use crate::widgets::{calendar::CalendarMessage, filter_btn_group::Filter};

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    // GoToSelectPlanType(SelectPlanTypeMessage),
    // GoToSetupPlan(SetupPlanTypeMessage),
    TimeSafePressed,
    FailSafePressed,
    AlertCloseBtnPressed,
    GoToFirstCreateNewPlanBtnPressed,
    GoToSecondCreateNewPlanBtnPressed,
    GoToThirdCreateNewPlanBtnPressed,
    GoToForthCreateNewPlanBtnPressed,
    GoToFifthCreateNewPlanPage,
    GoToDashboardPage,
    PlanNameContentChanged(String),
    TogglerUxtos(bool),
    RecommendAlertCloseBtnPressed,
    ToggleCheckbox1(bool),
    ToggleCheckbox2(bool),
    ToggleCheckbox3(bool),
    ToggleCheckbox4(bool),
    ToggleCheckbox5(bool),
    TogglePrivacyCheckbox(bool),
    ToggleUnderstandCheckbox(bool),
    FilterSelected(Filter),
    Tick,
    CalendarMessage(usize, CalendarMessage)
}