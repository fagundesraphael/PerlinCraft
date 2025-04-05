use plotters::{chart::ChartBuilder, prelude::{BitMapBackend, IntoDrawingArea}, series::LineSeries, style::{BLACK, WHITE}};
use rand::Rng;

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn grad(p: i32, gradients: &[i32; 256]) -> i32 {
    gradients[(p as usize) % 256]
}


pub fn noise(p: f32, gradients: &[i32; 256]) -> f32 {
    let p0 = p.floor() as i32;
    let p1 = p0 + 1;

    let t = p - p0 as f32;
    let fade_t = fade(t);

    let g0 = grad(p0, gradients) as f32;
    let g1 = grad(p1, gradients) as f32;

    let d0 = g0 * (p - p0 as f32);
    let d1 = g1 * (p - p1 as f32);

    (1.0 - fade_t) * d0 + fade_t * d1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("perlin_noise_graph_1d.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Perlin Noise 1D", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, -4.2..4.2)?;

    chart.configure_mesh().draw()?;

    let mut gradients = [0; 256];
    let mut rng = rand::rng();
    (0..256).for_each(|i| {
        gradients[i] = if rng.random::<bool>() { 1 } else { -1 };
    });

    chart.draw_series(LineSeries::new(
        (0..1000).map(|i| {
            let x = i as f64 * 0.1;
            let y = noise(x as f32, &gradients) as f64;
            (x, y)
        }),
        &BLACK,
    ))?;

    Ok(())
}
