use glam::Vec2;
use iced::widget::shader::{self, Event};
use iced::{Rectangle, mouse};

use iced::{advanced::Shell, event::Status, mouse::Cursor};

use super::controls::{Controls, Message, MouseInteraction};
use super::shader_primitive::FragmentShaderPrimitive;

pub struct FragmentShaderProgram {
    pub controls: Controls,
}

impl FragmentShaderProgram {
    pub fn new() -> Self {
        Self {
            controls: Controls::default(),
        }
    }
}

impl shader::Program<Message> for FragmentShaderProgram {
    type State = MouseInteraction;
    type Primitive = FragmentShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        FragmentShaderPrimitive::new(self.controls)
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
        _shell: &mut Shell<'_, Message>,
    ) -> (Status, Option<Message>) {
        // try capturing mouse wheel scrolled event
        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    let pos = Vec2::new(pos.x, pos.y);
                    let delta = match delta {
                        mouse::ScrollDelta::Lines { x: _, y } => y,
                        mouse::ScrollDelta::Pixels { x: _, y } => y,
                    };
                    return (
                        Status::Captured,
                        Some(Message::ZoomDelta(pos, bounds, delta)),
                    );
                }
            },
            _ => {}
        };
        // if not scrolling, look for button presses for panning behavior
        match state {
            MouseInteraction::Idle => match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if let Some(pos) = cursor.position_over(bounds) {
                        *state = MouseInteraction::Panning(Vec2::new(pos.x, pos.y));
                        return (Status::Captured, None);
                    }
                }
                _ => {}
            },
            MouseInteraction::Panning(prev_pos) => match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    *state = MouseInteraction::Idle;
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    let pos = Vec2::new(position.x, position.y);
                    let delta = pos - *prev_pos;
                    *state = MouseInteraction::Panning(pos);
                    return (Status::Captured, Some(Message::PanningDelta(delta)));
                }
                _ => {}
            },
        };
        // else ignore the event
        (Status::Ignored, None)
    }
}
