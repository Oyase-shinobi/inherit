use crate::widgets::calendar::CalendarMessage;

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
    CalendarMessage(usize, CalendarMessage)
}