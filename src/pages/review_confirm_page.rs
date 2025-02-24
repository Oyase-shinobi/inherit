use crate::messages::MyAppMessage;
use crate::state::State;
use crate::{BackButtonColor, ContinueButtonColor};
use iced::advanced::graphics::core::Element;
use iced::Renderer;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient, theme,
    widget::{container, container::Appearance, Svg, column, row, text, button, Theme, tooltip, checkbox, scrollable::{Direction, Properties}, Scrollable},
    font,
    gradient::{Linear, ColorStop},
};

pub fn review_confirm_page(state: &State) -> Element<'static, MyAppMessage, Theme, Renderer> {
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
                                weight: font::Weight::Bold,
                                ..Font::DEFAULT
                            }),
                            column![
                                text("Time Safe plan").size(14).font(Font {
                                    weight: font::Weight::Bold,
                                    ..Font::DEFAULT
                                }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                text(state.plan_name.clone() + "'s plan").size(24).font(Font {
                                    weight: font::Weight::Bold,
                                    ..Font::DEFAULT
                                }).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)).line_height(1.2),
                                row![
                                    text("Address: jbkdsvdg9...hfebrc49ejcin").size(14).font(Font {
                                        weight: font::Weight::Bold,
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
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }).line_height(1.2),
                                        row![
                                            text("Address: jbkdsvdg9...hfebrc49ejcin").size(14).font(Font {
                                                weight: font::Weight::Bold,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                        ].spacing(4).align_items(Alignment::Center)
                                    ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                                    container(row![
                                        Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                        text(format!("{} BTC", state.lock_btc_amount)).size(16)
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
                                                    weight: font::Weight::Bold,
                                                    ..Font::DEFAULT
                                                }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).line_height(1.5),
                                                container(column![
                                                    row![
                                                        container(text("Amount BTC").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                        text("Address").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                                                        row![
                                                            text("Confirmations").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                            Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                                        ].spacing(4).align_items(Alignment::Center).width(178),
                                                        text("Status").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                                                        
                                                    ].align_items(Alignment::Center).height(44),
                                                    Scrollable::new(column![
                                                        container(row![

                                                            container(text("0.015").size(14).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                        row![
                                                            text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                        ].spacing(8).align_items(Alignment::Center).width(178),
                                
                                                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
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

                                                            container(text("0.015").size(14).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            row![
                                                                text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                            ].spacing(8).align_items(Alignment::Center).width(178),
                                
                                                            text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
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

                                                            container(text("0.015").size(14).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            row![
                                                                text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                            ].spacing(8).align_items(Alignment::Center).width(178),
                                
                                                            text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
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

                                                            container(text("0.015").size(14).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            row![
                                                                text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                            ].spacing(8).align_items(Alignment::Center).width(178),
                                
                                                            text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
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

                                                            container(text("0.015").size(14).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178)).padding([0, 0, 0, 16]),
                                                            row![
                                                                text("as9sk0...wi9dso").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                                                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                                            ].spacing(8).align_items(Alignment::Center).width(178),
                                
                                                            text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
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
                                                            weight: font::Weight::Bold,
                                                            ..Font::DEFAULT
                                                        }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                                        text("selected").size(16).font(Font {
                                                            weight: font::Weight::Bold,
                                                            ..Font::DEFAULT
                                                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.))
                                                    ].spacing(4).align_items(Alignment::Center),
                                                    row![
                                                        text("Total BTC to lock").size(16).font(Font {
                                                            weight: font::Weight::Bold,
                                                            ..Font::DEFAULT
                                                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                        text("0.015 BTC").size(16).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                                    ].spacing(4).align_items(Alignment::Center),
                                                    row![
                                                        text("Time lock duration:").size(16).font(Font {
                                                            weight: font::Weight::Bold,
                                                            ..Font::DEFAULT
                                                        }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                                        text("6 months").size(16).font(Font {
                                                            weight: font::Weight::Bold,
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
                                                weight: font::Weight::Bold,
                                                ..Font::DEFAULT
                                            }).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).line_height(1.5),
                                            row![
                                                text("Jan 3, 2026, 12:00 AM").size(16).font(Font {
                                                    weight: font::Weight::Bold,
                                                    ..Font::DEFAULT
                                                }).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)).line_height(1.5),
                                                text("UTC").size(16).font(Font {
                                                    weight: font::Weight::Bold,
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
                                weight: font::Weight::Bold,
                                ..Font::DEFAULT
                            }),
                            column![
                                row![
                                    text("Bitcoin provided").size(16).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)).width(Length::Fill),
                                    container(row![
                                        Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                        text(format!("{} BTC", state.lock_btc_amount)).size(16).line_height(1.5)
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
                                        text("Service fee").size(16).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                        Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                    ].spacing(4).align_items(Alignment::Center).width(Length::Fill),
                                    container(row![
                                        Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                        text("0.0013 BTC").size(16).line_height(1.5)
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
                                        text("Total Bitcoin time-locked after service fee").size(16).line_height(1.5).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
                                        tooltip(
                                            Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                                            row![
                                                Svg::from_path("assets/create-plan/tooltip_polygon.svg").width(Length::Fixed(9.)).height(Length::Fixed(21.)),
                                                container(text("The fee is deducted from any assets in the user’s wallet that are outside of the Inherit plan. If no funds exist outside the Inherit plan, the fee will be deducted from the BTC balance before it is time-locked.").line_height(1.5).size(14).width(361).height(100)).padding([12.0, 16.0]).style(
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
                                        text("Total amount of BTC locked reduced by fee").size(12).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                                    ].width(Length::Fill).spacing(4).align_items(Alignment::Start),
                                    container(row![
                                        Svg::from_path("assets/create-plan/btc_image.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                                        text(format!("{} BTC", state.lock_btc_amount.parse::<f32>().unwrap() - 0.0013)).size(16).line_height(1.5)
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
                                text("Hide breakdown").size(14).style(Color::from_rgb(2. / 255., 84. / 255., 191. / 255.)).line_height(1.5),
                                Svg::from_path("assets/create-plan/check_down.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            ].spacing(8).align_items(Alignment::Center),
                            column![
                                text("Fee breakdown").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5),
                                column![
                                    row![
                                        text("Total fee").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                        text("0.0013 BTC").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
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
                                        text("Service fee").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                        text("0.001 BTC").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
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
                                        text("Network fee").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                        text("0.003 BTC").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
                                    ].align_items(Alignment::Center).width(Length::Fill),
                                ].spacing(6).align_items(Alignment::Start).width(Length::Fill)
                            ].spacing(8).align_items(Alignment::Start).width(Length::Fill),
                            column![
                                text("Transaction details").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5),
                                column![
                                    row![
                                        text("Fee rate").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                        text("100 sat/vByte").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
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
                                        text("Estimated confirmation time").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
                                        text("1-2 mins").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)).line_height(1.5)
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
                                        text("Replace-by-fee").size(14).style(Color::from_rgb(20. / 255., 23. / 255., 23. / 255.)).line_height(1.5).width(Length::Fill),
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
                                weight: font::Weight::Bold,
                                ..Font::DEFAULT
                            }),

                            row![
                                checkbox("", state.privacy_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::TogglePrivacyCheckbox),
                                row![
                                    text("I agree to Inherit’s").size(16).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                    text("Terms of Service").size(16).style(Color::from_rgb(2. /255., 84. /255., 191. /255.)).line_height(1.5),
                                    text("and").size(16).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                    text("Privacy Policy").size(16).style(Color::from_rgb(2. /255., 84. /255., 191. /255.)).line_height(1.5),
                                ].spacing(6).align_items(Alignment::Center)
                            ].spacing(16).align_items(Alignment::Center),
                            column![
                                text("Pay attention to important aspects:").size(16).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
                                container(
                                row![
                                    checkbox("", state.understand_is_checked).size(16.).spacing(0.).on_toggle(MyAppMessage::ToggleUnderstandCheckbox),
                                    text("I understand my BTC will be locked and inaccessible until the set time expires.").size(16).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).line_height(1.5),
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
                
            ].padding([10., 0., 80., 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0))),
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