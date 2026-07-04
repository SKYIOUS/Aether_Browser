use super::js_bridge::{JsBridge, TimerEntry};
use std::time::Duration;

impl JsBridge {
    pub fn set_timeout(&mut self, source: String, delay_ms: u64) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;
        let fire_at = std::time::Instant::now() + Duration::from_millis(delay_ms);
        self.timers.push(TimerEntry { id, source, delay_ms, is_interval: false, fire_at });
        id
    }

    pub fn set_interval(&mut self, source: String, delay_ms: u64) -> u32 {
        let delay_ms = delay_ms.max(4);
        let id = self.next_timer_id;
        self.next_timer_id += 1;
        let fire_at = std::time::Instant::now() + Duration::from_millis(delay_ms);
        self.timers.push(TimerEntry { id, source, delay_ms, is_interval: true, fire_at });
        id
    }

    pub fn clear_timer(&mut self, id: u32) {
        self.timers.retain(|t| t.id != id);
    }

    pub fn has_pending_timers(&self) -> bool {
        !self.timers.is_empty()
    }

    /// Returns (timer_id, source_code) pairs for expired timers.
    /// Re-registers interval timers for their next fire.
    pub fn poll_timers(&mut self) -> Vec<(u32, String)> {
        let now = std::time::Instant::now();
        let mut ready = vec![];
        let mut i = 0;
        while i < self.timers.len() {
            if self.timers[i].fire_at <= now {
                let entry = self.timers.remove(i);
                ready.push((entry.id, entry.source.clone()));
                if entry.is_interval {
                    let new_id = self.next_timer_id;
                    self.next_timer_id += 1;
                    let delay_ms = entry.delay_ms.max(1);
                    let fire_at = now + Duration::from_millis(delay_ms);
                    self.timers.push(TimerEntry { id: new_id, source: entry.source, delay_ms: entry.delay_ms, is_interval: true, fire_at });
                }
            } else {
                i += 1;
            }
        }
        ready
    }
}