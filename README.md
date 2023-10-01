# Anchor 2D Game Framework

Framework for building 2D games and applications. Not yet released on
[crates.io](https://crates.io/).

## Goals

- Be up-to-date with the rust gamedev ecosystem.
- Be fexible. Don't depend on an ECS or specific serialization format.
- Be lightweight. Use as few dependencies as possible.

## Features

- Efficient rendering of sprites, shapes and text.
- Flexible game loop with fixed timestep support.

## Example

Draws a texture in the middle of the window.

```rust
use anchor::game::{Config, Context, Game, GameResult};
use anchor::graphics::sprite::Texture;
use anchor::graphics::{AsDrawable, Canvas, Drawable};

struct MyGame {
    texture: Texture,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            texture: Texture::from_file(ctx, "assets/images/player.png")?,
        })
    }
}

impl Game for MyGame {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::new(ctx);

        self.texture
            .as_drawable()
            .anchor_center()
            .translation((320.0, 240.0))
            .draw(&mut canvas);

        canvas.present();
        Ok(())
    }
}

fn main() -> GameResult {
    anchor::run(
        MyGame::new,
        Config {
            window_title: "Example".to_string(),
            window_size: (640, 480),
            ..Default::default()
        },
    )
}
```

## Core Crates

Core crates used by Anchor.

- Platform interaction: [winit](https://crates.io/crates/winit)
- Graphics: [wgpu](https://crates.io/crates/wgpu)
- Text rendering: [glyph_brush](https://crates.io/crates/glyph_brush)
- Math: [glam](https://crates.io/crates/glam)

## Thanks

Anchor takes inspiration from other free and open source game frameworks and
game engines such as [ggez](https://crates.io/crates/ggez),
[bevy](https://crates.io/crates/bevy) and
[macroquad](https://crates.io/crates/macroquad).

# License

Anchor is dual-licensed under either

- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/license/mit/](https://opensource.org/license/mit/))

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))

at your option.

<br />

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above without any additional terms or conditions.
