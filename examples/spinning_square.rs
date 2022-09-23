use anchor::core::{Config, Context, Game, GameResult};
use anchor::graphics::{self, Color, Layer, Projection, Shape, ShapeBatch, ShapeParams, Transform};
use anchor::input::Key;
use anchor::time;
use anchor::window::WindowConfig;

struct SpinningSquareExample {
    shape_batch: ShapeBatch,
    shape_params: ShapeParams,
    transform: Transform,
}

impl SpinningSquareExample {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            shape_batch: ShapeBatch::new(Shape::square(ctx, 200.0, Color::WHITE)),
            shape_params: ShapeParams::from_color(Color::RED),
            transform: Transform::default(),
        })
    }
}

impl Game for SpinningSquareExample {
    fn on_key_press(&mut self, _ctx: &mut Context, key: Key) {
        match key {
            Key::R => self.shape_params.color = Color::RED,
            Key::G => self.shape_params.color = Color::GREEN,
            Key::B => self.shape_params.color = Color::BLUE,
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.transform.rotation += f32::to_radians(90.0) * time::delta_f32(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.shape_batch.clear();
        self.shape_batch.add(&self.shape_params, &self.transform);

        let projection = Projection::fill(graphics::window_size(ctx));
        graphics::draw(ctx, Color::gray(0.8), &mut [Layer::new(projection, &mut self.shape_batch)]);

        Ok(())
    }
}

fn main() -> GameResult {
    let config = Config {
        window: WindowConfig {
            title: "Spinning Square".to_string(),
            size: (640, 480),
            ..Default::default()
        },
        ..Default::default()
    };

    anchor::run(config, SpinningSquareExample::new)
}
