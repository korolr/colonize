extern crate event;
extern crate gfx_voxel;
extern crate input;
extern crate noise;
extern crate quack;
extern crate tcod;
extern crate tcod_window;
extern crate window;

use game::Game;

mod chunk;
mod game;
mod gamescene;
mod gamestate;
mod map;
mod menuscene;
mod scene;

fn main() {
    let game = Game::new();
    game.run();
}