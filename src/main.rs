use iced::{
    self, Sandbox, Settings, Length, Font, Color, Background, Border, Shadow, Padding, Alignment, Gradient, theme, window, Size,
    widget::{container, container::Appearance, Svg, column, row, text, horizontal_rule, text::Shaping, button, Theme, mouse_area, text_input, scrollable::{Direction, Properties, Viewport}, Scrollable, toggler},
    font,
    alignment::Vertical,
    gradient::{Linear, ColorStop},
    border::Radius
};

mod create_plan;
use crate::create_plan::select_plan::PlanCard;

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 1440.,
                height: 920.,
            },
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

enum Page {
    FirstCreateNewPlanPage,
    SecondCreateNewPlanPage,
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    // GoToSelectPlanType(SelectPlanTypeMessage),
    // GoToSetupPlan(SetupPlanTypeMessage),
    TimeSafePressed,
    FailSafePressed,
    AlertCloseBtnPressed,
    GoToSecondCreateNewPlanBtnPressed,
    PlanNameContentChanged(String),
    TogglerUxtos(bool)
}

struct MyApp {
    time_safe_selected: bool,
    fail_safe_selected: bool,
    time_safe_alert_visible: bool,
    current_page: Page,
    plan_name: String,
    is_selected_uxtos: bool
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        // Self { page: Page::A }
        Self {
            time_safe_selected: false,
            fail_safe_selected: false,
            time_safe_alert_visible: false,
            current_page: Page::FirstCreateNewPlanPage,
            plan_name: "".to_string(),
            is_selected_uxtos: false
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        // self.page = match message {
        //     MyAppMessage::GoToBButtonPressed => Page::B,
        //     MyAppMessage::GoToAButtonPressed => Page::A,
        // }
        match message {
            MyAppMessage::TimeSafePressed => {
                self.time_safe_selected = !self.time_safe_selected;
                if self.time_safe_selected {
                    self.time_safe_alert_visible = true;
                } else {
                    self.time_safe_alert_visible = false;
                }
                self.fail_safe_selected = false;
            }
            MyAppMessage::AlertCloseBtnPressed => {
                self.time_safe_alert_visible = false;
            }
            MyAppMessage::GoToSecondCreateNewPlanBtnPressed => {
                self.current_page = Page::SecondCreateNewPlanPage
            }
            MyAppMessage::PlanNameContentChanged(content) => {
                self.plan_name = content;
            }
            MyAppMessage::TogglerUxtos(is_checked) => {
                self.is_selected_uxtos = is_checked;
            }
            _ => {
                self.time_safe_selected = false;
                self.fail_safe_selected = !self.fail_safe_selected;
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        
        // .into()
        // PlanCard::new().into()
        // container(PlanCard::new()).into()
        let time_safe_content = container(column![
            Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
            column![
                text("Time Safe").size(16).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                }),
                text("Lock Bitcoin until a future date").size(14).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
            ].spacing(6.),
            column![
                text("Protected against").size(12).style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
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
                ].spacing(8)
            ].spacing(8),
            horizontal_rule(0),
            column![
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Time-locked access").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Keeps your funds securely locked until a specific future date, so they can’t be accessed early")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Unbreakable protection").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Ensures your funds are protected with advanced safeguards, so only the right person can claim them at the right time")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Claimable funds after expiry").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows you to claim the funds after the lock period ends")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.)
            ].spacing(15)
        ].spacing(24.)).width(438.);

        let fail_safe_content = container(column![
            Svg::from_path("assets/create-plan/guard.svg").width(Length::Fixed(40.)).height(Length::Fixed(40.)),
            column![
                text("Fail Safe Recovery").size(16).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                }),
                text("A dead man’s switch transferring assets on missed check-ins")
                .size(14)
                .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
            ].spacing(6.),
            column![
                text("Protected against")
                .size(12)
                .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                row![
                    container(row![
                            Svg::from_path("assets/create-plan/checks.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                            text("Private key loss").size(12)
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
                            text("Unexpected life events").size(12)
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
                ].spacing(8)
            ].spacing(8),
            horizontal_rule(0),
            column![
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Backup access plan").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Ensures your BTC can be recovered through predefined steps if you lose access to your private keys")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Time-based status check-ins").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows your beneficiary to recover BTC if you don’t confirm your status before the check-in period ends")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.),
                row![
                    Svg::from_path("assets/create-plan/colored-check.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                    column![
                        text("Beneficiary designation").size(12).font(Font {
                            weight: font::Weight::Medium,
                            ..Font::DEFAULT
                        }),
                        text("Allows you to assign trusted beneficiaries to claim your BTC if you miss a status check-in")
                        .size(12)
                        .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                    ]
                ].spacing(8.)
            ].spacing(15)
        ].spacing(24.)).width(438.);
        let dyn_space = match self.time_safe_alert_visible {
            true => 24,
            false => 0
        };
        let time_safe_info = match self.time_safe_alert_visible {
            true => row![
                row![
                    Svg::from_path("assets/create-plan/info.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                    column![
                        text("Dual protection guide")
                            .size(14)
                            .font(Font {
                                weight: font::Weight::Bold,
                                ..Font::DEFAULT
                        }).style(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                        text("To use both protections, create a fail safe first, then a time safe. Fail safes guard against key loss; time safes protect against advanced threats")
                            .size(12.5)
                            .style(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.))
                            .font(Font {
                                weight: font::Weight::Medium,
                                ..Font::DEFAULT
                        })
                    ].spacing(4)              
                ].spacing(16).align_items(Alignment::Start),
                mouse_area(
                    Svg::from_path("assets/create-plan/cross_btn.svg").width(Length::Fixed(24.)).height(Length::Fixed(24.)),
                ).on_press(MyAppMessage::AlertCloseBtnPressed)
            ].spacing(150).width(1028).padding([16, 16, 16, 35]).align_items(Alignment::Start),
            false => row!("").height(0)
        };
        match self.current_page {        
            Page::FirstCreateNewPlanPage => container(column![
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
                            Svg::from_path("assets/create-plan/selected_1.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Select plan type").size(14),
                        ].spacing(8),
                        Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/unselected_2.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Setup plan").size(14),
                        ].spacing(8),
                        Svg::from_path("assets/create-plan/arrow.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
                        row![
                            Svg::from_path("assets/create-plan/unselected_3.svg").width(Length::Fixed(18.)).height(Length::Fixed(18.)),
                            text("Review and confirm").size(14),
                        ].spacing(8),
                    ].spacing(16.).align_items(Alignment::Center).width(508.).height(21.),
                    column![
                        row![
                            PlanCard::new(self.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
                            PlanCard::new(self.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
                        ].spacing(24),
                        container(time_safe_info).style(
                            Appearance {
                                text_color: Some(Color::from_rgb(113. / 255., 121. / 255., 142. / 255.)),
                                background: Some(Background::Color(Color::from_rgb(255./ 255., 249./ 255., 223.))),
                                border: Border { color: Color::from_rgb(250. / 255., 234. / 255., 171. / 255.), width: 1., radius: 16.0.into() },
                                shadow: Shadow::default(),
                            }
                        )
                    ].spacing(dyn_space).align_items(Alignment::Center),
                    button("Continue").style(
                        theme::Button::Custom(Box::new(ButtonColor {}))
                        // Color::from_rgb(255., 0., 0.)
                    ).padding([12., 83.]).on_press(MyAppMessage::GoToSecondCreateNewPlanBtnPressed)
                ].padding([10., 0., 0., 0.]).align_items(Alignment::Center).spacing(24)).height(920).direction(Direction::Vertical(Properties::new())),
            ].align_items(Alignment::Center)) 
        .style(Appearance {
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
        }).width(Length::Fill).height(Length::Fill).center_x().into(),
            Page::SecondCreateNewPlanPage => container(
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
                                        text_input("Enter plan’s name", &self.plan_name).style(
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
                                            toggler(None, self.is_selected_uxtos, MyAppMessage::TogglerUxtos).size(24)
                                        ].spacing(12).align_items(Alignment::Center)
                                    ].spacing(512).align_items(Alignment::Center),
                                    
                                    container(column![
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
                                                Svg::from_path("assets/create-plan/info_tooltip.svg").width(Length::Fixed(16.)).height(Length::Fixed(16.)),
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
                                    ].padding([20.0, 24.0]).spacing(16).width(760).align_items(Alignment::Start)).style(
                                        Appearance {
                                            text_color: Some(Color::from_rgb(0. / 255., 0. / 255., 0. / 255.)),
                                            background: None,
                                            border: Border { color: Color::from_rgb(205. / 255., 220. / 255., 241. / 255.), width: 1., radius: 16.0.into() },
                                            shadow: Shadow::default(),
                                        }
                                    ),
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

                                    ].spacing(16).align_items(Alignment::Center)
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
                        button("Continue").style(
                            theme::Button::Custom(Box::new(ButtonColor {}))
                            // Color::from_rgb(255., 0., 0.)
                        ).padding([12., 83.]).on_press(MyAppMessage::GoToSecondCreateNewPlanBtnPressed)
                    ].padding([10., 0., 0., 0.]).align_items(Alignment::Center).spacing(24)).height(1080).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0))),
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
            }).width(Length::Fill).height(Length::Fill).center_x().into(),
        }
        // row![
        //     PlanCard::new(self.time_safe_selected, time_safe_content, MyAppMessage::TimeSafePressed),
        //     PlanCard::new(self.fail_safe_selected, fail_safe_content, MyAppMessage::FailSafePressed),
        // ].spacing(24).into()


        // PlanCard::new(fail_safe_content).into()
    }
}

