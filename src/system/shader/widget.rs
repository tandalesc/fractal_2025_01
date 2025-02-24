use glam::Vec2;
use iced::{
    Alignment, Element, Length, Rectangle,
    advanced::Shell,
    event::Status,
    mouse::{self, Cursor},
    widget::{column, row, shader, shader::Event, slider, text},
};

use super::{
    constants::*,
    controls::{Controls, Message, MouseInteraction},
    pipeline::FragmentShaderPrimitive,
};

struct FragmentShaderProgram {
    controls: Controls,
}

impl FragmentShaderProgram {
    fn new() -> Self {
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
        if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
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
        }

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

        (Status::Ignored, None)
    }
}

pub struct FractalWidget {
    program: FragmentShaderProgram,
}

fn control_group<'a>(
    label: &'static str,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![text(label), control.into()].spacing(10).into()
}

impl Default for FractalWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl FractalWidget {
    pub fn new() -> Self {
        Self {
            program: FragmentShaderProgram::new(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let controls = row![
            control_group(
                "Max Iterations",
                slider(
                    ITERS_MIN..=ITERS_MAX,
                    self.program.controls.max_iter,
                    move |iter| { Message::UpdateMaxIterations(iter) }
                )
                .width(Length::Fill)
            ),
            control_group(
                "Zoom",
                slider(
                    ZOOM_MIN..=ZOOM_MAX,
                    self.program.controls.zoom,
                    move |zoom| { Message::UpdateZoom(zoom) }
                )
                .step(0.01)
                .width(Length::Fill)
            ),
        ]
        .padding(10)
        .spacing(10);

        let shader = shader(&self.program)
            .width(Length::Fill)
            .height(Length::Fill);

        column![shader, controls]
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateMaxIterations(max_iter) => {
                self.program.controls.max_iter = max_iter;
            }
            Message::UpdateZoom(zoom) => {
                self.program.controls.zoom = zoom;
            }
            Message::PanningDelta(delta) => {
                self.program.controls.center -= 2.0 * delta * self.program.controls.scale();
            }
            Message::ZoomDelta(pos, bounds, delta) => {
                let delta = delta * ZOOM_WHEEL_SCALE;
                let prev_scale = self.program.controls.scale();
                let prev_zoom = self.program.controls.zoom;
                self.program.controls.zoom = (prev_zoom + delta).max(ZOOM_MIN).min(ZOOM_MAX);

                let vec = pos - Vec2::new(bounds.width, bounds.height) * 0.5;
                let new_scale = self.program.controls.scale();
                self.program.controls.center += vec * (prev_scale - new_scale) * 2.0;
            }
        }
    }
}
