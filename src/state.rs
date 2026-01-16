use core::cell::OnceCell;
use firefly_rust::{audio, load_file_buf, read_buttons, Buttons, FileBuf, Peer};

use crate::{game_state::*, rendering::*};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    pub buttons: Buttons,
    pub font: FileBuf,
    pub fx: audio::Node<audio::Gain>,
    pub game_state: GameState,
    pub points: i32,
    pub spritesheet: FileBuf,
    pub theme: audio::Node<audio::Gain>,
    pub title: FileBuf,
    pub won: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            buttons: Buttons::default(),
            font: load_file_buf("font").unwrap(),
            fx: audio::OUT.add_gain(1.0),
            game_state: GameState::Title,
            points: 0,
            spritesheet: load_file_buf("spritesheet").unwrap(),
            theme: audio::OUT.add_gain(0.5),
            title: load_file_buf("_splash").unwrap(),
            won: false,
        }
    }
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

impl State {
    pub fn update(&mut self) {
        // TODO: Read each player
        let buttons = read_buttons(Peer::COMBINED);
        let just_pressed = buttons.just_pressed(&self.buttons);
        let just_released = buttons.just_released(&self.buttons);
        self.buttons = buttons;

        match self.game_state {
            GameState::Title => {
                if just_pressed.any() {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Credits => {
                if just_pressed.any() {
                    self.game_state = GameState::Title;
                }
            }
            GameState::Info => {
                if just_pressed.any() {
                    self.game_state = GameState::Title;
                }
            }
            GameState::Died => {
                if just_pressed.any() {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // TODO: Read touchpad
                if just_pressed.s {
                    todo!()
                }
                if just_released.s {
                    todo!()
                }
                if just_pressed.w {
                    todo!()
                }
                if just_pressed.e {
                    todo!()
                }
            }
            GameState::GameOver => {
                if just_pressed.e {
                    self.restart();
                }
            }
        }
    }

    pub fn draw(&self) {
        render_ui();
        // TODO: Game
    }

    pub fn restart(&mut self) {
        todo!()
    }
}
