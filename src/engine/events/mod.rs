pub enum Event {
    Click { x: f32, y: f32 },
    KeyDown { key: String },
}

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_queue() {
        let mut queue = EventQueue::new();
        queue.push(Event::Click { x: 10.0, y: 20.0 });
        let event = queue.pop();
        assert!(matches!(event, Some(Event::Click { x: 10.0, y: 20.0 })));
    }
}
