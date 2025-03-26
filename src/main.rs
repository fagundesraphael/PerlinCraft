use draw::{render, Canvas, Color, Drawing, LineBuilder, Style, SvgRenderer};
use rand::Rng;

fn smoothed_noise(x: i32) -> f64 {
    (x % 10) as f64 / 10.0
}

fn linear_interpolation(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn interpolate_noise(x: f64) -> f64 {
    let integer_x = x as i32;
    let fractional_x = x - integer_x as f64;

    let v1 = smoothed_noise(integer_x);
    let v2 = smoothed_noise(integer_x + 1);

    linear_interpolation(v1, v2, fractional_x)
}

pub fn noise(x: i32) -> f64 {
    let x: i32 = (x << 13) ^ x;
    1.0 - (((x as i64 * (x as i64 * x as i64 * 15731 + 789221) + 1376312589) & 0x7FFFFFFF) as f64
        / 1073741824.0)
}

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
