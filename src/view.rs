use crate::messages::MyAppMessage;
use crate::pages::dashboard::dashboard;
use crate::pages::fs_plan_created_page::fs_plan_created_page;
use crate::pages::generation::generation;
use crate::pages::review_confirm_page::review_confirm_page;
use crate::pages::select_plan_page::select_plan_page;
use crate::pages::selected_setup_page::selected_setup_page;
use crate::state::{State, Page};
use iced::{
    self, Color, Background, Border, Gradient, theme,
    widget::{button, Theme, text_input},
    gradient::{Linear, ColorStop},
    border::Radius,
};

pub fn view(state: &State) -> iced::Element<MyAppMessage> {
    match state.current_page {        
        Page::FirstCreateNewPlanPage => select_plan_page(state),
        Page::SecondCreateNewPlanPage => selected_setup_page(state),
        Page::ThirdCreateNewPlanPage => review_confirm_page(state),
        Page::ForthCreateNewPlanPage => generation(),
        Page::FifthCreateNewPlanPage => fs_plan_created_page(state),
        Page::DashboardPage => dashboard(state)
    }
}

struct BackButtonColor {}
impl<> button::StyleSheet for BackButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
            border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 12.0.into() },
            text_color: Color { r: 20. /255., g: 23. /255., b: 23. /255., a: 100. },
            ..style.active(&theme::Button::Primary)
        }
    }
}

struct ContinueButtonColor {}

impl<> button::StyleSheet for ContinueButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Gradient(Gradient::Linear(Linear {
                angle: 0.778.into(),
                stops: [
                    Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            r: 4.0 / 255.0,
                            g: 47.0 / 255.0,
                            b: 104.0 / 255.0,
                            a: 100.0,
                        },
                    }),
                    Some(ColorStop {
                        offset: 0.951,
                        color: Color {
                            r: 2.0 / 255.0,
                            g: 84.0 / 255.0,
                            b: 191.0 / 255.0,
                            a: 100.0,
                        },
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    // Some(ColorStop { offset: 1., color: Color { r: 2., g: 84., b: 191., a: 1. } }),
                ]
            }))),
            border: Border::with_radius(Radius::from(12.0)),
            text_color: Color { r: 1., g: 1., b: 1., a: 100. },
            ..style.active(&theme::Button::Primary)
        }
    }
}

struct CustomTextInputStyle {}

impl<> text_input::StyleSheet for CustomTextInputStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(1.0, 1.0, 1.0).into(),  
            border: Border { color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }, width: 1.0, radius: Radius::from([10.0, 10.0, 10.0, 10.0]) },  // White background when focused
            icon_color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() } // White background when focused
                   // Gray border
        }
    }

    fn focused(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(1.0, 1.0, 1.0).into(),  
            border: Border { color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }, width: 1.0, radius: Radius::from([10.0, 10.0, 10.0, 10.0]) },  // White background when focused
            icon_color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(158. /255., 168. /255., 190. /255.) // Gray placeholder text
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.2, 0.2, 0.2) // Dark text color
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.8, 0.9, 1.0) // Light blue selection color
    }

    fn disabled_color(&self, _: &<Self as iced::widget::text_input::StyleSheet>::Style) -> iced::Color {
        Color::from_rgb(0.2, 0.2, 0.2)
    }

    fn disabled(&self, _: &<Self as iced::widget::text_input::StyleSheet>::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(0.9, 0.9, 0.9).into(),
            border: Border { color: Color::from_rgba(0.7, 0.7, 0.7, 0.5), width: 1.0, radius: Radius::from([5.0, 5.0, 5.0, 5.0]) },
            icon_color: Color::from_rgba(0.7, 0.7, 0.7, 0.5),
        }
    }
}