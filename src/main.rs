use draw::{render, Canvas, Color, Drawing, LineBuilder, Style, SvgRenderer};

fn main() {
    let mut canvas = Canvas::new(800, 800);

    let line = LineBuilder::new(10.0, 10.0)
        .curve_to(50.0, 50.0, 20.0, 30.0)
        .line_to(90.0, 90.0)
        .build();

    let drawing = Drawing::new()
        .with_shape(line)
        .with_style(Style::stroked(2, Color::black()));

    canvas.display_list.add(drawing);

    render::save(&canvas, "img/grap.jpeg", SvgRenderer::new()).expect("Failed to save")
}
