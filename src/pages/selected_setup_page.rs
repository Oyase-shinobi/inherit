use crate::messages::MyAppMessage;
use crate::state::State;
use crate::{BackButtonColor, ContinueButtonColor, CustomTextInputStyle};
use iced::advanced::graphics::core::Element;
use iced::alignment::Vertical;
use iced::Renderer;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient, theme,
    widget::{container, container::Appearance, Svg, column, row, text, button, Theme, mouse_area, text_input, tooltip, checkbox, scrollable::{Direction, Properties}, Scrollable, toggler},
    font,
    gradient::{Linear, ColorStop},
};

pub fn selected_setup_page(state: &State) -> Element<'static, MyAppMessage, Theme, Renderer> {
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
    container(
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
                
            ].padding([10., 0., 80.0, 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0))),
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
    }).width(Length::Fill).height(Length::Fill).center_x().into()
}