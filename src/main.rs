use ggez::{
	conf::WindowMode,
	event::{self, EventHandler, MouseButton},
	graphics::{self, Color, DrawMode, Mesh, Rect},
	mint::Point2,
	Context, ContextBuilder, GameError, GameResult,
};

const CELL_SIZE: (f32, f32) = (20.0, 20.0);
const GRID_SIZE: (f32, f32) = (30.0, 20.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);
const LINE_WIDTH: f32 = 1.0;

struct State {
	grid: Vec<Vec<bool>>,
	running: bool,
}

impl State {
	pub fn new() -> Self {
		State {
			grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
			running: false,
		}
	}
}

impl EventHandler<GameError> for State {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, Color::BLACK);

		for i in 0..GRID_SIZE.0 as usize {
			for j in 0..GRID_SIZE.1 as usize {
				if self.grid[i][j] {
					let rect = Mesh::new_rectangle(
						ctx,
						DrawMode::fill(),
						Rect::new(
							i as f32 * CELL_SIZE.0,
							j as f32 * CELL_SIZE.1,
							CELL_SIZE.0,
							CELL_SIZE.1,
						),
						Color::WHITE,
					)?;
					graphics::draw(ctx, &rect, (Point2 { x: 0.0, y: 0.0 },))?;
				}

				if j == 0 {
					continue;
				}

				let line = Mesh::new_line(
					ctx,
					&vec![
						Point2 {
							x: 0.0,
							y: j as f32 * CELL_SIZE.1,
						},
						Point2 {
							x: WINDOW_SIZE.0,
							y: j as f32 * CELL_SIZE.1,
						},
					],
					LINE_WIDTH,
					Color::WHITE,
				)?;
				graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
			}

			if i == 0 {
				continue;
			}

			let line = Mesh::new_line(
				ctx,
				&vec![
					Point2 {
						x: i as f32 * CELL_SIZE.0,
						y: 0.0,
					},
					Point2 {
						x: i as f32 * CELL_SIZE.1,
						y: WINDOW_SIZE.1,
					},
				],
				LINE_WIDTH,
				Color::WHITE,
			)?;
			graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
		}

		graphics::present(ctx)?;

		Ok(())
	}

	fn mouse_button_down_event(
		&mut self,
		_ctx: &mut Context,
		_button: MouseButton,
		x: f32,
		y: f32,
	) {
		self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize] =
			!self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize];
	}

	fn key_down_event(
		&mut self,
		_ctx: &mut Context,
		keycode: event::KeyCode,
		_keymods: event::KeyMods,
		repeat: bool,
	) {
		if keycode == event::KeyCode::Space && !repeat {
			self.running = !self.running;
		}
	}
}

fn main() -> GameResult {
	let state = State::new();

	let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	event::run(ctx, event_loop, state);
}
