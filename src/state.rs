use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::{
    audio, clear_screen, load_file_buf, log_debug, read_buttons, Buttons, Color, FileBuf, Peer,
    Peers,
};

use crate::{
    firefly::*, game_state::*, player::*, rendering::*, tile_array::*, utility::*, world::*,
};
pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    buttons: Buttons,
    fireflies: Vec<Firefly>,
    pub font: FileBuf,
    fx: audio::Node<audio::Gain>,
    pub game_state: GameState,
    pub players: Vec<Player>,
    me: Option<Peer>,
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
            players: Vec::new(),
            me: None,
            spritesheet: load_file_buf("spritesheet").unwrap(),
            theme: audio::OUT.add_gain(0.5),
            title: load_file_buf("_splash").unwrap(),
            world: World::new_from_2d_array(TILE_ARRAY),
        }
    }
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

impl State {
    const WIN_POINTS: i32 = 10;

    pub fn new(player: Peer, peers: Peers) -> Self {
        State {
            players: peers.iter().map(|peer| Player::new(peer)).collect(),
            me: Some(player),
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
                    player.update(&self.world);
                }
                if self.fireflies.len() < Firefly::MAX_COUNT as usize && random_range(0, 100) < 10 {
                    self.fireflies.push(Firefly::random(&self.world));
                }
                for firefly in self.fireflies.iter_mut() {
                    firefly.update(&self.world);
                }
                self.fireflies.retain(|firefly| {
                    if !firefly.is_in_goal(&self.world) {
                        return true;
                    }

                    // Score point for fireflies turned in at goal
                    if let Some(scoring_player) = self
                        .players
                        .iter_mut()
                        .find(|player| Some(player.attraction_target) == firefly.attracted_to)
                    {
                        scoring_player.points += 1;
                        return false;
                    }

                    true
                });

                // Check win condition
                if let Some(winner) = self
                    .players
                    .iter()
                    .find(|player| player.points >= Self::WIN_POINTS)
                {
                    self.game_state = GameState::GameOver(Some(winner.peer) == self.me);
                }
            }
            GameState::GameOver(_won) => {
                if just_pressed.e {
                    self.restart();
                }
            }
        }
    }

    pub fn draw(&self) {
        clear_screen(Color::White);

        if let Some(player) = self.local_player() {
            self.world.draw(&player.camera);

            for player in self.players.iter() {
                player.draw();
            }
            for firefly in self.fireflies.iter() {
                firefly.draw(&player.camera);
            }
            render_ui();
        }
    }

    pub fn restart(&mut self) {
        self.fireflies = vec![];
        self.players.iter_mut().for_each(|player| player.points = 0);
        self.game_state = GameState::Playing;
    }

    pub fn local_player(&self) -> Option<&Player> {
        self.me
            .and_then(|me| self.players.iter().find(|p| p.peer == me))
    }
}
