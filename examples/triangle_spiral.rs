use anchor::game::{Config, Context, Game, GameResult};
use anchor::glam::Vec2;
use anchor::graphics::{self, Color, Layer, Projection, Shape, ShapeBatch, ShapeParams, Transform};
use anchor::time;
use anchor::window::WindowConfig;
use std::collections::VecDeque;

const TRIANGLE_SIDE_LENGTH: f32 = 30.0;
const TRIANGLE_VELOCITY: f32 = 100.0;
const TRIANGLE_ANGULAR_VELOCITY: f32 = 90.0;
const SPIRAL_ANGULAR_VELOCITY: f32 = 90.0;
const SPIRAL_SPAWN_INTERVAL: u32 = 8;
const MAX_TRIANGLES: usize = 1024;

struct SpinningTriangle {
    shape_params: ShapeParams,
    transform: Transform,
    direction: Vec2,
}

impl SpinningTriangle {
    fn new(rotation: f32, color: Color) -> Self {
        Self {
            shape_params: ShapeParams::from_color(color),
            transform: Transform::default(),
            direction: Vec2::new(rotation.sin(), rotation.cos()),
        }
    }
}

struct TriangleSpiralExample {
    shape_batch: ShapeBatch,
    triangles: VecDeque<SpinningTriangle>,
    spawn_counter: u32,
    spiral_rotation: f32,
}

impl TriangleSpiralExample {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let triangle = Shape::equilateral_triangle(ctx, TRIANGLE_SIDE_LENGTH, Color::WHITE);

        Ok(Self {
            shape_batch: ShapeBatch::new(triangle),
            triangles: VecDeque::new(),
            spawn_counter: 0,
            spiral_rotation: 0.0,
        })
    }
}

impl Game for TriangleSpiralExample {
    fn fixed_update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.spawn_counter % SPIRAL_SPAWN_INTERVAL == 0 {
            self.triangles.push_back(SpinningTriangle::new(self.spiral_rotation, Color::RED));

            self.triangles.push_back(SpinningTriangle::new(
                self.spiral_rotation + f32::to_radians(120.0),
                Color::GREEN,
            ));

            self.triangles.push_back(SpinningTriangle::new(
                self.spiral_rotation + f32::to_radians(240.0),
                Color::BLUE,
            ));
        }

        if self.triangles.len() > MAX_TRIANGLES {
            self.triangles.drain(..(self.triangles.len() - MAX_TRIANGLES));
        }

        self.spawn_counter += 1;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = time::delta_f32(ctx);

        for triangle in self.triangles.iter_mut() {
            triangle.transform.translation += triangle.direction * TRIANGLE_VELOCITY * delta;
            triangle.transform.rotation += f32::to_radians(TRIANGLE_ANGULAR_VELOCITY) * delta;
        }

        self.spiral_rotation += f32::to_radians(SPIRAL_ANGULAR_VELOCITY) * delta;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.shape_batch.clear();
        for triangle in self.triangles.iter() {
            self.shape_batch.add(&triangle.shape_params, &triangle.transform);
        }

        let projection = Projection::fill(graphics::window_size(ctx));
        graphics::draw(ctx, Color::WHITE, &mut [Layer::new(projection, &mut self.shape_batch)]);
        Ok(())
    }
}

fn main() -> GameResult {
    let config = Config {
        window: WindowConfig {
            title: "Triangle Spiral".to_string(),
            size: (640, 480),
            ..Default::default()
        },
        ..Default::default()
    };

    anchor::run(config, TriangleSpiralExample::new)
}
