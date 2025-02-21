use iced::{
    font, gradient::{ColorStop, Linear}, widget::{column, container, container::Appearance, row, text}, Alignment, Background, Border, Color, Element, Font, Gradient, Length, Shadow
};
use std::time::{Duration, SystemTime};

use crate::messages::MyAppMessage;

#[derive(Debug)]
pub struct CountdownTimer {
    target_time: SystemTime,
    remaining: Duration,
    title: &'static str,
    bg_color: &'static str
}

impl CountdownTimer {
    pub fn new(target_time: SystemTime, title: &'static str, bg_color: &'static str) -> Self {
        let now = SystemTime::now();
        let remaining = target_time.duration_since(now).unwrap_or(Duration::ZERO);
        Self { target_time, remaining, title, bg_color }
    }

    pub fn view(&self) -> Element<'static, MyAppMessage> {
        let (days, hours, minutes, seconds) = self.format_time();

        let time_display = row![
            column![text(format!("{:03}", days)).size(match self.bg_color {
                "blue" => 24,
                _ => 16
            }).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }).line_height(1.2), text("days").size(12).line_height(1.4)]
                .align_items(Alignment::Center),
            text(":").size(16),
            column![text(format!("{:02}", hours)).size(match self.bg_color {
                "blue" => 24,
                _ => 16
            }).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }).line_height(1.2), text("hours").size(12).line_height(1.4)]
                .align_items(Alignment::Center),
            text(":").size(16).line_height(1.4),
            column![text(format!("{:02}", minutes)).size(match self.bg_color {
                "blue" => 24,
                _ => 16
            }).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }).line_height(1.2), text("mins").size(12).line_height(1.4)]
                .align_items(Alignment::Center),
            text(":").size(16).line_height(1.4),
            column![text(format!("{:02}", seconds)).size(match self.bg_color {
                "blue" => 24,
                _ => 16
            }).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }).line_height(1.2), text("secs").size(12).line_height(1.4)]
                .align_items(Alignment::Center),
        ]
        .align_items(Alignment::Center).spacing(5);

        container(column![text(self.title).size(match self.bg_color {
            "blue" => 14,
            _ => 12
        }).line_height(match self.bg_color {
            "blue" => 1.5,
            _ => 1.4
        }), time_display]
            .spacing(4).align_items(Alignment::Center)
            )
            .padding(match self.bg_color {
                "blue" => 22.5,
                _ => 10.
            })
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(
                match self.bg_color {
                    "purple" => Appearance {
                        text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                        background: Some(Background::Gradient(Gradient::Linear(Linear {
                            angle: 0.778.into(),
                            stops: [
                                Some(ColorStop {
                                    offset: 0.0,
                                    color: Color {
                                        r: 53.0 / 255.0,
                                        g: 131.0 / 255.0,
                                        b: 255.0 / 255.0,
                                        a: 100.0,
                                    },
                                }),
                                Some(ColorStop {
                                    offset: 0.951,
                                    color: Color {
                                        r: 150.0 / 255.0,
                                        g: 159.0 / 255.0,
                                        b: 255.0 / 255.0,
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
                        border: Border {
                            color: Color::from_rgba(25. / 255., 71. / 255., 254. / 255., 0.3),
                            width: 1.0,
                            radius: 16.0.into()
                        },
                        shadow: Shadow::default()
                    },
                    "gold" => {
                        Appearance {
                            text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                            background: Some(Background::Gradient(Gradient::Linear(Linear {
                                angle: 0.778.into(),
                                stops: [
                                    Some(ColorStop {
                                        offset: 0.0,
                                        color: Color {
                                            r: 248.0 / 255.0,
                                            g: 116.0 / 255.0,
                                            b: 28.0 / 255.0,
                                            a: 100.0,
                                        },
                                    }),
                                    Some(ColorStop {
                                        offset: 0.951,
                                        color: Color {
                                            r: 255.0 / 255.0,
                                            g: 203.0 / 255.0,
                                            b: 13.0 / 255.0,
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
                            border: Border {
                                color: Color::from_rgba(255. / 255., 134. / 255., 53. / 255., 0.3),
                                width: 1.0,
                                radius: 16.0.into()
                            },
                            shadow: Shadow::default()
                        }
                    },
                    "blue" => Appearance {
                        text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                        background: Some(Background::Gradient(Gradient::Linear(Linear {
                            angle: 90.0.into(),
                            stops: [
                                Some(ColorStop {
                                    offset: 0.0733,
                                    color: Color {
                                        r: 53.0 / 255.0,
                                        g: 229.0 / 255.0,
                                        b: 171.0 / 255.0,
                                        a: 1.,
                                    },
                                }),
                                Some(ColorStop {
                                    offset: 0.5191,
                                    color: Color {
                                        r: 62.0 / 255.0,
                                        g: 112.0 / 255.0,
                                        b: 253.0 / 255.0,
                                        a: 1.,
                                    },
                                }),
                                Some(ColorStop {
                                    offset: 0.939,
                                    color: Color {
                                        r: 135.0 / 255.0,
                                        g: 85.0 / 255.0,
                                        b: 241.0 / 255.0,
                                        a: 1.,
                                    },
                                }),
                                None, None, None, None, None,
                            ]
                        }))),
                        border: Border {
                            color: Color::from_rgba(25. / 255., 71. / 255., 254. / 255., 0.3),
                            width: 1.0,
                            radius: 16.0.into()
                        },
                        shadow: Shadow::default()
                    },
                    &_ => todo!()
                }
                )
            .into()
    }

    fn format_time(&self) -> (u64, u64, u64, u64) {
        let total_secs = self.remaining.as_secs();
        let days = total_secs / 86_400;
        let hours = (total_secs % 86_400) / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        (days, hours, minutes, seconds)
    }

    pub fn update(&mut self) {
        let now = SystemTime::now();
        self.remaining = self.target_time.duration_since(now).unwrap_or(Duration::ZERO);
    }
}
