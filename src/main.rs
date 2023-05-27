use ggez::event::{self, EventHandler};
use ggez::graphics::{self, BlendMode, Color};
use ggez::{Context, ContextBuilder, GameResult};

const CELL_SIZE: i32 = 10;

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

struct MainState {
    grid: Vec<Vec<bool>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.set_window_title("Cellular Automata");
        
        ctx.gfx.set_drawable_size(WIDTH as f32, HEIGHT as f32)?;

        let drawable_width = ctx.gfx.drawable_size().0;
        let drawable_height = ctx.gfx.drawable_size().1;

        let num_cols = (drawable_width / CELL_SIZE as f32) as usize;
        let num_rows = (drawable_height / CELL_SIZE as f32) as usize;

        let mut grid: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];

        let grid_length = grid.len();
        
        grid[grid_length - 1][num_cols / 2] = true;

        let s = MainState { grid };

        Ok(s)
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Make a copy of the last row
        let old_row = self.grid[self.grid.len() - 1].clone();
        
        // Calculate the new row
        let mut new_row = vec![false; old_row.len()];
        for j in 0..old_row.len() {
            let left = old_row[(j + old_row.len() - 1) % old_row.len()];
            let center = old_row[j];
            let right = old_row[(j + 1) % old_row.len()];

            // Rule 30
            new_row[j] = left ^ (center || right);
        }

        // Remove the first row and add the new row at the end
        self.grid.remove(0);
        self.grid.push(new_row);

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
                        j as f32 * CELL_SIZE as f32,
                        i as f32 * CELL_SIZE as f32,
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
