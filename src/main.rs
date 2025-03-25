use draw::{render, Canvas, Color, Drawing, LineBuilder, Style, SvgRenderer};
use rand::Rng;

fn main() {
    let mut canvas = Canvas::new(400, 400);
    let mut rand = rand::rng();

    for i in 0..5 {
        let cordy = rand.random_range(i as f32..100.0);
        let cordx = rand.random_range(0.0..200.0);
        let length = rand.random_range(50.0..200.0);

        let line = LineBuilder::new(cordx, cordy)
            //.curve_to(50.0, 50.0, 20.0, 30.0)
            .line_to(cordx + length, cordy)
            .build();

        let drawing = Drawing::new()
            .with_shape(line)
            .with_style(Style::stroked(2, Color::black()));

        canvas.display_list.add(drawing);
    }

    render::save(&canvas, "img/grap.jpeg", SvgRenderer::new()).expect("Failed to save")
}
