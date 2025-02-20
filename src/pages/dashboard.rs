use crate::messages::MyAppMessage;
use crate::widgets::balance_card::BalanceCard;
use crate::widgets::filter_btn_group::ButtonGroup;
use crate::{state::State, ContinueButtonColor};

use iced::theme;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient,
    widget::{container, container::Appearance, Svg, column, row, text, button, scrollable::{Direction, Properties}, Scrollable},
    font,
    gradient::{Linear, ColorStop},
    Element,
    
};

pub fn dashboard(state: &State) -> Element<'static, MyAppMessage> {
    let account_balance_card = BalanceCard::new("assets/create-plan/btc_icon.svg", "Account balance", 84.13);
    let time_safe_plan_card = BalanceCard::new("assets/create-plan/guard.svg", "Bitcoin in Time Safe(s)", 84.13);
    let fail_safe_plan_card = BalanceCard::new("assets/create-plan/btc_icon.svg", "Bitcoin in Fail Safe(s)", 84.13);
    let filter_btn_group = ButtonGroup::new(state.filter);
    container(
    Scrollable::new(column![
            row![
                container(container(row![
                    Svg::from_path("assets/create-plan/home.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    text("Dashboard").size(14),
                ].spacing(4).padding([4., 16., 4., 16.]).align_items(Alignment::Center)).width(Length::Shrink).style(
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
                )).width(Length::Fill),

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
            ].spacing(684).padding([7.0, 12.0]).align_items(Alignment::Center),
  
            container(column![
                column![
                    row![
                        container(text("Your dashboard").size(24).line_height(1.2).style(
                            Color::from_rgb(9. /255., 8. /255., 20. /255.)
                        ).font(Font {
                            weight: font::Weight::Bold,
                            ..Font::DEFAULT
                        })).width(Length::Fill),
                        button("+ Create new plan").style(
                            theme::Button::Custom(Box::new(ContinueButtonColor {}))
                        ).padding([12., 20.]).on_press(MyAppMessage::GoToForthCreateNewPlanBtnPressed)
                    ].width(Length::Fill).align_items(Alignment::Center),
                    row![
                        account_balance_card.view(),
                        time_safe_plan_card.view(),
                        fail_safe_plan_card.view()
                    ].align_items(Alignment::Center).spacing(16)
                ].spacing(32.).align_items(Alignment::Start),
                column![
                    row![
                        container(column![
                            text("Your inherit plans").size(24).line_height(1.2).style(
                                Color::from_rgb(9. /255., 8. /255., 20. /255.)
                                ).font(Font {
                                    weight: font::Weight::Bold,
                                    ..Font::DEFAULT
                                }),
                            text("Manage your inherit plans and perform status check-ins").size(16).line_height(1.5).style(
                                Color::from_rgb(9. /255., 8. /255., 20. /255.)
                                ),
                        ].spacing(8.).align_items(Alignment::Start).width(Length::Shrink)).width(Length::Fill),
                        filter_btn_group.view()
                    ].width(Length::Fill).align_items(Alignment::End),
                    column![
                        row![
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/guard.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                    Svg::from_path("assets/create-plan/locked_status.svg")
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Time safe").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan for our happy future", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                state.countdown.view()
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/guard.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                    Svg::from_path("assets/create-plan/available_status.svg")
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Time safe").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan for our happy future", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                button("Claim BTC").style(
                                    theme::Button::Custom(Box::new(ContinueButtonColor {}))
                                    // Color::from_rgb(255., 0., 0.)
                                ).padding([12., 89.5]).on_press(MyAppMessage::GoToThirdCreateNewPlanBtnPressed)
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/guard.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                    Svg::from_path("assets/create-plan/processing_status.svg")
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Time safe").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan for our happy future", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                container(
                                    text("Assets claimed. We will inform you \nwhen transaction is confirmed").size(14).line_height(1.5).style(Color::from_rgb(42. /255., 47. /255., 53. /255.))
                                ).padding(8).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(42. / 255., 47. / 255., 53. / 255.)),
                                        background: Some(Background::Color(Color::from_rgba(236. / 255., 238. / 255., 242. / 255., 1.))),
                                        border: Border {
                                            color: Color::from_rgba (
                                                217. / 255.,
                                                219. / 255.,
                                                225. /255.,
                                                100.
                                            ),
                                            width: 1.,
                                            radius: 16.0.into()
                                        },
                                        shadow: Shadow::default()
                                    }
                                ),
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/guard.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                    Svg::from_path("assets/create-plan/release_status.svg")
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Time safe").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan for our happy future", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                text("Time Safe expired on Jan 2, 2025").size(12).font(Font {
                                    weight: font::Weight::Normal,
                                    ..Font::DEFAULT
                                }).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                            ]
                        .spacing(26).align_items(Alignment::Start)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    )
                        ].align_items(Alignment::Center).spacing(16),
                        row![
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/lock_icon.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                    Svg::from_path("assets/create-plan/active_status.svg")
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Fail Safe Recovery").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                state.countdown1.view()
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                                    container(column![ 
                                        row![
                                            container(Svg::from_path("assets/create-plan/lock_icon.svg")
                                                .width(Length::Fixed(40.))
                                                .height(Length::Fixed(40.))).width(Length::Fill),
                                            Svg::from_path("assets/create-plan/grace_period.svg")
                                            .width(Length::Shrink)
                                            .height(Length::Shrink)
                                        ].width(Length::Fill),
                                        container(column![
                                            column![
                                                text("Fail Safe Recovery").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                text(format!("{}’s active bitcoin plan", state.plan_name)).size(16).font(Font {
                                                    weight: font::Weight::Bold,
                                                    ..Font::DEFAULT
                                                }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                            ].align_items(Alignment::Start).width(Length::Fill),
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
                                        ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                        state.countdown2.view()
                                    ]
                                .spacing(26).align_items(Alignment::Center)
                                )
                                        .padding(32.)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .width(328)
                                        .height(345)
                                        .style(
                                            Appearance {
                                                    text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                    background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                                    border: Border {
                                                        color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                        width: 0.75,
                                                        radius: 32.0.into()
                                                    },
                                                    shadow: Shadow::default()
                                                
                                            }
                                            ),
                                            container(column![ 
                                                row![
                                                    container(Svg::from_path("assets/create-plan/nonactive_lock_icon.svg")
                                                        .width(Length::Fixed(40.))
                                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                                    Svg::from_path("assets/create-plan/completed_status.svg")
                                                    .width(Length::Shrink)
                                                    .height(Length::Shrink)
                                                ].width(Length::Fill),
                                                container(column![
                                                    column![
                                                        text("Fail Safe Recovery").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                        text(format!("{}’s active bitcoin plan", state.plan_name)).size(16).font(Font {
                                                            weight: font::Weight::Bold,
                                                            ..Font::DEFAULT
                                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                                text("Fail Safe Recovery expired on Jan 2, 2025").size(12).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                            ]
                                        .spacing(26).align_items(Alignment::Start)
                                        )
                                                .padding(32.)
                                                .align_x(iced::alignment::Horizontal::Center)
                                                .align_y(iced::alignment::Vertical::Center)
                                                .width(328)
                                                .height(345)
                                                .style(
                                                    Appearance {
                                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                                            border: Border {
                                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                                width: 0.75,
                                                                radius: 32.0.into()
                                                            },
                                                            shadow: Shadow::default()
                                                        
                                                    }
                                                    ),
                        ].align_items(Alignment::Center).spacing(16),
                        row![
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/mailbox_icon.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        column![text("Receive assets").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text(format!("{}’s active bitcoin plan", state.plan_name)).size(16).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.))],
                                        text("The time-lock on an inheritance plan \nhas expired, and assets have been \ntransferred to your wallet as a \nbeneficiary. ").size(14).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.5).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill).spacing(12),
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                button("Claim BTC").style(
                                    theme::Button::Custom(Box::new(ContinueButtonColor {}))
                                    // Color::from_rgb(255., 0., 0.)
                                ).padding([12., 89.5]).on_press(MyAppMessage::GoToThirdCreateNewPlanBtnPressed)
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                                    container(column![ 
                                        row![
                                            container(Svg::from_path("assets/create-plan/guard.svg")
                                                .width(Length::Fixed(40.))
                                                .height(Length::Fixed(40.))).width(Length::Fill),
                                            Svg::from_path("assets/create-plan/locked_status.svg")
                                            .width(Length::Shrink)
                                            .height(Length::Shrink)
                                        ].width(Length::Fill),
                                        container(column![
                                            column![
                                                text("Time safe").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                text(format!("{}’s  active bitcoin plan for our happy future", state.plan_name)).size(16).font(Font {
                                                    weight: font::Weight::Bold,
                                                    ..Font::DEFAULT
                                                }).line_height(1.2).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                            ].align_items(Alignment::Start).width(Length::Fill),
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
                                        ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                        state.countdown.view()
                                    ]
                                .spacing(26).align_items(Alignment::Center)
                                )
                                        .padding(32.)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .width(328)
                                        .height(345)
                                        .style(
                                            Appearance {
                                                    text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                    background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                                    border: Border {
                                                        color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                        width: 0.75,
                                                        radius: 32.0.into()
                                                    },
                                                    shadow: Shadow::default()
                                                
                                            }
                                            ),
                        ].align_items(Alignment::Center).spacing(16),
                        row![
                            container(column![ 
                                row![
                                    container(Svg::from_path("assets/create-plan/mailbox_icon.svg")
                                        .width(Length::Fixed(40.))
                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                ].width(Length::Fill),
                                container(column![
                                    column![
                                        text("Receive assets").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        text("The time-lock on an inheritance plan \nhas expired, and assets have been \ntransferred to your wallet as a \nbeneficiary. ").size(14).line_height(1.5).style(Color::from_rgb(9. /255., 8. /255., 20. /255.)),
                                    ].align_items(Alignment::Start).width(Length::Fill).spacing(12),
                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                button("Claim BTC").style(
                                    theme::Button::Custom(Box::new(ContinueButtonColor {}))
                                    // Color::from_rgb(255., 0., 0.)
                                ).padding([12., 89.5]).on_press(MyAppMessage::GoToThirdCreateNewPlanBtnPressed)
                            ]
                        .spacing(26).align_items(Alignment::Center)
                        )
                                .padding(32.)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(328)
                                .height(345)
                                .style(
                                    Appearance {
                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                            border: Border {
                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                width: 0.75,
                                                radius: 32.0.into()
                                            },
                                            shadow: Shadow::default()
                                        
                                    }
                                    ),
                                    container(column![ 
                                        row![
                                            container(Svg::from_path("assets/create-plan/nonactive_lock_icon.svg")
                                                .width(Length::Fixed(40.))
                                                .height(Length::Fixed(40.))).width(Length::Fill),
                                            Svg::from_path("assets/create-plan/processing_status.svg")
                                            .width(Length::Shrink)
                                            .height(Length::Shrink)
                                        ].width(Length::Fill),
                                        container(column![
                                            column![
                                                text("Fail Safe Recovery").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                            ].align_items(Alignment::Start).width(Length::Fill),
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
                                        ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                        text("Assets claimed on Jan 2, 2025").size(12).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                    ]
                                .spacing(26).align_items(Alignment::Start)
                                )
                                        .padding(32.)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .width(328)
                                        .height(345)
                                        .style(
                                            Appearance {
                                                    text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                    background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                                    border: Border {
                                                        color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                        width: 0.75,
                                                        radius: 32.0.into()
                                                    },
                                                    shadow: Shadow::default()
                                                
                                            }
                                            ),
                                            container(column![ 
                                                row![
                                                    container(Svg::from_path("assets/create-plan/nonactive_lock_icon.svg")
                                                        .width(Length::Fixed(40.))
                                                        .height(Length::Fixed(40.))).width(Length::Fill),
                                                    Svg::from_path("assets/create-plan/completed_status.svg")
                                                    .width(Length::Shrink)
                                                    .height(Length::Shrink)
                                                ].width(Length::Fill),
                                                container(column![
                                                    column![
                                                        text("Fail Safe Recovery").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                    ].align_items(Alignment::Start).width(Length::Fill),
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
                                                ].spacing(12).align_items(Alignment::Start)).height(Length::Fill),
                                                text("Assets claimed on Jan 3, 2025").size(12).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                            ]
                                        .spacing(26).align_items(Alignment::Center)
                                        )
                                                .padding(32.)
                                                .align_x(iced::alignment::Horizontal::Center)
                                                .align_y(iced::alignment::Vertical::Center)
                                                .width(328)
                                                .height(345)
                                                .style(
                                                    Appearance {
                                                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                            background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                                                            border: Border {
                                                                color: Color::from_rgba(205. / 255., 220. / 255., 241. / 255., 0.3),
                                                                width: 0.75,
                                                                radius: 32.0.into()
                                                            },
                                                            shadow: Shadow::default()
                                                        
                                                    }
                                                    ),        
                        ].align_items(Alignment::Center).spacing(16),
                    ].align_items(Alignment::Start).spacing(16),
                    
                ].spacing(32).align_items(Alignment::Center)
            ].spacing(48.).align_items(Alignment::Center)).padding([0., 40., 82., 40.])
        ].spacing(40.).align_items(Alignment::Center).width(Length::Fill)).height(1024).direction(Direction::Vertical(Properties::new().scroller_width(4).width(0)))
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