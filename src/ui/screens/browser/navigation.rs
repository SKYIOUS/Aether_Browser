use crate::engine::pipeline::Tab;

pub(crate) struct NavigationState {
    tab_history: Vec<(Vec<String>, usize)>,
    pub(crate) is_history_nav: bool,
    pub(crate) url_history: Vec<String>,
}

impl NavigationState {
    pub fn new(loaded_tabs: &[Tab]) -> Self {
        let url_history: Vec<String> = loaded_tabs.iter().map(|t| t.url.clone()).collect();
        let tab_history: Vec<(Vec<String>, usize)> = loaded_tabs
            .iter()
            .map(|t| (vec![t.url.clone()], 0))
            .collect();
        Self {
            tab_history,
            is_history_nav: false,
            url_history,
        }
    }

    pub fn new_single_tab(default_url: &str) -> Self {
        Self {
            tab_history: vec![(vec![default_url.to_string()], 0)],
            is_history_nav: false,
            url_history: vec![],
        }
    }

    pub fn can_go_back(&self, active_tab: usize) -> bool {
        self.tab_history
            .get(active_tab)
            .map(|(_h, i)| *i > 0)
            .unwrap_or(false)
    }

    pub fn can_go_forward(&self, active_tab: usize) -> bool {
        self.tab_history
            .get(active_tab)
            .map(|(h, i)| *i + 1 < h.len())
            .unwrap_or(false)
    }

    pub fn go_back(&mut self, active_tab: usize) -> Option<String> {
        let (hist, idx) = self.tab_history.get_mut(active_tab)?;
        if *idx > 0 {
            *idx -= 1;
            self.is_history_nav = true;
            Some(hist[*idx].clone())
        } else {
            None
        }
    }

    pub fn go_forward(&mut self, active_tab: usize) -> Option<String> {
        let (hist, idx) = self.tab_history.get_mut(active_tab)?;
        if *idx + 1 < hist.len() {
            *idx += 1;
            self.is_history_nav = true;
            Some(hist[*idx].clone())
        } else {
            None
        }
    }

    pub fn update_after_page_load(&mut self, active_tab: usize, page_url: &str) {
        if !self.is_history_nav {
            if let Some((hist, idx)) = self.tab_history.get_mut(active_tab) {
                hist.truncate(*idx + 1);
                hist.push(page_url.to_string());
                *idx = hist.len() - 1;
            }
        }
        if !self.url_history.contains(&page_url.to_string()) && !page_url.starts_with("vayu://") {
            self.url_history.push(page_url.to_string());
        }
        self.is_history_nav = false;
    }

    pub fn add_to_url_history(&mut self, url: &str) {
        if !self.url_history.contains(&url.to_string()) {
            self.url_history.push(url.to_string());
        }
    }

    pub fn apply_history_delta(&mut self, active_tab: usize, delta: i32) -> Option<String> {
        let (hist, idx) = self.tab_history.get_mut(active_tab)?;
        let new_idx = (*idx as i32 + delta).clamp(0, hist.len() as i32 - 1) as usize;
        if new_idx < hist.len() && new_idx != *idx {
            *idx = new_idx;
            self.is_history_nav = true;
            Some(hist[new_idx].clone())
        } else {
            None
        }
    }

    pub fn clone_tab_history_for_duplicate(&self, index: usize) -> (Vec<String>, usize) {
        self.tab_history[index].clone()
    }

    pub fn clone_tab_history_for_close_others(&self, keep: usize) -> (Vec<String>, usize) {
        self.tab_history[keep].clone()
    }

    pub fn remove_tab_history(&mut self, index: usize) {
        if index < self.tab_history.len() {
            self.tab_history.remove(index);
        }
    }

    pub fn insert_tab_history(&mut self, index: usize, history: (Vec<String>, usize)) {
        self.tab_history.insert(index, history);
    }

    pub fn push_tab_history(&mut self, history: (Vec<String>, usize)) {
        self.tab_history.push(history);
    }

    pub fn replace_with_tab(&mut self, history: (Vec<String>, usize)) {
        self.tab_history = vec![history];
    }

    pub fn set_fresh_session(&mut self) {
        self.tab_history = vec![(vec!["about:blank".to_string()], 0)];
        self.url_history = vec![];
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.tab_history.len()
    }

    #[cfg(test)]
    pub fn with_tab_history(history: Vec<(Vec<String>, usize)>) -> Self {
        Self {
            tab_history: history,
            is_history_nav: false,
            url_history: vec![],
        }
    }
}
