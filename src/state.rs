use alloc::vec;
use alloc::{format, vec::Vec};
use core::cell::OnceCell;
use firefly_rust::{
    audio, clear_screen, load_file_buf, log_debug, read_buttons, read_pad, Buttons, Color, FileBuf,
    Peer, Peers,
};

use crate::{firefly::*, game_state::*, player::*, rendering::*, utility::*, world::*};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    buttons: Buttons,
    fireflies: Vec<Firefly>,
    pub font: FileBuf,
    fx: audio::Node<audio::Gain>,
    pub game_state: GameState,
    player: Option<Peer>,
    pub players: Vec<Player>,
    pub spritesheet: FileBuf,
    theme: audio::Node<audio::Gain>,
    pub title: FileBuf,
    world: World,
}

impl Default for State {
    fn default() -> Self {
        State {
            buttons: Buttons::default(),
            fireflies: vec![],
            font: load_file_buf("font").unwrap(),
            fx: audio::OUT.add_gain(1.0),
            game_state: GameState::Title,
            player: None,
            players: Vec::new(),
            spritesheet: load_file_buf("spritesheet").unwrap(),
            theme: audio::OUT.add_gain(0.5),
            title: load_file_buf("_splash").unwrap(),
            world: World::new(50,50),
        }
    }
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

impl State {
    pub fn new(player: Peer, peers: Peers) -> Self {
        State {
            player: Some(player),
            players: peers.iter().map(|peer| Player::new(peer)).collect(),
            ..State::default()
        }
    }

    pub fn update(&mut self) {
        let buttons = read_buttons(Peer::COMBINED);
        let just_pressed = buttons.just_pressed(&self.buttons);
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
                for player in self.players.iter_mut() {
                    player.update();
                }
                if self.fireflies.len() < Firefly::MAX_COUNT as usize && random_range(0, 100) < 10 {
                    self.fireflies.push(Firefly::random());
                }
                for firefly in self.fireflies.iter_mut() {
                    firefly.update();
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
        clear_screen(Color::White);
        self.world.draw_all_without_camera();
        for player in self.players.iter() {
            player.draw();
        }
        for firefly in self.fireflies.iter() {
            firefly.draw();
        }
        render_ui();
    }

    pub fn restart(&mut self) {
        todo!()
    }
}
