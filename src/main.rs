use ::rand::{thread_rng, Rng};
use macroquad::{
	color::colors,
	prelude::*,
	window::{screen_height, screen_width},
};

struct Trace {
	middle_point: Vec2,
	weather: Vec2,
}
impl Trace {
	fn new(middle_point: Vec2, weather: Vec2) -> Trace {
		Trace {
			middle_point,
			weather,
		}
	}

	fn update(&mut self) {
		self.middle_point -= self.weather * 3.0;
	}

	fn draw(&self) {
		let start = self.middle_point - self.weather * 2.5;
		let end = self.middle_point + self.weather * 2.5;

		draw_line(start.x, start.y, end.x, end.y, 2.0, colors::WHITE);
	}
}

#[derive(Clone)]
struct Bot {
	radius: f32,
	color: Color,
	pos: Vec2,
	vel: Vec2,
}
impl Bot {
	fn mutate(&self) -> Self {
		let mut rnd = thread_rng();
		let mut color = self.color;
		color.r = clamp(color.r + rnd.gen_range(-0.02..0.02), 0.0, 1.0);
		color.g = clamp(color.g + rnd.gen_range(-0.02..0.02), 0.0, 1.0);
		color.b = clamp(color.b + rnd.gen_range(-0.02..0.02), 0.0, 1.0);

		let mut vel = self.vel;
		vel.x += rnd.gen_range(-1.0..1.0);
		vel.y += rnd.gen_range(-1.0..1.0);

		let mut radius = self.radius;
		radius = clamp(radius + rnd.gen_range(-1.0..1.0), 2.5, 15.0);

		Self {
			pos: self.pos,
			radius,
			color,
			vel,
		}
	}
}

struct Dish {
	dish_radius: f32,
	bots: Vec<Bot>,
	wind: Vec2,
	center: Vec2,
	traces: Vec<Trace>,
	bot_lines: bool,
}
impl Dish {
	fn new() -> Self {
		let bots = Vec::new();
		let dish_radius = f32::min(screen_width(), screen_height()) / 2.0;
		let center = vec2(dish_radius, dish_radius);
		let weather = vec2(0.0, 0.0);
		let traces = Vec::new();
		Self {
			bots,
			dish_radius,
			center,
			wind: weather,
			traces,
			bot_lines: false,
		}
	}

	fn draw_bots(&self) {
		for bot in &self.bots {
			draw_circle(bot.pos.x, bot.pos.y, bot.radius, bot.color);
			if self.bot_lines {
				draw_line(
					bot.pos.x,
					bot.pos.y,
					bot.pos.x + bot.vel.x + 10.0 * (0.5 - self.wind.x) as f32,
					bot.pos.y + bot.vel.y + 10.0 * (0.5 - self.wind.y) as f32,
					2.0,
					Color::from_rgba(155, 0, 255, 255),
				);
			}
		}
	}

	fn draw_traces(&self) {
		for trace in &self.traces {
			trace.draw();
		}
	}

	fn draw(&self) {
		self.draw_bots();
		self.draw_traces()
	}

	fn wind(&mut self, (dx, dy): (f64, f64)) {
		self.wind = vec2(dx as f32, dy as f32);
	}

	fn update_traces(&mut self) {
		let mut rnd = thread_rng();

		let trace_pos = vec2(
			rnd.gen_range(0.0..screen_width()),
			rnd.gen_range(0.0..screen_height()),
		);
		let trace = Trace::new(trace_pos, self.wind);
		self.traces.push(trace);

		// Remove the oldest trace if there are more than 100 traces
		if self.traces.len() > 100 {
			self.traces.remove(0);
		}

		// actual update
		for trace in &mut self.traces {
			trace.update();
		}
	}

	fn update_bots(&mut self) {
		// check if there are any bots left
		if self.bots.is_empty() {
			self.bots.push(Bot {
				radius: 5.0,
				color: Color::new(0.5, 0.5, 0.5, 1.0),
				pos: self.center,
				vel: vec2(0.0, 0.0),
			})
		}

		let num_bots = self.bots.len();
		let mut rnd = thread_rng();
		let mut i = 0;

		// evolution algorithm
		while i < self.bots.len() {
			// if (self.bots[i].pos - self.center).length() > self.dish_radius {
			let lol = vec2(screen_width(), screen_height());
			let lol2 = self.bots[i].pos / lol;
			if lol2.floor() != vec2(0.0, 0.0) {
				self.bots.swap_remove(i);
				continue;
			}

			let bot_count = self.bots.len();
			let mutation_chance = (num_bots.ilog2() as f32).recip();

			if bot_count < 500 && rnd.gen_range(0.0..1.0) < mutation_chance {
				self.bots.push(self.bots[i].mutate());
			}

			i += 1;
		}

		// actual update
		for bot in &mut self.bots {
			bot.pos += bot.vel;
			bot.pos += Vec2 {
				x: 10.0 * (0.5 - self.wind.x) as f32,
				y: 10.0 * (0.5 - self.wind.x) as f32,
			};
		}
	}

	fn update(&mut self) {
		self.check_keys();

		self.center = vec2(screen_width() / 2.0, screen_height() / 2.0);
		self.dish_radius = f32::min(screen_width(), screen_height()) / 2.0;

		self.update_bots();

		self.update_traces();
	}

	fn check_keys(&mut self) {
		if is_key_pressed(KeyCode::Escape) {
			std::process::exit(0);
		}
		if is_key_pressed(KeyCode::Space) {
			self.bot_lines = !self.bot_lines;
		}
	}
}

fn window_conf() -> Conf {
	Conf {
		window_title: "Bots in a Dish".to_owned(),
		fullscreen: true,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut dish = Dish::new();

	loop {
		dish.wind((get_time() * 0.125).sin_cos());

		dish.update();
		if is_key_down(KeyCode::Enter) {
			dish.update();
		}

		clear_background(Color::new(0.1, 0.1, 0.1, 1.0)); // clear the screen

		dish.draw();

		next_frame().await; // wait for the next frame
	}
}
