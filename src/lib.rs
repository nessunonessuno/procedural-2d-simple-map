use nannou::color::rgb;
use nannou::math::map_range;
use nannou::color::rgb::Rgb;
use nannou::window;
use nannou::noise::NoiseFn;
use nannou::noise::Perlin;

pub struct Model {
    pub window: window::Id,
    pub noise: Perlin,
    pub seed: f64,
}

#[inline]
pub fn generate_n_octave_perlin_noise(n: u64, x: f64, y: f64, seed: f64, noise: Perlin) -> f64{
	let mut noise_val = 0.0;
	let mut amplitude = 1.0;
	let mut frequency = 1.0;

	for _i in 0..n{
	   noise_val += amplitude * noise.get([x * frequency, y * frequency, seed]);
	   amplitude = amplitude / 2.0;
	   frequency = frequency * 2.0;
	} 
	noise_val
}

#[inline]
pub fn return_color_from_noise(noise_val: f64) -> Rgb{
	if noise_val > 0.9 {
		// High elevations: rock
		rgb(0.47, 0.47, 0.47)
	} else if noise_val > -0.2{
		// Middle-high elevations: land/grass
        let g = map_range(noise_val, 0.5, 0.8, 0.7, 1.0);
        rgb(0.0, g as f32, 0.0)
    } else if noise_val > -0.6 {
        // Low elevations: sand
        rgb(0.76, 0.7, 0.5)
    } else {
        // Lowest elevation: water
        let b = map_range(noise_val, -1.0, 0.2, 0.5, 1.0);
        rgb(0.0, 0.0, b as f32)
    }
}