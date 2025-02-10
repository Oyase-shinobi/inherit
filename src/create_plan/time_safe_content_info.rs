use crate::iced::{
    advanced::{
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
        Layout, Widget, Clipboard, Shell
    },
    Border, Color, Element, Length, Rectangle, Sandbox, Settings, Shadow, Size, Theme, Event, Alignment,
    mouse::{self as mouse_enum}
};

// #[derive(Debug, Clone)]
// enum PlanCardMessage {
//     SelectedPlan(bool)
// }

// type Pcm = PlanCardMessage

pub struct TimeSafeInfo<'a, Message, Renderer> {
    visible: bool,
    content: Element<'a, Message, Theme, Renderer>,
    message: Message
}

impl<'a, Message, Renderer> TimeSafeInfo<'a, Message, Renderer> {
    pub fn new(
        visible: bool,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        message: Message
    ) -> Self {
        Self { 
            visible,
            content: content.into(),
            message
        }
    }
}

impl<'a, Message: Clone, Renderer> Widget<Message, Theme, Renderer> for TimeSafeInfo<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);

    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits
    ) -> layout::Node {
        match tree.children.len() {
            0 => layout::Node::new(Size::new(1028., 91.)),
            _ => {
                let mut child_node = self.content.as_widget().layout(&mut tree.children[0], renderer, limits);
                let size_of_this_node;
                if self.visible == false {
                    size_of_this_node = Size::new(0., 0.);
                } else {
                    size_of_this_node = Size::new(1028., 91.);
                }
                child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);
                layout::Node::with_children(size_of_this_node, vec![child_node])
            }
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle
    ) {

        match self.visible {
            true => {
                renderer.fill_quad(
                    Quad {
                        bounds: layout.bounds(),
                        border: Border {
                            color: Color::from_rgb(250. / 255., 234. / 255., 171. / 255.),
                            width: 1.,
                            radius: 16.0.into(),
                        },
                        shadow: Shadow::default(),
                    },
                    Color::from_rgb(255. / 255., 249. / 255., 223.),
                );
                match state.children.len() {
                    0 => {},
                    _ => self.content.as_widget().draw(
                        &state.children[0],
                        renderer,
                        theme,
                        style,
                        layout.children().next().unwrap(),
                        cursor,
                        viewport
                    )
                }
            }

            false => {
            }
        }

    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(
                mouse_enum::Event::ButtonPressed(mouse_enum::Button::Left)
            ) => {
                // self.selected = !self.selected;
                // event::Status::Captured
                if cursor.is_over(layout.bounds()) {
                    self.visible = !self.visible;
                    shell.publish(self.message.clone());
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }

            _ => event::Status::Ignored,
        }
    }
}

impl<'a, Message, Renderer> From<TimeSafeInfo<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(widget: TimeSafeInfo<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}