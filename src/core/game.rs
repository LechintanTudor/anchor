use crate::core::{Context, GameError, GameResult};
use crate::graphics::{self, Color};
use crate::input::{Key, ModifiersState, ScrollDelta};
use glam::DVec2;
use winit::event::MouseButton;

/// Allows implementors to receive input events, update their state and display graphics.
#[allow(unused_variables)]
pub trait Game
where
    Self: Sized + 'static,
{
    /// Called once when the application starts.
    fn on_init(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Called right before the application exits.
    fn on_destroy(&mut self, ctx: &mut Context) {}

    /// Intercepts exit requests and decides whether to exit the application.
    /// Return `true` to exit or `false` to ignore the exit request.
    fn on_exit_request(&mut self, ctx: &mut Context) -> bool {
        true
    }

    /// Called when the window is resized with the new window dimensions.
    fn on_window_resize(&mut self, ctx: &mut Context, width: u32, height: u32) {}

    /// Called when a key is pressed.
    fn on_key_press(&mut self, ctx: &mut Context, key: Key) {}

    /// Called when a key is released.
    fn on_key_release(&mut self, ctx: &mut Context, key: Key) {}

    /// Called when the keyboard modifiers changes.
    fn on_modifiers_change(&mut self, ctx: &mut Context, modifiers: ModifiersState) {}

    /// Called when a mouse button is pressed.
    fn on_mouse_button_press(&mut self, ctx: &mut Context, button: MouseButton) {}

    /// Called when a mouse button is released.
    fn on_mouse_button_release(&mut self, ctx: &mut Context, button: MouseButton) {}

    /// Called when a mouse motion occurs.
    fn on_mouse_motion(&mut self, ctx: &mut Context, delta: DVec2) {}

    /// Called when the mouse is moved over the window.
    fn on_cursor_move(&mut self, ctx: &mut Context, position: DVec2) {}

    /// Called when a scroll occurs.
    fn on_scroll(&mut self, ctx: &mut Context, delta: ScrollDelta) {}

    /// Called once per frame after input events are processed.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Called 0 or more times per frame to handle logic that requires a fixed timestep.
    fn fixed_update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Called once per frame before drawing.
    fn late_update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Called once per frame to draw graphics.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, Color::BLACK, &mut []);
        Ok(())
    }

    /// Intercepts errors and decides whether to exit the application in response to them.
    /// Return `true` to exit the application or `false` continue.
    fn handle_error(&mut self, ctx: &mut Context, error: GameError) -> bool {
        eprintln!("{:?}: {}", ctx.game_phase, error);
        true
    }
}

/// Allows the [run](crate::run) functions to build the application after the [Context] is created.
pub trait GameBuilder {
    /// The type of game to build.
    type Game: Game;

    /// Builds the game.
    fn build(self, ctx: &mut Context) -> GameResult<Self::Game>;
}

impl<F, G> GameBuilder for F
where
    F: FnOnce(&mut Context) -> GameResult<G>,
    G: Game,
{
    type Game = G;

    fn build(self, ctx: &mut Context) -> GameResult<Self::Game> {
        self(ctx)
    }
}
