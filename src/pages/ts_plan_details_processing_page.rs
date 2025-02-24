use crate::messages::MyAppMessage;
use crate::state::State;
use iced::advanced::graphics::core::Element;
use iced::Renderer;
use iced::{
    self, Length, Font, Color, Background, Border, Shadow, Alignment, Gradient,
    widget::{container, container::Appearance, Svg, column, Column, row, text, Theme, mouse_area, scrollable::{Direction, Properties}, Scrollable},
    font,
    gradient::{Linear, ColorStop}
};

pub fn ts_plan_details_processing_page(state: &State) -> Element<'static, MyAppMessage, Theme, Renderer> {
    let recommend_alert = match state.recommend_alert_visible {
        true => row![
            row![
                Svg::from_path("assets/create-plan/info.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                text("Assets claimed. We will inform you when transaction is confirmed.")
                    .size(14)
                    .width(500)
                    .style(Color::from_rgb(42. / 255., 47. / 255., 53. / 255.))
                            
            ].spacing(16).align_items(Alignment::Start),
        ].spacing(150).width(1028).padding([16, 16, 16, 35]).align_items(Alignment::Start),
        false => row!("").height(0)
    };
    let select_uxtos_component = match state.is_selected_uxtos {
        false => column![
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
        ].padding([20.0, 24.0]).spacing(16).width(760).align_items(Alignment::Start),
        true =>
            column![

            container(column![
                
                row![
                    container(text("Amount BTC").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                    text("Address").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                    row![
                        text("Confirmations").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                        Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    ].spacing(4).align_items(Alignment::Center).width(178),
                    text("Status").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(178),
                    
                ].align_items(Alignment::Center),
                Scrollable::new(column![
                    container(row![
                        container(text("0.015").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                            Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(178),

                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                        container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(178),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(text("0.015").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(178),

                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                        container(Svg::from_path("assets/create-plan/spendable_status.svg").width(Length::Fixed(84.)).height(Length::Fixed(25.))).width(178),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(text("0.015").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(178),

                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                        container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(178),

                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    container(row![
                        container(text("0.015").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                        row![
                            text("wehht6...dgfzdc").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(178),

                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                        container(Svg::from_path("assets/create-plan/pending_status.svg").width(Length::Fixed(68.)).height(Length::Fixed(25.))).width(178),
                    ].align_items(Alignment::Center)).style(
                        Appearance {
                            text_color: None,
                            background: None,
                            border: Border { color: Color::from_rgb(205. / 220., 238. / 255., 241. / 255.), width: 1., radius: 0.0.into() },
                            shadow: Shadow::default(), 
                        }
                    ),
                    row![
                        container(text("0.015").size(14).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(162)).padding([10., 0., 13., 16.]),
                        row![
                            text("as9sk0...wi9dso").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)),
                        Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        ].spacing(8).align_items(Alignment::Center).width(178),

                        text("15").size(14).line_height(1.5).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(178),
                        container(Svg::from_path("assets/create-plan/locked_status.svg").width(Length::Fixed(61.)).height(Length::Fixed(25.))).width(178),
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
                    text("0.015 BTC").size(16).font(Font {
                        weight: font::Weight::Bold,
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(0. /255., 0. /255., 0. /255.)),
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
        ].padding([20.0, 24.0]).spacing(16).width(760).align_items(Alignment::Start)
    };
    let mut time_pick_list: Column<MyAppMessage> = Column::new();
    for time in state.times.clone() {
        let time_clone = time.clone();
        time_pick_list = time_pick_list
        .push(
            mouse_area(container(row![
                    container(text(time_clone.clone()).size(14).line_height(1.5).font(Font {
                        weight: match state.selected_time == time_clone {
                            true => font::Weight::Bold,
                            false => font::Weight::Normal
                        },
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(Length::Fill)),
                    match state.selected_time == time_clone {
                        true => Svg::from_path("assets/create-plan/blue_check.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        false => Svg::new("")
                    }
            ].align_items(Alignment::Center).width(Length::Fill)).padding([8., 12.])).on_press(MyAppMessage::TimeSelected(time))
        )
        .width(Length::Fill).align_items(Alignment::Center);
    }
    let timezones = vec!["PTC", "EST", "CST", "MST", "UTC", "GMT", "CET", "IST", "JST", "AEST"];
    let mut timezone_pick_list: Column<MyAppMessage> = Column::new();
    for timezone in timezones {
        let timezone_clone = timezone;
        timezone_pick_list = timezone_pick_list
        .push(
            mouse_area(container(row![
                    container(text(timezone_clone).size(14).line_height(1.5).font(Font {
                        weight: match state.timezone == timezone_clone {
                            true => font::Weight::Bold,
                            false => font::Weight::Normal
                        },
                        ..Font::DEFAULT
                    }).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(Length::Fill)),
                    match state.timezone == timezone_clone {
                        true => Svg::from_path("assets/create-plan/blue_check.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                        false => Svg::new("")
                    }
            ].align_items(Alignment::Center).width(Length::Fill)).padding([8., 12.])).on_press(MyAppMessage::TimeZoneSelected(timezone.to_string()))
        )
        .width(Length::Fill).align_items(Alignment::Center);
    }
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
                        text("Plan details").size(14),
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
            container(row![
                container(mouse_area(
                    text("<- Back").size(16).line_height(1.5).style(Color::from_rgb(
                        113. /255., 121. /255., 142. /255.
                    ))
                ).on_press(MyAppMessage::GoToDashboardPage)).padding([12., 0.]),
                Scrollable::new(column![
                column![
                    container(
                        column![
                            row![
                                container(column![
                                    row![
                                        Svg::from_path("assets/create-plan/processing_status.svg").width(Length::Fixed(136.)).height(Length::Fixed(25.)),
                                        text("You’re a plan owner").size(12).line_height(1.4).style(Color::from_rgb(113. /255., 121. /255., 142. /255.))
                                    ].spacing(8).align_items(Alignment::Center),
                                    column![
                                        text(format!("{}’s active bitcoin plan", state.plan_name.clone())).size(24).line_height(1.2).style(Color::from_rgb(8. /255., 15. /255., 33. /255.)).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }),
                                        text("Time safe plan").size(14).line_height(1.5).style(Color::from_rgb(8. /255., 15. /255., 33. /255.))
                                    ].spacing(4).align_items(Alignment::Start)
                                ].spacing(16).align_items(Alignment::Start)).width(Length::Fill),
                                container(
                                    text("ID: 29382-3-423").size(14).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.))
                                ).padding([9.5, 0.])        
                            ].width(Length::Fill).align_items(Alignment::Start),
                            recommend_alert,
                            container(column![row![
                                container(text("Amount time-locked").size(16).line_height(1.5)).width(Length::Fill),
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
                                        ),
                                        text( "Securely record the unlock date in a way that ensures easy reference for you \nand your beneficiaries when needed").size(12).line_height(1.4).style(
                                            Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                        ).font(Font {
                                            weight: font::Weight::Bold,
                                            ..Font::DEFAULT
                                        }),

                                    ].align_items(Alignment::Start).spacing(4)
                                ).width(Length::Fill),
                                column![
                                    row![
                                        text(format!("{} {}, {}, {}",  state.month_names[state.month.clone() as usize - 1] , state.day.clone(), state.year.clone(), state.selected_time.clone())).size(16).line_height(1.5).style(
                                            Color::from_rgb(8. /255., 15. /255., 33. /255.)
                                        ),
                                        text("UTC").size(16).line_height(1.5).style(
                                            Color::from_rgb(113. /255., 121. /255., 142. /255.)
                                        ),
                                    ].align_items(Alignment::End).spacing(4),
                                ].align_items(Alignment::End).spacing(4)
                            ].align_items(Alignment::Start).width(Length::Fill)].align_items(Alignment::Center).spacing(16).width(Length::Fill)).padding([20., 24.]).width(Length::Fill).style(
                                Appearance {
                                    text_color: Some(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                                    background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                    border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                    shadow: Shadow::default(),
                                }
                            ),
                            column![
                                text("Bitcoin transaction link").size(14).line_height(1.5).style(
                                    Color::from_rgb(20. /255., 23. /255., 23. /255.)
                                ),
                                container(row![
                                    container(text("https://www.blockchain.com/explorer/transaction...").size(14).line_height(1.5).style(
                                        Color::from_rgb(42. /255., 47. /255., 53. /255.)
                                    )).width(Length::Fill),
                                    if state.is_tx_link_copied {
                                        mouse_area(Svg::from_path("assets/create-plan/blue_check.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)))
                                        .on_press(MyAppMessage::TxLinkCopyBtnPressed(state.transaction_link.clone()))
                                    } else {
                                        mouse_area(Svg::from_path("assets/create-plan/copy_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)))
                                        .on_press(MyAppMessage::TxLinkCopyBtnPressed(state.transaction_link.clone()))
                                    },
                                ].spacing(8).align_items(Alignment::Center).width(372)).style(
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
                            ].align_items(Alignment::Start).spacing(2),
                        ].spacing(26).padding(32).width(840).height(Length::Shrink)
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
                            text("BENEFICIARY").size(14).font(Font {
                                weight: font::Weight::Bold,
                                ..Font::DEFAULT
                            }),

                            container(
                                select_uxtos_component
                            ).style(
                                Appearance {
                                    text_color: Some(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                                    background: None,
                                    border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                    shadow: Shadow::default(),
                                }
                            )
                        ].spacing(24).padding(40).width(840).height(Length::Shrink).align_items(Alignment::Start)
                    ).style(
                        Appearance {
                            text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                            background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                            border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                            shadow: Shadow::default(),
                        }
                    ),
                    
                ].spacing(16).align_items(Alignment::Center),
                
            ].padding([10., 0., 80.0, 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0)))].spacing(192)).padding([0., 0., 0., 40.]),
        ].align_items(Alignment::Start)
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