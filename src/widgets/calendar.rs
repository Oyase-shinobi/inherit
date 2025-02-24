
use iced::widget::container::Appearance;
use iced::{Background, Border, Color, Element, Shadow};
use iced::widget::{container, mouse_area, Container, Svg};
use iced::widget::{Column, Row, Text};
// Styling options
use iced::{Alignment, Length};

use crate::messages::MyAppMessage;
use crate::state::State;

extern crate time;

/// This is the state of the application. In here we store the current count.
#[derive(Debug, Clone, Copy)]
pub struct Calendar {
    today: u8
}



impl Calendar {
    /// Default settings when starting the GUI
    pub fn new() -> Self {
        let now = time::OffsetDateTime::now_utc().date();
        let today = now.day();
        Self {
            today
        }
    }

    // The view logic where we define the gui
    pub fn view(&self, state: &State) -> Element<'static, MyAppMessage> {
        let days = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let months = vec!["January", "February", "March", "April", "May", "June",
                          "July", "August", "September", "October", "November", "December"];

        let month = Text::new(months[(state.month as usize) - 1])
            .size(14)
            .line_height(1.5)
            .style(iced::theme::Text::Color([9. /255., 8. /255., 20. /255.].into()));

        let year = Text::new(state.year.to_string())
            .size(14)
            .line_height(1.5)
            .style(iced::theme::Text::Color([9. /255., 8. /255., 20. /255.].into()));

        // Create a header for the weekdays name
        let header: Row<MyAppMessage> = days
            .iter()
            .enumerate()
            .fold(Row::new().spacing(21), |row, (_i, day)| {
                row.push(
                    Column::new()
                        .push(
                            Text::new(day.to_string())
                                .size(12)
                        )
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                )
            });

        let mut calendar = Column::new()
        // Push the calendar month and year header
            .push(
                Row::new()
                    .push(
                        mouse_area(Svg::from_path("assets/create-plan/left_arrow.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)))
                            .on_press(MyAppMessage::PreviousMonth)
                    )
                    .push(
                        container(Row::new()
                            .push(month)
                            .push(year)
                            .spacing(12)
                            .width(Length::Shrink)).width(Length::Fill)
                    )
                    .push(
                        mouse_area(Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                    )
                            .on_press(MyAppMessage::NextMonth)
                            
                    )
                    .padding([8.0, 0.])
                    .spacing(35)
                    .width(Length::Fill)
        // Push the weekday header
        .align_items(Alignment::Center)).push(
            container(Row::new().height(1).width(Length::Fill)).style(
                Appearance {
                    text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                    background: Some(Background::Color(Color::from_rgb(2./ 255., 84./ 255., 191. /255.))),
                    border: Border { color: Color::from_rgb(113. / 255., 121. / 255., 142. / 255.), width: 1., radius: 0.0.into() },
                    shadow: Shadow::default(),
                }
            )
        ).push(header).align_items(Alignment::Center);
        

        // Check the starting day of the selected month
        let month_start_day = time::Date::from_calendar_date(state.year, time::Month::try_from(state.month).unwrap(), 1)
            .unwrap()
            .weekday()
            .number_from_sunday();

        // Keep track of the current day
        let mut day_count = 0;
        
        // Iterate over 6 weeks - months won't ocupy more than 6 weeks
        for _ in 0..6 {
            let mut week = Row::new();

            // Iterate over all the weekdays, starting from sunday
            for weekday_num in 1..8 {
                if month_start_day == weekday_num || day_count >= 1
                {
                    day_count += 1;
                    
                    if time::Date::from_calendar_date(state.year, time::Month::try_from(state.month).unwrap(), day_count).is_ok() {
                        week = week.push(
                            Column::new()
                                .push(
                                    mouse_area(container(Text::new(day_count.to_string()).size(14)).style(
                                        match state.day == day_count {
                                            false => Appearance {
                                                text_color: Some(Color::from_rgb(9. / 255., 8. / 255., 20. / 255.)),
                                                background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255.))),
                                                border: match day_count == self.today {
                                                    true => Border { color: Color::from_rgb(217. / 255., 219. / 255., 225. / 255.), width: 2., radius: 3.0.into() },
                                                    false => Border { color: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.), width: 1., radius: 6.0.into() }
                                                },
                                                shadow: Shadow::default(),
                                            },
                                            true => Appearance {
                                                text_color: Some(Color::from_rgb(255. / 255., 255. / 255., 255. / 255.)),
                                                background: Some(Background::Color(Color::from_rgb(2./ 255., 84./ 255., 191. /255.))),
                                                border: Border { color: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.), width: 1., radius: 6.0.into() },
                                                shadow: Shadow::default(),
                                            }
                                        }
                                        
                                    ).width(22).height(22)
                                    .center_x()
                                    .center_y()).on_press(MyAppMessage::SelectDay(day_count))
                                )
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                        )
                            .spacing(10);
                    } else {
                        week = week.push(
                            Column::new()
                                .push(
                                    Text::new("")
                                )
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                        )
                            .spacing(10);
                    }
                } else {
                    week = week.push(
                        Column::new()
                            .push(
                                Text::new("")
                            )
                            .align_items(Alignment::Center)
                            .width(Length::Fill)
                    )
                        .spacing(10);
                }
            }

            calendar = calendar.push(week);
        }
        
        Container::new(calendar)
            .width(243)
            .height(Length::Shrink)
            .center_x()
            .center_y()
            .padding([8., 17.5])
            .style(
                Appearance {
                    text_color: None,
                    background: None,
                    border: Border { color: Color::from_rgb(236. / 255., 238. / 255., 242. / 255.), width: 1., radius: 10.0.into() },
                    shadow: Shadow::default(),
                }
            )
            .into()
    }

}