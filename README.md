# Anchor 2D Game Framework

Modern game framework for building 2D games. Not yet released on [crates.io](https://crates.io/).

## Goals

* Be up-to-date with the rust gamedev ecosystem.
* Be fexible. Don't depend on an ECS or specific serialization format.
* Be lightweight. Use as few dependencies as possible.

## Features

* Efficient rendering of sprites, shapes and text.
* Flexible game loop with fixed update support.
* Convenient abstractions for querying the state of input devices.

## Example

Draws a blue square in the center of the screen.

```rust
use anchor::core::{Config, Context, Game, GameResult};
use anchor::graphics::{self, Color, Layer, Projection, Shape, ShapeBatch, ShapeParams, Transform};

struct MyGame {
    shape_batch: ShapeBatch,
    shape_params: ShapeParams,
    shape_transform: Transform,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<MyGame> {
        Ok(Self {
            shape_batch: ShapeBatch::new(Shape::square(ctx, 300.0, Color::WHITE)),
            shape_params: ShapeParams::from_color(Color::BLUE),
            shape_transform: Transform::default(),
        })
    }
}

impl Game for MyGame {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.shape_batch.clear();
        self.shape_batch.add(&self.shape_params, &self.shape_transform);

        let projection = Projection::fill(graphics::window_size(ctx));
        graphics::draw(ctx, Color::WHITE, &mut [Layer::new(projection, &mut self.shape_batch)]);

        Ok(())
    }
}

fn main() -> GameResult {
    anchor::run(Config::default(), MyGame::new)
}
```

## Core Crates

Core crates used by Anchor.

* Platform interaction: [winit](https://crates.io/crates/winit)
* Graphics: [wgpu](https://crates.io/crates/wgpu)
* Text rendering: [glyph_brush](https://crates.io/crates/glyph_brush)
* Math: [glam](https://crates.io/crates/glam)

## Thanks

Anchor takes inspiration from other free and open source game frameworks and game engines
such as [ggez](https://crates.io/crates/ggez), [bevy](https://crates.io/crates/bevy)
and [macroquad](https://crates.io/crates/macroquad).

# License

Anchor is dual-licensed under either

- MIT License ([docs/LICENSE-MIT] or https://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([docs/LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)

at your option. <br /> Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above without any additional terms or conditions.
