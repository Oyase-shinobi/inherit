use crate::messages::MyAppMessage;
use crate::state::State;
use crate::{BackButtonColor, ContinueButtonColor};
use iced::advanced::graphics::core::Element;
use iced::Renderer;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient, theme,
    widget::{container, container::Appearance, Svg, column, row, text, button, Theme},
    font,
    gradient::{Linear, ColorStop},
};

pub fn fs_plan_created_page(state: &State) -> Element<'static, MyAppMessage, Theme, Renderer> {
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
            ].spacing(684).padding([7.0, 12.0, 7.0, 88.]).align_items(Alignment::Center),
            
            row![
                container(
                    column![
                        Svg::from_path("assets/create-plan/eliptical.svg").width(Length::Fixed(120.)).height(Length::Fixed(120.)),
                        column![
                            text("0.235 BTC is now time-locked!").size(24).line_height(1.2).style(
                                Color::from_rgb(0., 0., 0.)
                            ).font(Font {
                                weight: font::Weight::ExtraBold,
                                ..Font::DEFAULT
                            }),
                            column![
                                text("Protected against").size(14).line_height(1.5).style(
                                    Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }),
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
                                ].spacing(8).align_items(Alignment::Center)
                            ].spacing(8).align_items(Alignment::Center)
                        ].spacing(24).align_items(Alignment::Center),
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
                        column![
                            row![
                                Svg::from_path("assets/create-plan/abstruct-rounded.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                text("What’s next?").size(20).line_height(1.2).style(
                                    Color::from_rgb(0., 0., 0.)
                                ).font(Font {
                                    weight: font::Weight::ExtraBold,
                                    ..Font::DEFAULT
                                }),
                            ].spacing(8).align_items(Alignment::Center),
                            column![
                                text("1. Wait for the time safe to expire, then claim your Bitcoin.").size(14).line_height(1.5).style(
                                    Color::from_rgb(9. /255., 8. /255., 20. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }),
                                text("2. Add extra protection by setting up a fail safe to guard\n    against private key loss or unforeseen life events.").size(14).line_height(1.5).style(
                                    Color::from_rgb(9. /255., 8. /255., 20. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }),
                            ]
                        ].spacing(16).align_items(Alignment::Start).width(Length::Fill),
                        row![
                            container(button("Create Fail Safe plan").style(
                                theme::Button::Custom(Box::new(ContinueButtonColor {}))
                            ).padding([12., 20.]).on_press(MyAppMessage::GoToSecondCreateNewPlanBtnPressed)).width(Length::Fill),
                            container(button("Go to dashboard").style(
                                theme::Button::Custom(Box::new(BackButtonColor {}))
                            ).padding([12., 20.]).on_press(MyAppMessage::GoToDashboardPage)),
                        ].width(409),
                    ].spacing(32).align_items(Alignment::Center)).padding([40., 80.]).style(
                        Appearance {
                            text_color: None,
                            background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                            border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                            shadow: Shadow::default(),
                        }
                ).width(608.5).height(610),
                container(
                    column![
                        row![
                            container(text("Plan details").size(14).line_height(1.2).style(
                                Color::from_rgb(158. /255., 168. /255., 190. /255.)
                            ).font(Font {
                                weight: font::Weight::ExtraBold,
                                ..Font::DEFAULT
                            })).width(Length::Fill),
                            text("ID: 29382-3-423").size(14).line_height(1.5).style(
                                Color::from_rgb(113. /255., 121. /255., 142. /255.)
                            ).font(Font {
                                weight: font::Weight::Medium,
                                ..Font::DEFAULT
                            }),
                        ].align_items(Alignment::Start).width(Length::Fill),
                        row![
                            container(
                                column![
                                    text("Time Safe plan").size(12).line_height(1.4).style(
                                        Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                    ).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }),
                                    text( state.plan_name.clone() + "'s plan").size(20).line_height(1.2).style(
                                        Color::from_rgb(8. /255., 15. /255., 33. /255.)
                                    ).font(Font {
                                        weight: font::Weight::ExtraBold,
                                        ..Font::DEFAULT
                                    }),
                                    row![
                                        text("Address: jbkdsvdg9...hfebrc49ejcin").size(12).line_height(1.4).font(Font {
                                            weight: font::Weight::Medium,
                                            ..Font::DEFAULT
                                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                    ].spacing(4).align_items(Alignment::Center).width(227),
                                ].align_items(Alignment::Start).spacing(4)
                            ).width(Length::Fill),
                            column![
                                text("Time-locked").size(12).line_height(1.4).style(
                                    Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }),
                                container(row![
                                    Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                    text("0.234 BTC").size(16).line_height(1.5)
                                ].spacing(4).padding([4., 6.0, 4., 4.0]).align_items(Alignment::Center)).style(
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
                            ].align_items(Alignment::End).spacing(4)
                        ].align_items(Alignment::Start).width(Length::Fill),
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
                            container(
                                column![
                                    text("Unlock date").size(16).line_height(1.5).style(
                                        Color::from_rgb(20. /255., 23. /255., 23. /255.)
                                    ).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }),
                                    text( "Securely record the unlock date in a way that\nensures easy reference for you and your\nbeneficiaries when needed").size(12).line_height(1.4).style(
                                        Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                    ).font(Font {
                                        weight: font::Weight::ExtraBold,
                                        ..Font::DEFAULT
                                    }),

                                ].align_items(Alignment::Start).spacing(4)
                            ).width(Length::Fill),
                            column![
                                row![
                                    text("Jan 3, 2026, 12:00 AM").size(16).line_height(1.5).style(
                                        Color::from_rgb(8. /255., 15. /255., 33. /255.)
                                    ).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }),
                                    text("UTC").size(16).line_height(1.5).style(
                                        Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                    ).font(Font {
                                        weight: font::Weight::Medium,
                                        ..Font::DEFAULT
                                    }),
                                ].align_items(Alignment::End).spacing(4),
                                
                                container(row![
                                    Svg::from_path("assets/create-plan/calendar_plus.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                    text("Add to calendar").size(14).line_height(1.5),
                                    Svg::from_path("assets/create-plan/below_arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                ].spacing(4).padding([9.5, 16.0]).align_items(Alignment::Center)).style(
                                    Appearance {
                                        text_color: Some(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)),
                                        background: Some(Background::Color(Color::from_rgba(255. / 255., 255. / 255., 255. / 255., 1.))),
                                        border: Border {
                                            color: Color::from_rgba (
                                                236. / 255.,
                                                238. / 255.,
                                                242. /255.,
                                                100.
                                            ),
                                            width: 1.,
                                            radius: 12.0.into()
                                        },
                                        shadow: Shadow::default()
                                    }
                                )
                            ].align_items(Alignment::End).spacing(4)
                        ].align_items(Alignment::Start).width(Length::Fill),
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
                        column![
                            row![
                                text("Bitcoin transaction link").size(14).line_height(1.5).style(
                                    Color::from_rgb(20. /255., 23. /255., 23. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                }),
                                Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            ].align_items(Alignment::Center).spacing(4),
                            container(row![
                                container(text("https://www.blockchain.com/explorer/transactions/btc/f4a8f0379...").size(14).line_height(1.5).style(
                                    Color::from_rgb(42. /255., 47. /255., 53. /255.)
                                ).font(Font {
                                    weight: font::Weight::Medium,
                                    ..Font::DEFAULT
                                })).width(Length::Fill),
                                Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                            ].spacing(8).align_items(Alignment::Center).width(Length::Fill)).style(
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
                            ).padding([8.5, 12.0])
                        ].align_items(Alignment::Start).spacing(2)
                    ].spacing(32).align_items(Alignment::Center)).padding([105., 40.]).style(
                        Appearance {
                            text_color: None,
                            background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                            border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                            shadow: Shadow::default(),
                        }
                ).width(608.5).height(610),
            ].spacing(10).align_items(Alignment::Center)
        ].spacing(150).align_items(Alignment::Center)
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
                        a: 0.45,
                    },
                }),
                Some(ColorStop {
                    offset: 0.5191,
                    color: Color {
                        r: 62.0 / 255.0,
                        g: 112.0 / 255.0,
                        b: 253.0 / 255.0,
                        a: 0.45,
                    },
                }),
                Some(ColorStop {
                    offset: 0.939,
                    color: Color {
                        r: 135.0 / 255.0,
                        g: 85.0 / 255.0,
                        b: 241.0 / 255.0,
                        a: 0.38,
                    },
                }),
                None, None, None, None, None,
            ]
        }))),
        ..Appearance::default()
    }).width(Length::Fill).height(Length::Fill).center_x().into()
}
