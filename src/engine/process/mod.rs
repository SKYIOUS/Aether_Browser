use std::sync::mpsc::{channel, Sender, Receiver};

// IPC framework for secure communication
pub enum BrowserMessage {
    RenderRequest(String),
    CommandRequest(String),
}

pub struct BrowserProcess {
    pub pid: u32,
    pub tx: Sender<BrowserMessage>,
    pub rx: Receiver<BrowserMessage>,
}

impl BrowserProcess {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { pid: 1, tx, rx }
    }

    pub fn send(&self, msg: BrowserMessage) {
        self.tx.send(msg).expect("Failed to send message over IPC channel");
    }
}
