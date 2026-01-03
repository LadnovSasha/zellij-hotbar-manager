use crate::session_manager::{HotbarManager, ModalMode};

pub fn render_hotbar_modal(manager: &mut HotbarManager, rows: usize, cols: usize) {
    // layout: title (2 lines) + sessions area + help (2 lines) + borders (2 lines)
    // Reserve space for title, help, and borders
    let reserved_lines = 7; // title (2) + help (2) + borders (2) + padding (1)
    let max_session_area_height = rows.saturating_sub(reserved_lines).max(5);

    // Determine how many sessions can be displayed
    let visible_session_count = manager.all_sessions.len().min(max_session_area_height);
    let modal_height = (visible_session_count + reserved_lines).min(rows);

    // Adjust scroll position based on selection
    manager.adjust_scroll(visible_session_count);

    // Render from top-left (1,1) since we're in a floating pane
    let start_row = 1;
    let start_col = 1;

    print!("\u{001b}[2J");

    render_box(start_row, start_col, modal_height, cols);
    render_title(start_row, start_col, cols, manager, visible_session_count);
    render_scroll_indicators(manager, start_row + 2, start_col, cols, visible_session_count);
    render_sessions(manager, start_row + 3, start_col + 2, visible_session_count);
    render_help(manager, start_row + 3 + visible_session_count + 1, start_col + 2);
}

fn render_box(row: usize, col: usize, height: usize, cols: usize) {
    let width = cols.saturating_sub(2);

    print!("\u{001b}[{};{}H╭", row, col);
    for _ in 0..width.saturating_sub(2) {
        print!("─");
    }
    print!("╮");

    for i in 1..height - 1 {
        print!("\u{001b}[{};{}H│", row + i, col);
        print!("\u{001b}[{};{}H│", row + i, col + width - 1);
    }

    print!("\u{001b}[{};{}H╰", row + height - 1, col);
    for _ in 0..width.saturating_sub(2) {
        print!("─");
    }
    print!("╯");
}

fn render_title(
    row: usize,
    col: usize,
    cols: usize,
    manager: &HotbarManager,
    visible_count: usize,
) {
    let title = " Session Hotbar Manager ";
    let width = cols.saturating_sub(2);
    let title_col = col + (width.saturating_sub(title.len())) / 2;
    print!(
        "\u{001b}[{};{}H\u{001b}[1m{}\u{001b}[0m",
        row + 1,
        title_col,
        title
    );

    // Show scroll position on the right side of the title line
    if manager.can_scroll_up() || manager.can_scroll_down(visible_count) {
        let scroll_info = format!(
            "{}/{}",
            manager.selected_index + 1,
            manager.all_sessions.len()
        );
        let info_col = col + width - scroll_info.len() - 3;
        print!(
            "\u{001b}[{};{}H\u{001b}[2m{}\u{001b}[0m",
            row + 1,
            info_col,
            scroll_info
        );
    }
}

fn render_scroll_indicators(
    manager: &HotbarManager,
    row: usize,
    col: usize,
    cols: usize,
    visible_count: usize,
) {
    let width = cols.saturating_sub(2);
    let indicator_col = col + width - 2;

    // Show up arrow if can scroll up
    if manager.can_scroll_up() {
        print!(
            "\u{001b}[{};{}H\u{001b}[1;33m↑\u{001b}[0m",
            row,
            indicator_col
        );
    }

    // Show down arrow if can scroll down (at the bottom of the session area)
    if manager.can_scroll_down(visible_count) {
        print!(
            "\u{001b}[{};{}H\u{001b}[1;33m↓\u{001b}[0m",
            row + visible_count + 1,
            indicator_col
        );
    }
}

fn render_sessions(manager: &HotbarManager, row: usize, col: usize, visible_height: usize) {
    let end_index = (manager.scroll_offset + visible_height).min(manager.all_sessions.len());

    for (display_row, session_index) in (manager.scroll_offset..end_index).enumerate() {
        let session_name = &manager.all_sessions[session_index];
        let is_selected = session_index == manager.selected_index;
        let is_current = manager.active_session.as_ref() == Some(session_name);

        let prefix = if is_selected { "▶ " } else { "  " };

        let slot_indicator = if let Some(slot) = manager.get_slot_for_session(session_name) {
            format!("[{}] ", slot + 1)
        } else {
            "    ".to_string()
        };

        let current_indicator = if is_current { " (current)" } else { "" };

        let line = format!(
            "{}{}{}{}",
            prefix, slot_indicator, session_name, current_indicator
        );

        if is_selected {
            print!(
                "\u{001b}[{};{}H\u{001b}[1;36m{}\u{001b}[0m",
                row + display_row,
                col,
                line
            );
        } else if is_current {
            print!(
                "\u{001b}[{};{}H\u{001b}[1m{}\u{001b}[0m",
                row + display_row,
                col,
                line
            );
        } else {
            print!("\u{001b}[{};{}H{}", row + display_row, col, line);
        }
    }
}

fn render_help(manager: &HotbarManager, row: usize, col: usize) {
    let help_text = if manager.modal_mode == ModalMode::Move {
        vec![
            "Move Mode - Select destination slot:",
            "↑/↓: Navigate  Enter: Confirm  Esc: Cancel",
        ]
    } else {
        vec![
            "1-5: Assign to hotbar slot  x: Remove from hotbar",
            "↑/↓: Navigate  Enter: Switch session  Esc/q: Close",
        ]
    };

    for (i, text) in help_text.iter().enumerate() {
        print!(
            "\u{001b}[{};{}H\u{001b}[2m{}\u{001b}[0m",
            row + i,
            col,
            text
        );
    }
}
