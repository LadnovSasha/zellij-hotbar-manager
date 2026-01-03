use std::collections::HashMap;
use zellij_tile::prelude::*;

use crate::persistence::HotbarData;

pub const PREFIX_KEY: &str = "hotbar_";

#[derive(Default)]
pub enum HotBarMode {
    #[default]
    Session,
    Tab,
}

#[derive(Default, PartialEq)]
pub enum ModalMode {
    #[default]
    Normal,
    Move,
}

#[derive(Default)]
pub struct HotbarManager {
    pub saved_sessions: HashMap<String, String>,
    pub hotbar_mode: HotBarMode,
    pub active_session: Option<String>,
    previous_session: Option<String>,
    pub hotbar_data: HotbarData,
    pub selected_index: usize,
    pub modal_mode: ModalMode,
    pub all_sessions: Vec<String>,
    pub is_visible: bool,
    pub scroll_offset: usize,
}

impl HotbarManager {
    pub fn show_ui(&mut self) {
        show_self(true);
        self.is_visible = true;
        set_selectable(true);
        // Reset scroll to show the selected item
        self.scroll_offset = 0;
    }

    pub fn hide_ui(&mut self) {
        hide_self();
        self.is_visible = false;
        set_selectable(false);
    }

    fn switch_to_session(&mut self, hotbar_key: &str) {
        if let Some(session) = self.saved_sessions.get(hotbar_key) {
            switch_session(Some(session));
            switch_to_input_mode(&InputMode::Normal);

            println!("Switched to session: {}", session);
        } else {
            println!("No session found with key: {hotbar_key}");
        }
    }

    fn switch_to_tab(&mut self, hotbar_key: &str) {
        let tab_idx = hotbar_key.trim_start_matches(PREFIX_KEY);
        if let Ok(idx) = tab_idx.parse::<u32>() {
            switch_tab_to(idx);
            switch_to_input_mode(&InputMode::Normal);

            println!("Switched to tab index: {idx}");
        } else {
            println!("Invalid tab index: {tab_idx} for key: {hotbar_key}");
        }
    }

    pub fn switch_to(&mut self, key: &str) {
        match self.hotbar_mode {
            HotBarMode::Session => self.switch_to_session(key),
            HotBarMode::Tab => self.switch_to_tab(key),
        }
    }

    pub fn switch_to_previous_session(&mut self) {
        if let Some(promote_session) = self.previous_session.clone() {
            // Don't call close_focus() - plugin runs headless
            switch_session(Some(&promote_session));
            switch_to_input_mode(&InputMode::Normal);
        }
    }

    pub fn set_hotbar_mode(&mut self, mode: &str) {
        self.hotbar_mode = match mode {
            "tab" => HotBarMode::Tab,
            _ => HotBarMode::Session,
        };
    }

    pub fn set_active_session(&mut self, session_name: &str) -> bool {
        if Some(session_name) == self.active_session.as_deref() {
            return false;
        }

        self.previous_session = self.active_session.clone();
        self.active_session = Some(session_name.to_string());

        self.hotbar_data.previous_session = self.previous_session.clone();

        eprintln!(
            "Active session changed to: {:?}, previous session: {:?}",
            self.active_session, self.previous_session
        );

        true
    }

    pub fn load_hotbar_data(&mut self) {
        match HotbarData::load_from_file() {
            Ok(data) => {
                self.hotbar_data = data;
                self.previous_session = self.hotbar_data.previous_session.clone();
                eprintln!("Loaded hotbar data: {:?}", self.hotbar_data);
            }
            Err(e) => {
                eprintln!("Failed to load hotbar data: {}, using defaults", e);
                self.hotbar_data = HotbarData::default();
            }
        }
    }

    pub fn save_hotbar_data(&self) {
        if let Err(e) = self.hotbar_data.save_to_file() {
            eprintln!("Failed to save hotbar data: {}", e);
        }
    }

    pub fn remove_from_slot(&mut self, slot: usize) {
        if slot < 5 {
            self.hotbar_data.set_session_at_slot(slot, None);
            self.save_hotbar_data();
            eprintln!("Removed session from slot {}", slot + 1);
        }
    }

    pub fn switch_to_slot(&mut self, slot: usize) {
        if let Some(session_name) = self.hotbar_data.get_session_at_slot(slot) {
            if self.active_session.as_deref() == Some(session_name.as_str()) {
                eprintln!("Already in session: {}", session_name);
                return;
            }
            // Don't call close_focus() - plugin runs headless
            switch_session(Some(session_name));
            switch_to_input_mode(&InputMode::Normal);
            eprintln!("Switched to session: {}", session_name);
        } else {
            eprintln!("No session saved in slot {}", slot + 1);
        }
    }

    pub fn navigate_sessions(&mut self, direction: i32) {
        let max_index = self.all_sessions.len().saturating_sub(1);
        if direction > 0 {
            self.selected_index = (self.selected_index + 1).min(max_index);
        } else if direction < 0 && self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 || self.all_sessions.is_empty() {
            return;
        }

        // If selected item is above the visible window, scroll up
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
        // If selected item is below the visible window, scroll down
        else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    pub fn can_scroll_up(&self) -> bool {
        self.scroll_offset > 0
    }

    pub fn can_scroll_down(&self, visible_height: usize) -> bool {
        self.scroll_offset + visible_height < self.all_sessions.len()
    }

    pub fn update_session_list(&mut self, sessions: Vec<zellij_tile::prelude::SessionInfo>) {
        self.all_sessions = sessions.iter().map(|s| s.name.clone()).collect();
        self.all_sessions.sort();

        if self.selected_index >= self.all_sessions.len() && !self.all_sessions.is_empty() {
            self.selected_index = self.all_sessions.len() - 1;
        }
    }

    pub fn get_selected_session(&self) -> Option<&String> {
        self.all_sessions.get(self.selected_index)
    }

    pub fn get_slot_for_session(&self, session_name: &str) -> Option<usize> {
        for i in 0..5 {
            if let Some(slot_session) = self.hotbar_data.get_session_at_slot(i)
                && slot_session == session_name
            {
                return Some(i);
            }
        }
        None
    }

    pub fn assign_selected_to_slot(&mut self, slot: usize) {
        if let Some(session) = self.get_selected_session() {
            let session = session.clone();
            self.hotbar_data
                .set_session_at_slot(slot, Some(session.clone()));
            self.save_hotbar_data();
            eprintln!("Assigned '{}' to slot {}", session, slot + 1);
        }
    }

    pub fn remove_selected_from_hotbar(&mut self) {
        if let Some(session) = self.get_selected_session()
            && let Some(slot) = self.get_slot_for_session(session)
        {
            self.remove_from_slot(slot);
        }
    }
}
