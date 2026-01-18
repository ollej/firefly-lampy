use core::cell::OnceCell;
use firefly_rust::audio::{self, File, Gain, Loop, Node};

pub static mut AUDIO: OnceCell<AudioPlayer> = OnceCell::new();


pub struct AudioPlayer {
    sfx_gain: Node<Gain>,
    music_gain: Node<Gain>,
    music_loop: Option<Node<Loop>>,

}

pub fn get_audio_player() -> &'static mut AudioPlayer {
    #[allow(static_mut_refs)]
    unsafe { AUDIO.get_mut() }.unwrap()
}

impl AudioPlayer {
    pub fn new() -> Self {

        let sfx_gain = audio::OUT.add_gain(1.0);
        let music_gain = audio::OUT.add_gain(0.5);

        AudioPlayer {
            sfx_gain,
            music_gain,
            music_loop: None,
        }
    }

    pub fn play_sfx(&mut self, file_name: &str) {
        self.sfx_gain.add_file(file_name);
    }

    // Volume is set 0.0 - 1.0 (full)
    pub fn play_sfx_with_volume(&mut self, file_name: &str, volume: f32) {
        let sound_gain = self.sfx_gain.add_gain(volume);
        sound_gain.add_file(file_name);
    }

    pub fn play_music(&mut self, file_name: &str) {
        let music_loop = self.music_gain.add_loop();
        let music_gain = music_loop.add_gain(0.5);
        music_gain.add_file(file_name);
        self.music_loop = Some(music_loop);
    }
}