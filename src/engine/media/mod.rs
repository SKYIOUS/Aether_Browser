use std::sync::{Arc, Mutex};

pub struct MediaEngine {
    pub playing: bool,
    pub frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl MediaEngine {
    pub fn new() -> Self {
        Self { playing: false, frame_buffer: Arc::new(Mutex::new(vec![])) }
    }

    pub fn update(&mut self) {
        // Here we would push new frames into frame_buffer
    }
}
