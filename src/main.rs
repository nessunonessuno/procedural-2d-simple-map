use nannou::color::SKYBLUE;
use nannou::App;
use nannou::prelude::Update;
use nannou::Frame;
use procedural_2d_landscape::{generate_n_octave_perlin_noise, return_color_from_noise, Model};
use std::time::Duration;
use nannou::noise::Perlin;

const CHUNK_SIZE: f32 = 10.0;
const MS_TO_SLEEP: u64 = 300;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App,) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    let noise = Perlin::new();
    let seed: f64 = 0.0;

    Model { window, noise, seed }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.seed += 1.0;
    std::thread::sleep(Duration::from_millis(MS_TO_SLEEP));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SKYBLUE);
    let window_rect = app.window_rect();

    for x in (window_rect.left() as i32..window_rect.right() as i32).step_by(CHUNK_SIZE as usize) {
        for y in (window_rect.bottom() as i32..window_rect.top() as i32).step_by(CHUNK_SIZE as usize) {
            let x_f = x as f64 * 0.005;
            let y_f = y as f64 * 0.005;

            let noise_val = generate_n_octave_perlin_noise(8, x_f, y_f, model.seed, model.noise);
            let color = return_color_from_noise(noise_val);

            draw.rect()
                .x_y(x as f32 + CHUNK_SIZE / 2.0, y as f32 + CHUNK_SIZE / 2.0)
                .w_h(CHUNK_SIZE, CHUNK_SIZE)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

