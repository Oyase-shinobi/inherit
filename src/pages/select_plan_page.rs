use crate::create_plan::select_plan::PlanCard;
use crate::messages::MyAppMessage;
use crate::state::State;
use crate::ContinueButtonColor;
use iced::advanced::graphics::core::Element;
use iced::Renderer;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient, theme,
    widget::{container, container::Appearance, Svg, column, row, text, horizontal_rule, button, Theme, mouse_area, scrollable::{Direction, Properties}, Scrollable},
    font,
    gradient::{Linear, ColorStop},
};


pub fn select_plan_page(state: &State) -> Element<'static, MyAppMessage, Theme, Renderer> {
    let time_safe_content = container(column![
        Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
        column![
            text("Time Safe").size(16).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }),
            text("Lock Bitcoin until a future date").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
        ].spacing(6.),
        column![
            text("Protected against").size(12).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
            row![
                container(row![
                        Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("Wrench attacks").size(12)
                    ].spacing(4).align_items(Alignment::Center)
                ).style(Appearance {
                    text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                    background: Some(Background::Color(Color::from_rgba(0., 149. / 255., 42. / 255., 0.12))),
                    border: Border {
                        color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                        width: 1.0,
                        radius: 100.0.into()
                    },
                    shadow: Shadow::default()
                }).padding([4.5, 10.]),
                container(row![
                        Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("Unforeseen risks").size(12)
                    ].spacing(4).align_items(Alignment::Center)
                ).style(Appearance {
                    text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                    background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                    border: Border {
                        color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                        width: 1.0,
                        radius: 100.0.into()
                    },
                    shadow: Shadow::default()
                }).padding([4.5, 10.]),
                container(row![
                        Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("Emotional selling").size(12)
                    ].spacing(4).align_items(Alignment::Center)
                ).style(Appearance {
                    text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                    background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                    border: Border {
                        color: Color::from_rgba(0., 149., 42., 0.2),
                        width: 1.0,
                        radius: 100.0.into()
                    },
                    shadow: Shadow::default()
                }).padding([4.5, 10.]),
            ].spacing(8)
        ].spacing(8),
        horizontal_rule(0),
        column![
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Time-locked access").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Keeps your funds securely locked until a specific future date, so they can’t be accessed early")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.),
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Unbreakable protection").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Ensures your funds are protected with advanced safeguards, so only the right person can claim them at the right time")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.),
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Claimable funds after expiry").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Allows you to claim the funds after the lock period ends")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.)
        ].spacing(15)
    ].spacing(24.)).width(438.);

    let fail_safe_content = container(column![
        Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
        column![
            text("Fail Safe Recovery").size(16).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }),
            text("A dead man’s switch transferring assets on missed check-ins")
            .size(14)
            .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
        ].spacing(6.),
        column![
            text("Protected against")
            .size(12)
            .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
            row![
                container(row![
                        Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("Private key loss").size(12)
                    ].spacing(4).align_items(Alignment::Center)
                ).style(Appearance {
                    text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                    background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                    border: Border {
                        color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                        width: 1.0,
                        radius: 100.0.into()
                    },
                    shadow: Shadow::default()
                }).padding([4.5, 10.]),
                container(row![
                        Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("Unexpected life events").size(12)
                    ].spacing(4).align_items(Alignment::Center)
                ).style(Appearance {
                    text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                    background: Some(Background::Color(Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.12))),
                    border: Border {
                        color: Color::from_rgba(0. / 255., 149. / 255., 42. / 255., 0.2),
                        width: 1.0,
                        radius: 100.0.into()
                    },
                    shadow: Shadow::default()
                }).padding([4.5, 10.]),
            ].spacing(8)
        ].spacing(8),
        horizontal_rule(0),
        column![
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Backup access plan").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Ensures your BTC can be recovered through predefined steps if you lose access to your private keys")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.),
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Time-based status check-ins").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Allows your beneficiary to recover BTC if you don’t confirm your status before the check-in period ends")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.),
            row![
                Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                column![
                    text("Beneficiary designation").size(12).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }),
                    text("Allows you to assign trusted beneficiaries to claim your BTC if you miss a status check-in")
                    .size(12)
                    .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                ]
            ].spacing(8.)
        ].spacing(15)
    ].spacing(24.)).width(438.);
    let blank_content = row![""];
    let dyn_space = match state.time_safe_alert_visible {
        true => 24,
        false => 0
    };
    let time_safe_info = match state.time_safe_alert_visible {
        true => row![
            row![
                Svg::from_path("assets/create-plan/info.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                column![
                    text("Dual protection guide")
                        .size(14)
                        .font(Font {
                            weight: font::Weight::Bold,
                            ..Font::DEFAULT
                    }).style(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                    text("To use both protections, create a fail safe first, then a time safe. Fail safes guard against key loss; time safes protect against advanced\nthreats")
                        .size(12.5)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                        .font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                    })
                ].spacing(4)              
            ].spacing(16).align_items(Alignment::Start),
            mouse_area(
                Svg::from_path("assets/create-plan/cross_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
            ).on_press(MyAppMessage::AlertCloseBtnPressed)
        ].spacing(150).width(1028).padding([16, 16, 16, 35]).align_items(Alignment::Start),
        false => row!("").height(0)
    };
    container(column![
        row![
            row![
                container(row![
                    Svg::from_path("assets/create-plan/home.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    text("Dashboard").size(14),
                ].spacing(4).padding([4., 16., 4., 16.]).align_items(Alignment::Center)).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                        border: Border {
                            color: Color::from_rgba (
                                236. / 255.,
                                238. / 255.,
                                242. /255.,
                                100.
                            ),
                            width: 1.,
                            radius: 10.0.into()
                        },
                        shadow: Shadow::default()
                    }
                ),
                Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                container(row![
                    text("Create new plan").size(14),
                ].padding([4., 16., 4., 16.])).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                        border: Border {
                            color: Color::from_rgba (
                                236. / 255.,
                                238. / 255.,
                                242. /255.,
                                100.
                            ),
                            width: 1.,
                            radius: 10.0.into()
                        },
                        shadow: Shadow::default()
                    }
                )
            ].spacing(8).align_items(Alignment::Center),
            row![
                container(row![
                    row![
                        Svg::from_path("assets/create-plan/wallet.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        text("ox0...s8d").size(16)
                    ].spacing(4).align_items(Alignment::Center),
                    container(row![
                        Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        text("0.234 BTC").size(14)
                    ].spacing(2.5).padding([4., 10.0, 4., 6.0]).align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: Some(Color::from_rgb(42. / 255., 47. / 255., 53. / 255.)),
                            background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                            border: Border {
                                color: Color::from_rgba (
                                    236. / 255.,
                                    238. / 255.,
                                    242. /255.,
                                    100.
                                ),
                                width: 1.,
                                radius: 10.0.into()
                            },
                            shadow: Shadow::default()
                        }
                    ),
                    Svg::from_path("assets/create-plan/below_arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                ].spacing(8.0).padding([5.5, 8.0, 5.5, 8.0]).align_items(Alignment::Center)).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(42. / 255., 47. / 255., 53. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                        border: Border {
                            color: Color::from_rgba (
                                236. / 255.,
                                238. / 255.,
                                242. /255.,
                                100.
                            ),
                            width: 1.,
                            radius: 10.0.into()
                        },
                        shadow: Shadow::default()
                    }
                ),
                container(row![
                    Svg::from_path("assets/create-plan/bell.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                ].padding(8)).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                        border: Border {
                            color: Color::from_rgba (
                                236. / 255.,
                                238. / 255.,
                                242. /255.,
                                100.
                            ),
                            width: 1.,
                            radius: 10.0.into()
                        },
                        shadow: Shadow::default()
                    }
                ),
                container(row![
                    Svg::from_path("assets/create-plan/triple_dot.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                ].padding(8)).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                        border: Border {
                            color: Color::from_rgba (
                                236. / 255.,
                                238. / 255.,
                                242. /255.,
                                100.
                            ),
                            width: 1.,
                            radius: 10.0.into()
                        },
                        shadow: Shadow::default()
                    }
                )
            ].spacing(8).align_items(Alignment::Center)
        ].spacing(684).padding([7.0, 12.0, 7.0, 88.]).align_items(Alignment::Center) ,
        Scrollable::new(column![
            text("Create new plan").size(32).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }),
            row![
                row![
                    Svg::from_path("assets/create-plan/selected_1.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                    text("Select plan type").size(14),
                ].spacing(8),
                Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                row![
                    Svg::from_path("assets/create-plan/unselected_2.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                    text("Setup plan").size(14),
                ].spacing(8),
                Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                row![
                    Svg::from_path("assets/create-plan/unselected_3.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                    text("Review and confirm").size(14),
                ].spacing(8),
            ].spacing(16.).align_items(Alignment::Center).width(508.).height(21.),
            column![
                row![
                    PlanCard::new(state.is_selected_uxtos, blank_content, MyAppMessage::TimeSafePressed),
                ].height(0).width(0),
                row![
                    PlanCard::new(state.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
                    PlanCard::new(state.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
                ].spacing(24).align_items(Alignment::Center),
                container(time_safe_info).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                        background: Some(Background::Color(Color::from_rgb(255./ 255., 249./ 255., 223.))),
                        border: Border { color: Color::from_rgb(250. / 255., 234. / 255., 171. / 255.), width: 1., radius: 16.0.into() },
                        shadow: Shadow::default(),
                    }
                )
            ].spacing(dyn_space).align_items(Alignment::Center),
            button("Continue").style(
                theme::Button::Custom(Box::new(ContinueButtonColor {}))
                // Color::from_rgb(255., 0., 0.)
            ).padding([12., 83.]).on_press(MyAppMessage::GoToSecondCreateNewPlanBtnPressed)
        ].padding([10., 0., 0., 0.]).align_items(Alignment::Center).spacing(24)).height(920).direction(Direction::Vertical(Properties::new())),
    ].align_items(Alignment::Center)) 
.style(Appearance {
    background: Some(Background::Gradient(Gradient::Linear(Linear {
        angle: 2.6.into(),
        stops: [
            Some(ColorStop {
                offset: 0.0733,
                color: Color {
                    r: 53.0 / 255.0,
                    g: 229.0 / 255.0,
                    b: 171.0 / 255.0,
                    a: 0.15,
                },
            }),
            Some(ColorStop {
                offset: 0.5191,
                color: Color {
                    r: 62.0 / 255.0,
                    g: 112.0 / 255.0,
                    b: 253.0 / 255.0,
                    a: 0.15,
                },
            }),
            Some(ColorStop {
                offset: 0.939,
                color: Color {
                    r: 135.0 / 255.0,
                    g: 85.0 / 255.0,
                    b: 241.0 / 255.0,
                    a: 0.15,
                },
            }),
            None, None, None, None, None,
        ]
    }))),
    ..Appearance::default()
    }).width(Length::Fill).height(Length::Fill).center_x().into()
}