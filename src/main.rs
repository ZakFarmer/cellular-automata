use ggez::event::{self, EventHandler};
use ggez::graphics::{self, BlendMode, Color};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

const CELL_SIZE: i32 = 10;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 1200;

struct MainState {
    grid: Vec<Vec<bool>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.set_drawable_size(WIDTH as f32, HEIGHT as f32)?;

        let drawable_width = ctx.gfx.drawable_size().0;
        let drawable_height = ctx.gfx.drawable_size().1;

        let num_cols = (drawable_width / CELL_SIZE as f32) as usize;
        let num_rows = (drawable_height / CELL_SIZE as f32) as usize;

        let rng = &mut rand::thread_rng();

        let grid: Vec<Vec<bool>> = (0..num_cols)
            .map(|_| (0..num_rows).map(|_| rng.gen_bool(1.0 / 3.0)).collect())
            .collect();

        let s = MainState { grid };

        Ok(s)
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.set_blend_mode(BlendMode::REPLACE);

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell {
                    let rect = graphics::Rect::new(
                        (j as f32 * CELL_SIZE as f32),
                        (i as f32 * CELL_SIZE as f32),
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                    );

                    // println!("Drawing rect: {:?}", rect);

                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest(rect.point())
                            .scale(rect.size())
                            .color(Color::WHITE),
                    );
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> () {
    let (mut ctx, event_loop) = ContextBuilder::new("cellular_automata", "ZakFarmer")
        .build()
        .expect("Couldn't create context");

    let state = MainState::new(&mut ctx).expect("Couldn't init state");
    event::run(ctx, event_loop, state);
}
