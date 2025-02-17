// States for the elements
use iced::widget::button;
// The elements to use
use iced::Element;
use iced::widget::Container;
use iced::widget::{Button, Column, Row, Text};
// Styling options
use iced::{alignment::Horizontal, Alignment, Length};
// The application settings
use iced::{Settings, Application, executor, Command};

extern crate time;

/// This is the state of the application. In here we store the current count.
#[derive(Debug, Clone, Copy)]
pub struct Calendar {
    day: u8,
    month: u8,
    year: i32,

    // Buttons handler that manipulate the counter
    // this is the local state of the buttons
    previous_month_button: button::State,
    next_month_button: button::State,
}

/// The events that will trigger changes on the calendar
#[derive(Debug, Clone, Copy)]
pub enum CalendarMessage {
    PreviousMonth,
    NextMonth,
}

impl Calendar {
    /// Default settings when starting the GUI
    pub fn new() -> Self {
        let now = time::OffsetDateTime::now_utc().date();
        let day = now.day();
        let month = now.month() as u8;
        let year = now.year();
        
        Self {
            day,
            month,
            year,
            previous_month_button: Default::default(),
            next_month_button: Default::default()
        }
    }

    // The view logic where we define the gui
    pub fn view(&mut self) -> Element<CalendarMessage> {
        let days = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let months = vec!["January", "February", "March", "April", "May", "June",
                          "July", "August", "September", "October", "November", "December"];

        let month = Text::new(months[(self.month as usize) - 1])
            .size(32)
            .style(iced::theme::Text::Color([0.6, 0.6, 0.6].into()));

        let year = Text::new(self.year.to_string())
            .size(32)
            .style(iced::theme::Text::Color([0.6, 0.6, 0.6].into()));

        // Create a header for the weekdays name
        let header: Row<CalendarMessage> = days
            .iter()
            .enumerate()
            .fold(Row::new().spacing(25), |row, (_i, day)| {
                row.push(
                    Column::new()
                        .push(
                            Text::new(day.to_string())
                                .size(25)
                        )
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                        .max_width(75)
                )
            });

        let mut calendar = Column::new()
        // Push the calendar month and year header
            .push(
                Row::new()
                    .push(
                        Row::new()
                            .push(month)
                            .push(year)
                            .spacing(12)
                            .width(Length::from(250))
                    )
                    .push(
                        Button::new(Text::new("<"))
                            .on_press(CalendarMessage::PreviousMonth)
                            .width(Length::from(65))
                    )
                    .push(
                        Button::new(Text::new(">"))
                            .on_press(CalendarMessage::NextMonth)
                            .width(Length::from(65))
                    )
                    .padding(12)
                    .spacing(12)
                    .width(Length::Fill)
        // Push the weekday header
            .push(header)
            .align_items(Alignment::Center));

        // Check the starting day of the selected month
        let month_start_day = time::Date::from_calendar_date(self.year, time::Month::try_from(self.month).unwrap(), 1)
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
                    
                    if time::Date::from_calendar_date(self.year, time::Month::try_from(self.month).unwrap(), day_count).is_ok() {
                        week = week.push(
                            Column::new()
                                .push(
                                    Text::new(day_count.to_string())
                                )
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                        )
                            .spacing(25);
                    } else {
                        week = week.push(
                            Column::new()
                                .push(
                                    Text::new("")
                                )
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                        )
                            .spacing(25);
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
                        .spacing(25);
                }
            }

            calendar = calendar.push(week);
        }
        
        Container::new(calendar)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    // The update logic
    pub fn update(&mut self, message: CalendarMessage) -> Command<CalendarMessage> {
        match message {
            CalendarMessage::PreviousMonth => {
                if self.month <= 1 {
                    self.month = 12;
                    self.year -= 1;
                } else {
                    self.month -= 1
                }
                Command::none()
            }
            CalendarMessage::NextMonth => {
                if self.month >= 12 {
                    self.month = 1;
                    self.year += 1;
                } else {
                    self.month += 1;
                }
                Command::none()
            }
        }
    }
}