use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::{
    Buttons, Color, FileBuf, Peer, Peers, add_progress, audio, clear_screen, load_file_buf,
    read_buttons,
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
    particles: ParticleSystem,
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
            particles: ParticleSystem::new(100),
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
            players: peers.iter().map(Player::new).collect(),
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
            self.particles.render(&player.camera);
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
        self.particles.update();
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
                    self.spawn_collection_burst(firefly);
                    return;
                }
            }
        }
    }

    pub fn spawn_collection_burst(&mut self, firefly: &Firefly) {
        self.particles.spawn_radial_burst(
            firefly.position.x,
            firefly.position.y,
            random_range(30, 40) as u8,
            random_range(1, 3) as i16,
            8,
            firefly.color.into(),
        );
    }
}
