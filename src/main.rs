mod system;

use system::shader::FractalWidget;

fn main() -> iced::Result {
    iced::run(
        "Fractal Shader",
        FractalWidget::update,
        FractalWidget::view,
    )
}
