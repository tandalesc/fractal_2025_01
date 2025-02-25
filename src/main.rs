mod fractal;

use crate::fractal::widget::FractalWidget;

fn main() -> iced::Result {
    iced::run(
        "Fractal Shader",
        FractalWidget::update,
        FractalWidget::view,
    )
}
