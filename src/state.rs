use alloc::vec;
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::{
    add_progress, audio, clear_screen, load_file_buf, read_buttons, Buttons, Color, FileBuf, Peer,
    Peers, Point,
};

use crate::{
    audio::*, constants::*, fireflies::*, firefly::*, game_state::*, particles::*, player::*,
    rendering::*, tile_array::*, utility::*, world::*,
};
pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    buttons: Buttons,
    pub debug: bool,
    fireflies: Fireflies,
    pub font: FileBuf,
    fx: audio::Node<audio::Gain>,
    pub game_state: GameState,
    me: Option<Peer>,
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
            debug: false,
            fireflies: Fireflies::new(),
            font: load_file_buf("font").unwrap(),
            fx: audio::OUT.add_gain(1.0),
            game_state: GameState::Title,
            me: None,
            players: Vec::new(),
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
                self.update_playing();
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
            self.fireflies.draw(&player.camera);
            render_ui();
        }
    }

    pub fn restart(&mut self) {
        self.fireflies = Fireflies::new();
        self.players.iter_mut().for_each(|player| player.points = 0);
        self.game_state = GameState::Playing;
    }

    pub fn local_player(&self) -> Option<&Player> {
        self.me
            .and_then(|me| self.players.iter().find(|p| p.peer == me))
    }

    fn update_playing(&mut self) {
        for player in self.players.iter_mut() {
            player.update(&self.world);
        }
        let removed_fireflies = self.fireflies.update(&self.world);
        self.collect_fireflies(removed_fireflies);
        self.check_win_condition();
    }

    fn check_win_condition(&mut self) {
        if let Some(winner) = self
            .players
            .iter()
            .find(|player| player.points >= Self::WIN_POINTS)
        {
            add_progress(winner.peer, BADGE_WINS, 1);
            self.game_state = GameState::GameOver(Some(winner.peer) == self.me);
        }
    }

    fn collect_fireflies(&mut self, fireflies: Vec<Firefly>) {
        fireflies
            .iter()
            .for_each(|firefly| self.handle_collected_firefly(firefly));
    }

    fn handle_collected_firefly(&mut self, firefly: &Firefly) {
        if let Some(attracted_to) = firefly.attracted_to {
            for player in self.players.iter_mut() {
                if player.attraction_target == attracted_to {
                    player.points += 1;
                    get_audio_player().play_sfx("pling");
                    return;
                }
            }
        }
    }
}
