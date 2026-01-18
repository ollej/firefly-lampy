use core::cell::OnceCell;
use firefly_rust::audio::{self, File, Gain, Loop, Node, Mix};

pub static mut AUDIO: OnceCell<AudioPlayer> = OnceCell::new();


pub struct AudioPlayer {
    sfx_mix: Node<Mix>,
    sfx_nodes: [Node<Gain>; 4],
    sfx_index: usize,
    music_gain: Node<Gain>,
    music_loop: Node<Loop>,

}

pub fn get_audio_player() -> &'static mut AudioPlayer {
    #[allow(static_mut_refs)]
    unsafe { AUDIO.get_mut() }.unwrap()
}

impl AudioPlayer {
    pub fn new() -> Self {

        let music_loop = audio::OUT.add_loop();
        let music_gain = music_loop.add_gain(0.5);

        let sfx_mix = audio::OUT.add_mix();
        let sfx_nodes = [
              sfx_mix.add_gain(1.0),
              sfx_mix.add_gain(1.0),
              sfx_mix.add_gain(1.0),
              sfx_mix.add_gain(1.0),
        ];
        AudioPlayer {
            sfx_mix,
            sfx_nodes,
            sfx_index: 0,
            music_gain,
            music_loop,
        }
    }

    pub fn play_sfx(&mut self, file_name: &str) {
        self.sfx_nodes[self.sfx_index].clear();
        self.sfx_nodes[self.sfx_index].add_file(file_name);
        self.sfx_index = (self.sfx_index + 1) % 4;
    }

    pub fn play_music(&mut self, file_name: &str) {
        self.music_gain.clear();
        self.music_gain.add_file(file_name);
    }
}