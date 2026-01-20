use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::OnceCell;

use firefly_rust::{
    add_progress, audio, clear_screen, load_file_buf, read_buttons, Buttons, Color, FileBuf, Peer,
    Peers,
};

use crate::{
    audio::*, camera::Camera, constants::*, fireflies::Fireflies, firefly::Firefly,
    game_state::GameState, particles::ParticleSystem, player::Player, point_math::PointMath,
    rendering::*, text::Text, tile_array::TILE_ARRAY, utility::random_range, world::World,
};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    buttons: Buttons,
    pub camera: Camera,
    pub debug: bool,
    fireflies: Fireflies,
    pub font: FileBuf,
    pub font_points: FileBuf,
    fx: audio::Node<audio::Gain>,
    pub game_state: GameState,
    pub me: Option<Peer>,
    particles: ParticleSystem,
    pub players: Vec<Player>,
    pub spritesheet: FileBuf,
    texts: Vec<Text>,
    theme: audio::Node<audio::Gain>,
    pub title: FileBuf,
    world: World,
}

impl Default for State {
    fn default() -> Self {
        State {
            buttons: Buttons::default(),
            camera: Camera::new(WORLD_WIDTH, WORLD_HEIGHT),
            debug: false,
            fireflies: Fireflies::new(),
            font: load_file_buf("font").unwrap(),
            font_points: load_file_buf("font_points").unwrap(),
            fx: audio::OUT.add_gain(1.0),
            game_state: GameState::Title,
            me: None,
            particles: ParticleSystem::new(200),
            players: Vec::new(),
            spritesheet: load_file_buf("spritesheet").unwrap(),
            texts: vec![],
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
    const WIN_POINTS: i32 = 20;

    pub fn new(player: Peer, peers: Peers) -> Self {
        let world = World::new_from_2d_array(TILE_ARRAY);
        State {
            players: peers.iter().map(|peer| Player::new(peer, &world)).collect(),
            me: Some(player),
            world,
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

        self.world.draw(&self.camera);

        for player in self.players.iter() {
            player.draw(&self.camera);
        }

        self.fireflies.draw(&self.camera);
        self.particles.render(&self.camera);
        self.draw_texts();
        render_ui();
    }

    pub fn restart(&mut self) {
        self.fireflies = Fireflies::new();
        self.players
            .iter_mut()
            .for_each(|player| player.reset(&self.world));
        self.texts = vec![];
        self.particles = ParticleSystem::new(200);
        self.game_state = GameState::Playing;
    }

    pub fn add_points(&mut self, points: i32) -> i32 {
        for player in self.players.iter_mut() {
            if self.me == Some(player.peer) {
                player.points += points;
                return player.points;
            }
        }
        0
    }

    fn update_playing(&mut self) {
        for player in self.players.iter_mut() {
            player.update(&self.world);
            if self.me == Some(player.peer) {
                self.camera.follow_player(player.position, 0.2);
            }
        }
        let removed_fireflies = self.fireflies.update(&self.world);
        self.collect_fireflies(removed_fireflies);
        self.check_win_condition();
        self.update_texts();
        self.particles.update();
    }

    fn draw_texts(&self) {
        for text in self.texts.iter() {
            text.draw(&self.camera);
        }
    }

    fn update_texts(&mut self) {
        self.texts.iter_mut().for_each(|text| text.update());
        self.texts.retain(|text| !text.remove());
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
                    player.points += firefly.points();
                    get_audio_player().play_sfx("pling");
                    self.spawn_collection_burst(firefly);
                    self.spawn_point_text(firefly);
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
            random_range(1, 2) as i16,
            8,
            firefly.color().into(),
        );
    }

    pub fn spawn_point_text(&mut self, firefly: &Firefly) {
        let content = format!("+{}", firefly.points());

        // Scatter position to show all texts whencollecting
        // multiple fireflies at the same time.

        self.texts.push(Text::new(
            content,
            firefly.color().into(),
            firefly.position.scatter(12),
        ));
    }
}
