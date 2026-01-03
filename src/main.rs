mod persistence;
mod session_manager;
mod ui;

use session_manager::HotbarManager;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

register_plugin!(HotbarManager);

const MODE_SWITCH_COMMAND: &str = "hotbar_switch_mode";
const OPEN_RECENT: &str = "open_recent_hotbar";
const SWITCH_SLOT_PREFIX: &str = "switch_slot_";
const TOGGLE_UI: &str = "toggle_ui";

impl ZellijPlugin for HotbarManager {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);

        for (key, value) in configuration {
            if key.starts_with(session_manager::PREFIX_KEY) {
                self.saved_sessions.insert(key, value);
            }
        }

        self.load_hotbar_data();

        subscribe(&[EventType::SessionUpdate, EventType::Key, EventType::Visible]);

        // Plugin starts as headless background service
        self.is_visible = false;
        set_selectable(false);
    }

    fn pipe(&mut self, message: PipeMessage) -> bool {
        let name = message.name.as_str();

        if name == TOGGLE_UI {
            if self.is_visible {
                self.hide_ui();
            } else {
                self.show_ui();
            }
            return true; // Request a re-render
        }

        if let Some(slot_str) = name.strip_prefix(SWITCH_SLOT_PREFIX)
            && let Ok(slot @ 1..=5) = slot_str.parse::<usize>()
        {
            // Reload hotbar data to get latest assignments from other plugin instances
            self.load_hotbar_data();
            self.switch_to_slot(slot - 1);
            return false;
        }

        if name.starts_with(session_manager::PREFIX_KEY) {
            self.load_hotbar_data();
            self.switch_to(name);
            return false;
        }

        if name == OPEN_RECENT {
            self.load_hotbar_data();
            self.switch_to_previous_session();
            return false;
        }

        if name == MODE_SWITCH_COMMAND {
            let current = &self.hotbar_mode;
            let new_mode = match current {
                session_manager::HotBarMode::Session => "tab",
                session_manager::HotBarMode::Tab => "session",
            };
            self.set_hotbar_mode(new_mode);
            return false;
        }

        false
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Visible(is_visible) => {
                self.is_visible = is_visible;
                set_selectable(is_visible);
                true
            }
            Event::SessionUpdate(sessions, _) => {
                self.update_session_list(sessions.clone());
                sessions
                    .into_iter()
                    .find(|s| s.is_current_session)
                    .map(|session| self.set_active_session(&session.name));
                false
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Esc => {
                    self.hide_ui();
                    true
                }
                BareKey::Up => {
                    self.navigate_sessions(-1);
                    true
                }
                BareKey::Down => {
                    self.navigate_sessions(1);
                    true
                }
                BareKey::Char('q') => {
                    self.hide_ui();
                    true
                }
                BareKey::Char('x') => {
                    self.remove_selected_from_hotbar();
                    true
                }
                BareKey::Char('1') => {
                    self.assign_selected_to_slot(0);
                    true
                }
                BareKey::Char('2') => {
                    self.assign_selected_to_slot(1);
                    true
                }
                BareKey::Char('3') => {
                    self.assign_selected_to_slot(2);
                    true
                }
                BareKey::Char('4') => {
                    self.assign_selected_to_slot(3);
                    true
                }
                BareKey::Char('5') => {
                    self.assign_selected_to_slot(4);
                    true
                }
                BareKey::Enter => {
                    if let Some(session) = self.get_selected_session() {
                        let session = session.clone();
                        self.hide_ui();
                        switch_session(Some(&session));
                    }
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // Only render UI when plugin is explicitly opened (is_visible = true)
        // When running as headless background service, skip rendering
        if !self.is_visible {
            return;
        }
        ui::render_hotbar_modal(self, rows, cols);
    }
}
