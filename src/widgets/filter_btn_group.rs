use iced::border::Radius;
use iced::widget::container::Appearance;
use iced::widget::{button, row, container, text};
use iced::{theme, Alignment, Background, Border, Color, Element, Shadow, Theme};

use crate::messages::MyAppMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Owner,
    Beneficiary,
}


pub struct ButtonGroup {
    selected: Filter,
}

impl ButtonGroup {
    pub fn new(selected: Filter) -> Self {
        Self { selected }
    }

    pub fn view(&self) -> Element<'static, MyAppMessage> {
        let filters = [Filter::All, Filter::Owner, Filter::Beneficiary];

        let buttons = filters.iter().map(|&filter| {
            let is_selected = self.selected == filter;

            button(text(format!("{:?}", filter)))
                .padding([5, 15])
                .style(if is_selected {
                    iced::theme::Button::Custom(Box::new(SelectedButtonColor {}))
                } else {
                    iced::theme::Button::Custom(Box::new(UnSelectedButtonColor {}))
                })
                .on_press(MyAppMessage::FilterSelected(filter))
                .into()
        });

        container(row(buttons).align_items(Alignment::Center))
            .padding(4)
            .style(
                Appearance {
                    text_color: Some(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                    background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255. /255.))),
                    border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 100.0.into() },
                    shadow: Shadow::default(),
                }
            )
            .into()
    }
}

struct SelectedButtonColor {}

impl<> button::StyleSheet for SelectedButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(2./ 255., 84./ 255., 191. /255.))),
            border: Border::with_radius(Radius::from(100.0)),
            text_color: Color { r: 1., g: 1., b: 1., a: 100. },
            ..style.active(&theme::Button::Primary)
        }
    }
}

struct UnSelectedButtonColor {}

impl<> button::StyleSheet for UnSelectedButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border: Border::with_radius(Radius::from(0.0)),
            text_color: Color { r: 42. /255., g: 47. /255., b: 53. /255., a: 100. },
            ..style.active(&theme::Button::Primary)
        }
    }
}