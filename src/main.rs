#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;

use firefly_rust as ff;

mod constants;
mod drawing;
mod firefly;
mod game_state;
mod particles;
mod player;
mod point_math;
mod rendering;
mod state;
mod camera;
mod utility;

use game_state::*;
use rendering::*;
use state::*;

#[unsafe(no_mangle)]
extern "C" fn handle_menu(menu_item: u8) {
    let state = get_state();
    match menu_item {
        1 => state.game_state = GameState::Credits,
        2 => state.restart(),
        3 => state.game_state = GameState::Info,
        _ => (),
    }
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let peers = ff::get_peers();
    let me = ff::get_me();
    #[allow(static_mut_refs)]
    unsafe { STATE.set(State::new(me, peers)) }.ok().unwrap();
    ff::add_menu_item(1, "Credits");
    ff::add_menu_item(2, "Restart");
    ff::add_menu_item(3, "Info");
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    state.update();
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    match state.game_state {
        GameState::Title => render_title(),
        GameState::Credits => render_credits(),
        GameState::Info => render_info(),
        GameState::Playing => state.draw(),
        GameState::Died => render_died(),
        GameState::GameOver => render_gameover(),
    }
}
