use iced::{
    self, font, widget::{column, container, container::Appearance, row, text, Svg}, Alignment, Background, Border, Color, Element, Font, Length, Shadow
};

use crate::messages::MyAppMessage;

#[allow(dead_code)]
pub struct BalanceCard {
    icon_path: &'static str,
    title: &'static str,
    balance: f64,
}


impl BalanceCard {
    pub fn new(icon_path: &'static str, title: &'static str, balance: f64) -> Self {
        Self { icon_path, title, balance }
    }

    pub fn view(&self) -> Element<'static, MyAppMessage> {
        let icon = Svg::from_path(self.icon_path)
            .width(Length::Fixed(40.))
            .height(Length::Fixed(40.));

        let title =  text(self.title).size(16).font(Font {
            weight: font::Weight::Medium,
            ..Font::DEFAULT
        }).line_height(1.5).style(Color::from_rgb(113. /255., 121. /255., 142. /255.)).width(Length::Fill);

        let balance = text(format!("{:.2} BTC", self.balance))
            .size(20).font(Font {
            weight: font::Weight::ExtraBold,
            ..Font::DEFAULT
        }).line_height(1.2).style(Color::from_rgb(20. /255., 23. /255., 23. /255.)).width(Length::Fill);

        let content = column![title, balance].spacing(4).align_items(Alignment::Start).width(200.).height(52.);
        let row = row![icon, content].spacing(24).align_items(Alignment::Center);

        container(row)
            .padding([24., 32.])
            .width(328)
            .height(100)
            .style(
                Appearance {
                    text_color: Some(Color::from_rgb(113. /255., 121. /255., 142. /255.)),
                    background: Some(Background::Color(Color::from_rgb(255./ 255., 255./ 255., 255. /255.))),
                    border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 32.0.into() },
                    shadow: Shadow::default(),
                }
            )
            .into()
    }
}