struct ButtonColor {}

impl<> button::StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Gradient(Gradient::Linear(Linear {
                angle: 0.778.into(),
                stops: [
                    Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            r: 4.0 / 255.0,
                            g: 47.0 / 255.0,
                            b: 104.0 / 255.0,
                            a: 100.0,
                        },
                    }),
                    Some(ColorStop {
                        offset: 0.951,
                        color: Color {
                            r: 2.0 / 255.0,
                            g: 84.0 / 255.0,
                            b: 191.0 / 255.0,
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
            border: Border::with_radius(Radius::from(12.0)),
            text_color: Color { r: 1., g: 1., b: 1., a: 100. },
            ..style.active(&theme::Button::Primary)
        }
    }
}

struct CustomTextInputStyle {}

impl<> text_input::StyleSheet for CustomTextInputStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(1.0, 1.0, 1.0).into(),  
            border: Border { color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }, width: 1.0, radius: Radius::from([10.0, 10.0, 10.0, 10.0]) },  // White background when focused
            icon_color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() } // White background when focused
                   // Gray border
        }
    }

    fn focused(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(1.0, 1.0, 1.0).into(),  
            border: Border { color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }, width: 1.0, radius: Radius::from([10.0, 10.0, 10.0, 10.0]) },  // White background when focused
            icon_color: Color { r: 236. /255., g: 238. /255., b: 242. /255., a: 100.0.into() }
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(158. /255., 168. /255., 190. /255.) // Gray placeholder text
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.2, 0.2, 0.2) // Dark text color
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.8, 0.9, 1.0) // Light blue selection color
    }

    fn disabled_color(&self, _: &<Self as iced::widget::text_input::StyleSheet>::Style) -> iced::Color {
        Color::from_rgb(0.2, 0.2, 0.2)
    }

    fn disabled(&self, _: &<Self as iced::widget::text_input::StyleSheet>::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(0.9, 0.9, 0.9).into(),
            border: Border { color: Color::from_rgba(0.7, 0.7, 0.7, 0.5), width: 1.0, radius: Radius::from([5.0, 5.0, 5.0, 5.0]) },
            icon_color: Color::from_rgba(0.7, 0.7, 0.7, 0.5),
        }
    }
}