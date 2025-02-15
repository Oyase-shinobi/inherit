use crate::messages::MyAppMessage;
use crate::state::{State, Page};
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient, theme,
    widget::{container, container::Appearance, Svg, column, row, text, horizontal_rule, button, Theme, mouse_area, text_input, tooltip, checkbox, scrollable::{Direction, Properties}, Scrollable, toggler},
    font,
    alignment::Vertical,
    gradient::{Linear, ColorStop},
    border::Radius,
};

use crate::create_plan::select_plan::PlanCard;

pub fn view(state: &State) -> iced::Element<MyAppMessage> {
        
    // .into()
    // PlanCard::new().into()
    // container(PlanCard::new()).into()
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
                    text("To use both protections, create a fail safe first, then a time safe. Fail safes guard against key loss; time safes protect against advanced threats")
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
    let recommend_alert = match state.recommend_alert_visible {
        true => row![
            row![
                Svg::from_path("assets/create-plan/info.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                text("We recommend keeping your time lock to 3 years or less, as changes in Bitcoin's network, technological advancements, and unforeseen risks could impact long-term security.")
                    .size(14)
                    .width(500)
                    .style(Color::from_rgb(42. / 255., 47. / 255., 53. / 255.))
                    .font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                })        
            ].spacing(16).align_items(Alignment::Start),
            mouse_area(
                Svg::from_path("assets/create-plan/cross_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
            ).on_press(MyAppMessage::RecommendAlertCloseBtnPressed)
        ].spacing(150).width(1028).padding([16, 16, 16, 35]).align_items(Alignment::Start),
        false => row!("").height(0)
    };
    
    let select_uxtos_component = match state.is_selected_uxtos {
        false => column![
            column![
                text("Address").size(14).font(Font {
                    weight: font::Weight::Medium,
                    ..Font::DEFAULT
                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                container(text("default.connected@address.com").size(14).font(Font {
                    weight: font::Weight::Medium,
                    ..Font::DEFAULT
                }).width(469)).padding([10.0, 12.0]).style(
                    Appearance {
                        text_color: Some(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                        background: Some(Background::Color(Color::from_rgb(236./ 255., 238./ 255., 242. /255.))),
                        border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 10.0.into() },
                        shadow: Shadow::default(),
                    }
                )
            ].spacing(2).align_items(Alignment::Start),
            column![
                row![
                    Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                    text("Amount of BTC").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                    tooltip(
                        Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/tooltip_polygon.svg").width(Length::Fixed(9.)).height(Length::Fixed(21.)),
                            container(text("Inherit will automatically use the oldest UTXOs first when a fixed amount is entered. To customize UTXOs, you can enable the toggle.").line_height(1.5).size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).width(400).height(63)).padding([12.0, 16.0]).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(1., 1., 1.)),
                                background: Some(Background::Color(Color::from_rgb(0., 62. /255., 144. /255.))),
                                border: Border { color: Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 0.0), width: 0., radius: 16.0.into() },
                                shadow: Shadow::default(),
                            }
                        )
                        ].spacing(0).align_items(Alignment::Center),
                        tooltip::Position::Right,
                    )
                ].spacing(4).align_items(Alignment::Center),
                row![
                    container(text("3.926").size(32).font(Font {
                        weight: font::Weight::ExtraBold,
                        ..Font::DEFAULT
                    }).width(227).vertical_alignment(Vertical::Center).horizontal_alignment(iced::alignment::Horizontal::Center)).padding([18.0, 8.0]).style(
                        Appearance {
                            text_color: Some(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                            background: None,
                            border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 6.0.into() },
                            shadow: Shadow::default(),
                        }
                    ),
                    text("= $280,888.80").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(138. /255., 146. /255., 165. /255.))
                ].align_items(Alignment::Center).spacing(8),
                row![
                    text("Available: 4.21 BTC").size(14).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(138. /255., 146. /255., 165. /255.)),
                    text("Add max").size(14).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(2. /255., 84. /255., 191. /255.))
                ].align_items(Alignment::Center).spacing(8)
            ].align_items(Alignment::Start).spacing(2)
        ].padding([20.0, 24.0]).spacing(16).width(760).align_items(Alignment::Start),
        true =>
            column![
                column![
                    text("Name (optional)").size(14).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                    text_input("Enter beneficiary", &state.plan_name).style(
                        theme::TextInput::Custom(Box::new(CustomTextInputStyle {}))
                    ).padding([10.0, 12.0]).width(227).on_input(MyAppMessage::PlanNameContentChanged)
                ].spacing(2).align_items(Alignment::Start),  

            container(column![
                row![
                    container(
                        Svg::from_path("assets/create-plan/check-box.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    ).padding([14., 16.]),
                    text("Amount BTC").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(166),
                    text("Address").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(166),
                    row![
                        text("Confirmations").size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                        Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    ].spacing(4).align_items(Alignment::Center).width(166),
                    text("Status").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(166),
                    
                ].align_items(Alignment::Center),
                Scrollable::new(column![
                    container(row![
                    container(
                        checkbox("", state.first_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleCheckbox1),
                    ).padding([14., 16.]),
                    text("0.015").size(14).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                    row![
                        text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                    Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                    ].spacing(8).align_items(Alignment::Center).width(166),

                    text("15").size(14).line_height(1.5).font(Font {
                        weight: font::Weight::Medium,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                    container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(166),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(
                            checkbox("", state.second_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleCheckbox2),
                        ).padding([14., 16.]),
                        text("0.015").size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(166),

                        text("15").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(166),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(
                            checkbox("", state.third_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleCheckbox3),
                        ).padding([14., 16.]),
                        text("0.015").size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(166),

                        text("15").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(166),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(
                            checkbox("", state.fifth_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleCheckbox4),
                        ).padding([14., 16.]),
                        text("0.015").size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(166),

                        text("15").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(166),
                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    row![
                        container(
                            checkbox("", state.sixth_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleCheckbox5),
                        ).padding([14., 16.]),
                        text("0.015").size(14).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        row![
                            text("as9sk0...wi9dso").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(166),

                        text("15").size(14).line_height(1.5).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(166),
                        container(Svg::from_path("assets/create-plan/locked_status.svg").width(Length::Fixed(61.)).height(Length::Fixed(25.))).width(166),
                    ].align_items(Alignment::Center)
                ].align_items(Alignment::Center)).height(176).direction(Direction::Vertical(Properties::new().scroller_width(4).width(0)))
            ].width(Length::Fill)).height(Length::Shrink).style(
                Appearance {
                    text_color: Some(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                    background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255. /255.))),
                    border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 16.0.into() },
                    shadow: Shadow::default(),
                }
            ),
            row![
                row![
                    text("1 UTXO").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                    text("selected").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.))
                ].spacing(4).align_items(Alignment::Center),
                row![
                    text("Total BTC to lock").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                    text("0.015 BTC").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                ].spacing(4).align_items(Alignment::Center),
                row![
                    text("Time lock duration:").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                    text("6 months").size(16).font(Font {
                        weight: font::Weight::Semibold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                ].spacing(4).align_items(Alignment::Center),
                
            ].align_items(Alignment::Center).spacing(20)
        ].padding([20.0, 24.0]).spacing(16).width(760).align_items(Alignment::Start)
    };
    match state.current_page {        
        Page::FirstCreateNewPlanPage => container(column![
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
        }).width(Length::Fill).height(Length::Fill).center_x().into(),
        Page::SecondCreateNewPlanPage => container(
            column![
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
                            Svg::from_path("assets/create-plan/selected_check.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Select plan type").size(14),
                        ].spacing(8),
                        Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/selected_2.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Setup plan").size(14),
                        ].spacing(8),
                        Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/unselected_3.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Review and confirm").size(14),
                        ].spacing(8),
                    ].spacing(16.).align_items(Alignment::Center).width(508.).height(21.),
                    column![
                        container(
                            column![
                                text("PLAN DETAILS").size(14).font(Font {
                                    weight: font::Weight::Semibold,
                                    ..Font::DEFAULT
                                }),
                                row![
                                    text("Chosen plan type").size(16).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(0., 0., 0.)),
                                    text("Time Safe").size(16).font(Font {
                                        weight: font::Weight::Semibold,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(0., 0., 0.)),
                                ].spacing(560),
                                column![
                                    text("Plan name (optional)").size(14).font(Font {
                                        weight: font::Weight::Semibold,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                    text_input("Enter plan’s name", &state.plan_name).style(
                                        theme::TextInput::Custom(Box::new(CustomTextInputStyle {}))
                                    ).padding([10.0, 12.0]).on_input(MyAppMessage::PlanNameContentChanged)
                                ].spacing(2).align_items(Alignment::Start)
                            ].spacing(24).padding(40).width(840).height(233)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        container(
                            column![
                                row![
                                    text("BENEFICIARY").size(14).font(Font {
                                        weight: font::Weight::Semibold,
                                        ..Font::DEFAULT
                                    }),
                                    row![
                                        text("Select UXTOs").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                        toggler(None, state.is_selected_uxtos, MyAppMessage::TogglerUxtos).size(24)
                                    ].spacing(12).align_items(Alignment::Center)
                                ].spacing(512).align_items(Alignment::Center),
                                match state.is_selected_uxtos {
                                    true => column![
                                        text("Select UTXOs to Lock").size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).line_height(1.05),
                                        text("Choose specific Bitcoin UTXOs you want to lock for recovery. Once locked, these funds will be inaccessible until the recovery conditions are met. Multiple UTXOs will be grouped into a single time-lock transaction.").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).line_height(1.5),
                                        tooltip(
                                            text("What is a UTXO?").size(16).font(Font {
                                                weight: font::Weight::Bold,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(2. /255., 84. /255., 191. /255.)).line_height(1.5),
                                            row![
                                                Svg::from_path("assets/create-plan/tooltip_polygon.svg").width(Length::Fixed(9.)).height(Length::Fixed(21.)),
                                                container(text("A UTXO (Unspent Transaction Output) represents a specific amount of Bitcoin available for spending. By selecting specific UTXOs, you can control exactly which funds are locked.").line_height(1.5).size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).width(476).height(63)).padding([12.0, 16.0]).style(
                                                Appearance {
                                                    text_color: Some(Color::from_rgb(1., 1., 1.)),
                                                    background: Some(Background::Color(Color::from_rgb(0., 62. /255., 144. /255.))),
                                                    border: Border { color: Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 0.0), width: 0., radius: 16.0.into() },
                                                    shadow: Shadow::default(),
                                                }
                                            )
                                            ].spacing(0).align_items(Alignment::Center),
                                            tooltip::Position::Right,
                                        )
                                        
                                    ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                                    false => column![""].height(0)
                                },
                                
                                column![
                                    match state.is_selected_uxtos {
                                        true => text("Beneficiary 1").size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)),
                                        false => text("").height(0)
                                    },    
                                container(
                                    select_uxtos_component
                                ).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                                        background: None,
                                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                        shadow: Shadow::default(),
                                    }
                                )].spacing(
                                    match state.is_selected_uxtos {
                                        true => 8,
                                        false => 0
                                    }
                                ).align_items(Alignment::Start),
                            ].spacing(20).padding(40).width(840).height(Length::Shrink).align_items(Alignment::Start)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        container(
                            column![
                                text("UNLOCK DATE").size(14).font(Font {
                                    weight: font::Weight::Semibold,
                                    ..Font::DEFAULT
                                }),

                                text("Select the date, time, and timezone when your Bitcoin will be unlocked").size(16).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),

                                row![

                                ].spacing(16).align_items(Alignment::Center),
                                container(recommend_alert).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                        background: Some(Background::Color(Color::from_rgb(227./ 255., 239./ 255., 1.))),
                                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                        shadow: Shadow::default(),
                                    }
                                )
                            ].spacing(24).padding(40).width(840).height(Length::Shrink)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        
                    ].spacing(16).align_items(Alignment::Center),
                    row![
                        container(button("<- Back").style(
                            theme::Button::Custom(Box::new(BackButtonColor {}))
                            // Color::from_rgb(255., 0., 0.)
                        ).padding([12., 20.]).on_press(MyAppMessage::GoToFirstCreateNewPlanBtnPressed)).width(Length::Fill),
                        button("Continue").style(
                            theme::Button::Custom(Box::new(ContinueButtonColor {}))
                            // Color::from_rgb(255., 0., 0.)
                        ).padding([12., 20.]).on_press(MyAppMessage::GoToThirdCreateNewPlanBtnPressed)
                    ].width(840),
                    
                ].padding([10., 0., 0., 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0))),
            ].align_items(Alignment::Center)
        ).style(Appearance {
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
        }).width(Length::Fill).height(Length::Fill).center_x().into(),
        Page::ThirdCreateNewPlanPage => container(
            column![
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
                            Svg::from_path("assets/create-plan/selected_check.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Setup plan").size(14),
                        ].spacing(8),
                        Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/selected_2.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Review and confirm").size(14),
                        ].spacing(8),
                    ].spacing(16.).align_items(Alignment::Center).height(21.),
                    column![
                        container(
                            column![
                                text("INHERIT PLAN SUMMARY").size(14).font(Font {
                                    weight: font::Weight::Semibold,
                                    ..Font::DEFAULT
                                }),
                                column![
                                    text("Time Safe plan").size(14).font(Font {
                                        weight: font::Weight::Semibold,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                    text(state.plan_name.clone() + "'s plan").size(24).font(Font {
                                        weight: font::Weight::ExtraBold,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)).line_height(1.2),
                                    row![
                                        text("Address: jbkdsvdg9...hfebrc49ejcin").size(14).font(Font {
                                            weight: font::Weight::Semibold,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                    ].spacing(4).align_items(Alignment::Center)
                                ].spacing(4).align_items(Alignment::Start),
                                container(
                                    column![
                                        row![
                                        column![
                                            text("Beneficiary wallet").size(16).font(Font {
                                                weight: font::Weight::ExtraBold,
                                                ..Font::DEFAULT
                                            }).line_height(1.2),
                                            row![
                                                text("Address: jbkdsvdg9...hfebrc49ejcin").size(14).font(Font {
                                                    weight: font::Weight::Semibold,
                                                    ..Font::DEFAULT
                                                }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                            ].spacing(4).align_items(Alignment::Center)
                                        ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                                        container(row![
                                            Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                            text("0.234 BTC").size(16)
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
                                                    radius: 100.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ),
                                        ].spacing(16).align_items(Alignment::Center).width(Length::Fill),
                                        match state.is_selected_uxtos {
                                            true => {
                                                column![
                                                    container(
                                                        row![""]
                                                    ).style(
                                                        Appearance {
                                                            text_color: None,
                                                            background: None,
                                                            border: Border {
                                                                color: Color::from_rgba (
                                                                    236. / 255.,
                                                                    238. / 255.,
                                                                    242. /255.,
                                                                    100.
                                                                ),
                                                                width: 1.,
                                                                radius: 0.0.into()
                                                            },
                                                            shadow: Shadow::default()
                                                        }
                                                    ).width(Length::Fill).height(1),
                                                    text("Locked UTXOs").size(14).font(Font {
                                                        weight: font::Weight::Semibold,
                                                        ..Font::DEFAULT
                                                    }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).line_height(1.5),
                                                    container(column![
                                                        row![
                                                            container(text("Amount BTC").size(14).font(Font {
                                                                weight: font::Weight::Medium,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            text("Address").size(14).font(Font {
                                                                weight: font::Weight::Medium,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                                                            row![
                                                                text("Confirmations").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                                Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                                            ].spacing(4).align_items(Alignment::Center).width(178),
                                                            text("Status").size(14).font(Font {
                                                                weight: font::Weight::Medium,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                                                            
                                                        ].align_items(Alignment::Center).height(44),
                                                        Scrollable::new(column![
                                                            container(row![

                                                                container(text("0.015").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            row![
                                                                text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                                                                weight: font::Weight::Medium,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                            ].spacing(8).align_items(Alignment::Center).width(178),
                                    
                                                            text("15").size(14).line_height(1.5).font(Font {
                                                                weight: font::Weight::Medium,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                                                            container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(178),
                                    
                                                            ].align_items(Alignment::Center).height(44)).style(
                                                                Appearance {
                                                                    text_color: None,
                                                                    background: None,
                                                                    border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                                                                    shadow: Shadow::default(), 
                                                                }
                                                            ),
                                                            container(row![

                                                                container(text("0.015").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                                row![
                                                                    text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                                ].spacing(8).align_items(Alignment::Center).width(178),
                                    
                                                                text("15").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                                                                container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(178),
                                    
                                                            ].align_items(Alignment::Center).height(44)).style(
                                                                Appearance {
                                                                    text_color: None,
                                                                    background: None,
                                                                    border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                                                                    shadow: Shadow::default(), 
                                                                }
                                                            ),
                                                            container(row![

                                                                container(text("0.015").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                                row![
                                                                    text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                                ].spacing(8).align_items(Alignment::Center).width(178),
                                    
                                                                text("15").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                                                                container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(178),
                                    
                                                            ].align_items(Alignment::Center).height(44)).style(
                                                                Appearance {
                                                                    text_color: None,
                                                                    background: None,
                                                                    border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                                                                    shadow: Shadow::default(), 
                                                                }
                                                            ),
                                                            container(row![

                                                                container(text("0.015").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                                row![
                                                                    text("wehht6...dgfzdc").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                                ].spacing(8).align_items(Alignment::Center).width(178),
                                    
                                                                text("15").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                                                                container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(178),
                                                            ].align_items(Alignment::Center).height(44)).style(
                                                                Appearance {
                                                                    text_color: None,
                                                                    background: None,
                                                                    border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                                                                    shadow: Shadow::default(), 
                                                                }
                                                            ),
                                                            row![

                                                                container(text("0.015").size(14).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                                row![
                                                                    text("as9sk0...wi9dso").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                                ].spacing(8).align_items(Alignment::Center).width(178),
                                    
                                                                text("15").size(14).line_height(1.5).font(Font {
                                                                    weight: font::Weight::Medium,
                                                                    ..Font::DEFAULT
                                                                }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                                                                container(Svg::from_path("assets/create-plan/locked_status.svg").width(Length::Fixed(61.)).height(Length::Fixed(25.))).width(178),
                                                            ].align_items(Alignment::Center).height(44)
                                                        ].align_items(Alignment::Center)).height(176).direction(Direction::Vertical(Properties::new().scroller_width(4).width(0)))
                                                    ].width(Length::Fill)).height(Length::Shrink).style(
                                                        Appearance {
                                                            text_color: Some(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                            background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255. /255.))),
                                                            border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 16.0.into() },
                                                            shadow: Shadow::default(),
                                                        }
                                                    ),
                                                    row![
                                                        row![
                                                            text("1 UTXO").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                                            text("selected").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.))
                                                        ].spacing(4).align_items(Alignment::Center),
                                                        row![
                                                            text("Total BTC to lock").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                            text("0.015 BTC").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                                        ].spacing(4).align_items(Alignment::Center),
                                                        row![
                                                            text("Time lock duration:").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                            text("6 months").size(16).font(Font {
                                                                weight: font::Weight::Semibold,
                                                                ..Font::DEFAULT
                                                            }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                                        ].spacing(4).align_items(Alignment::Center),
                                                        
                                                    ].align_items(Alignment::Center).spacing(20)
                                                ].width(Length::Fill).height(Length::Shrink).spacing(12).align_items(Alignment::Start)
                                            },
                                            false => {
                                                column![""].height(0)
                                            }
                                        }
                                    ].width(Length::Fill).height(Length::Shrink).spacing(
                                        match state.is_selected_uxtos {
                                            true => 12,
                                            false => 0
                                        }
                                    ).align_items(Alignment::Start)
                                ).padding([32., 24.]).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                        background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                        shadow: Shadow::default(),
                                    }
                                ),
                                container(
                                    match state.is_selected_uxtos {
                                        true => {
                                            column![
                                                text("Unlock date").size(16).font(Font {
                                                    weight: font::Weight::Semibold,
                                                    ..Font::DEFAULT
                                                }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                                row![
                                                    text("Jan 3, 2026, 12:00 AM").size(16).font(Font {
                                                        weight: font::Weight::Semibold,
                                                        ..Font::DEFAULT
                                                    }).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)).line_height(1.5),
                                                    text("UTC").size(16).font(Font {
                                                        weight: font::Weight::Semibold,
                                                        ..Font::DEFAULT
                                                    }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                                ].spacing(4).align_items(Alignment::Center)
                                            ].spacing(4).align_items(Alignment::Start)
                                            
                                        },
                                        false => {
                                            column![""].height(0)
                                        }
                                    }      
                                ).padding(match state.is_selected_uxtos {
                                    true => 16,
                                    false => 0
                                }).width(match state.is_selected_uxtos {
                                    true => 372,
                                    false => 0
                                }).height(match state.is_selected_uxtos {
                                    true => 84,
                                    false => 0
                                }).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                        background: Some(Background::Color(Color::from_rgb(227./ 255., 239./ 255., 255.))),
                                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                        shadow: Shadow::default(),
                                    }
                                ),
                            ].spacing(24).padding(40).width(840).height(Length::Shrink)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        container(
                            column![
                                text("COST & TRANSACTION DETAILS").size(14).font(Font {
                                    weight: font::Weight::Semibold,
                                    ..Font::DEFAULT
                                }),
                                column![
                                    row![
                                        text("Bitcoin provided").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).width(Length::Fill),
                                        container(row![
                                            Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                            text("3 BTC").size(16).font(Font {
                                                weight: font::Weight::Semibold,
                                                ..Font::DEFAULT
                                            }).line_height(1.5)
                                        ].spacing(2.5).padding([4., 6., 4., 4.0]).align_items(Alignment::Center)).style(
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
                                                    radius: 100.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Shrink).height(Length::Shrink),
                                    ].width(Length::Fill).height(Length::Shrink).align_items(Alignment::Center),
                                    container(
                                        row![""]
                                    ).style(
                                        Appearance {
                                            text_color: None,
                                            background: None,
                                            border: Border {
                                                color: Color::from_rgba (
                                                    236. / 255.,
                                                    238. / 255.,
                                                    242. /255.,
                                                    100.
                                                ),
                                                width: 1.,
                                                radius: 0.0.into()
                                            },
                                            shadow: Shadow::default()
                                        }
                                    ).width(Length::Fill).height(1),
                                    row![
                                        row![
                                            text("Service fee").size(16).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                            Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                        ].spacing(4).align_items(Alignment::Center).width(Length::Fill),
                                        container(row![
                                            Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                            text("0.0013 BTC").size(16).font(Font {
                                                weight: font::Weight::Semibold,
                                                ..Font::DEFAULT
                                            }).line_height(1.5)
                                        ].spacing(2.5).padding([4., 6., 4., 4.0]).align_items(Alignment::Center)).style(
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
                                                    radius: 100.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Shrink).height(Length::Shrink),
                                    ].width(Length::Fill).height(Length::Shrink).align_items(Alignment::Center),
                                    container(
                                        row![""]
                                    ).style(
                                        Appearance {
                                            text_color: None,
                                            background: None,
                                            border: Border {
                                                color: Color::from_rgba (
                                                    236. / 255.,
                                                    238. / 255.,
                                                    242. /255.,
                                                    100.
                                                ),
                                                width: 1.,
                                                radius: 0.0.into()
                                            },
                                            shadow: Shadow::default()
                                        }
                                    ).width(Length::Fill).height(1),
                                    row![
                                        column![
                                            row![
                                            text("Total Bitcoin time-locked after service fee").size(16).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                            tooltip(
                                                Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                                row![
                                                    Svg::from_path("assets/create-plan/tooltip_polygon.svg").width(Length::Fixed(9.)).height(Length::Fixed(21.)),
                                                    container(text("The fee is deducted from any assets in the user’s wallet that are outside of the Inherit plan. If no funds exist outside the Inherit plan, the fee will be deducted from the BTC balance before it is time-locked.").line_height(1.5).size(14).font(Font {
                                                    weight: font::Weight::Medium,
                                                    ..Font::DEFAULT
                                                }).width(361).height(100)).padding([12.0, 16.0]).style(
                                                    Appearance {
                                                        text_color: Some(Color::from_rgb(1., 1., 1.)),
                                                        background: Some(Background::Color(Color::from_rgb(0., 62. /255., 144. /255.))),
                                                        border: Border { color: Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 0.0), width: 0., radius: 16.0.into() },
                                                        shadow: Shadow::default(),
                                                    }
                                                )
                                                ].spacing(0).align_items(Alignment::Center),
                                                tooltip::Position::Right,
                                            )
                                            ].spacing(4).align_items(Alignment::Center),
                                            text("Total amount of BTC locked reduced by fee").size(12).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        ].width(Length::Fill).spacing(4).align_items(Alignment::Start),
                                        container(row![
                                            Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                            text("2.9987 BTC").size(16).font(Font {
                                                weight: font::Weight::Semibold,
                                                ..Font::DEFAULT
                                            }).line_height(1.5)
                                        ].spacing(2.5).padding([4., 6., 4., 4.0]).align_items(Alignment::Center)).style(
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
                                                    radius: 100.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Shrink).height(Length::Shrink),
                                    ].width(Length::Fill).height(Length::Shrink).align_items(Alignment::Center),
                                ].spacing(12).width(Length::Fill).height(Length::Shrink).align_items(Alignment::Start),
                                row![
                                    text("Hide breakdown").size(14).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(2. / 255., 84. / 255., 191. / 255.)).line_height(1.5),
                                    Svg::from_path("assets/create-plan/check_down.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                ].spacing(8).align_items(Alignment::Center),
                                column![
                                    text("Fee breakdown").size(14).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5),
                                    column![
                                        row![
                                            text("Total fee").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            text("0.0013 BTC").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                        container(
                                            row![""]
                                        ).style(
                                            Appearance {
                                                text_color: None,
                                                background: None,
                                                border: Border {
                                                    color: Color::from_rgba (
                                                        236. / 255.,
                                                        238. / 255.,
                                                        242. /255.,
                                                        100.
                                                    ),
                                                    width: 1.,
                                                    radius: 0.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Fill).height(1),
                                        row![
                                            text("Service fee").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            text("0.001 BTC").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                        container(
                                            row![""]
                                        ).style(
                                            Appearance {
                                                text_color: None,
                                                background: None,
                                                border: Border {
                                                    color: Color::from_rgba (
                                                        236. / 255.,
                                                        238. / 255.,
                                                        242. /255.,
                                                        100.
                                                    ),
                                                    width: 1.,
                                                    radius: 0.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Fill).height(1),
                                        row![
                                            text("Network fee").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            text("0.003 BTC").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                    ].spacing(6).align_items(Alignment::Start).width(Length::Fill)
                                ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                                column![
                                    text("Transaction details").size(14).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5),
                                    column![
                                        row![
                                            text("Fee rate").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            text("100 sat/vByte").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                        container(
                                            row![""]
                                        ).style(
                                            Appearance {
                                                text_color: None,
                                                background: None,
                                                border: Border {
                                                    color: Color::from_rgba (
                                                        236. / 255.,
                                                        238. / 255.,
                                                        242. /255.,
                                                        100.
                                                    ),
                                                    width: 1.,
                                                    radius: 0.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Fill).height(1),
                                        row![
                                            text("Estimated confirmation time").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            text("1-2 mins").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                        container(
                                            row![""]
                                        ).style(
                                            Appearance {
                                                text_color: None,
                                                background: None,
                                                border: Border {
                                                    color: Color::from_rgba (
                                                        236. / 255.,
                                                        238. / 255.,
                                                        242. /255.,
                                                        100.
                                                    ),
                                                    width: 1.,
                                                    radius: 0.0.into()
                                                },
                                                shadow: Shadow::default()
                                            }
                                        ).width(Length::Fill).height(1),
                                        row![
                                            text("Replace-by-fee").size(14).font(Font {
                                                weight: font::Weight::Medium,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                            Svg::from_path("assets/create-plan/enabled_badge.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.)),
                                        ].align_items(Alignment::Center).width(Length::Fill),
                                    ].spacing(6).align_items(Alignment::Start).width(Length::Fill)
                                ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                            ].spacing(24).padding(40).width(840).height(Length::Shrink).align_items(Alignment::Start)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        container(
                            column![
                                text("TERMS & IMPORTANT INFORMATION").size(14).font(Font {
                                    weight: font::Weight::Semibold,
                                    ..Font::DEFAULT
                                }),

                                row![
                                    checkbox("", state.privacy_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::TogglePrivacyCheckbox),
                                    row![
                                        text("I agree to Inherit’s").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                        text("Terms of Service").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(2. /255., 84. /255., 191. /255.)).line_height(1.5),
                                        text("and").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                        text("Privacy Policy").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(2. /255., 84. /255., 191. /255.)).line_height(1.5),
                                    ].spacing(6).align_items(Alignment::Center)
                                ].spacing(16).align_items(Alignment::Center),
                                column![
                                    text("Pay attention to important aspects:").size(16).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                    container(
                                    row![
                                        checkbox("", state.understand_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleUnderstandCheckbox),
                                        text("I understand my BTC will be locked and inaccessible until the set time expires.").size(16).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                    ].spacing(8).align_items(Alignment::Center),
                                ).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                        background: Some(Background::Color(Color::from_rgb(227./ 255., 239./ 255., 1.))),
                                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                        shadow: Shadow::default(),
                                    }
                                ).width(Length::Fill).height(Length::Shrink).padding([16., 24.])
                                ]
                            ].spacing(24).padding(40.).width(840).height(Length::Shrink)
                        ).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                                shadow: Shadow::default(),
                            }
                        ),
                        
                    ].spacing(16).align_items(Alignment::Center),
                    row![
                        container(button("<- Back").style(
                            theme::Button::Custom(Box::new(BackButtonColor {}))
                            // Color::from_rgb(255., 0., 0.)
                        ).padding([12., 20.]).on_press(MyAppMessage::GoToSecondCreateNewPlanBtnPressed)).width(Length::Fill),
                        button("Create Inherit plan").style(
                            theme::Button::Custom(Box::new(ContinueButtonColor {}))
                            // Color::from_rgb(255., 0., 0.)
                        ).padding([12., 20.]).on_press(MyAppMessage::GoToForthCreateNewPlanBtnPressed)
                    ].width(840),
                    
                ].padding([10., 0., 0., 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0))),
            ].align_items(Alignment::Center)
        ).style(Appearance {
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
        }).width(Length::Fill).height(Length::Fill).center_x().into(),
        Page::ForthCreateNewPlanPage => container(
            column![
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
                ].spacing(684).padding([7.0, 12.0, 7.0, 88.]).align_items(Alignment::Center),
                container(
                    column![
                        Svg::from_path("assets/create-plan/eliptical.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)).width(180).height(180),
                        column![
                            text("Your time safe plan is being generated...").size(24).line_height(1.2).style(
                                Color::from_rgb(0., 0., 0.)
                            ).font(Font {
                                weight: font::Weight::ExtraBold,
                                ..Font::DEFAULT
                            }),
                            text("Please wait here while it is created").size(16).line_height(1.5).style(
                                Color::from_rgb(0., 0., 0.)
                            ).font(Font {
                                weight: font::Weight::Medium,
                                ..Font::DEFAULT
                            }),
                        ].spacing(8).align_items(Alignment::Center)
                    ].spacing(24).align_items(Alignment::Center)
                ).padding([40., 203.5]).style(
                    Appearance {
                        text_color: None,
                        background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                        border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                        shadow: Shadow::default(),
                    }
                ).width(840).height(Length::Shrink),
            ].spacing(256.5).align_items(Alignment::Center)
        ).style(Appearance {
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
    // row![
    //     PlanCard::new(state.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
    //     PlanCard::new(self.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
    // ].spacing(24).into()


    // PlanCard::new(fail_safe_content).into()
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