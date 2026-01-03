# zellij-hotbar-manager

A [Harpoon](https://github.com/ThePrimeagen/harpoon)-inspired session manager plugin for [Zellij](https://zellij.dev/). Quickly switch between your most-used sessions using hotbar slots.

## Features

- **5 Hotbar Slots** - Assign sessions to slots 1-5 for instant switching with `Ctrl+1` through `Ctrl+5`
- **Previous Session** - Jump back to your last session with `Ctrl+0`
- **Management UI** - Floating modal to view, assign, and remove hotbar entries
- **Persistent Storage** - Hotbar assignments survive Zellij restarts
- **Headless Operation** - Runs as a background service, UI appears only when needed

## Installation

### Requirements

- Rust toolchain with `wasm32-wasip1` target
- Zellij 0.43.0+

### Build

```bash
git clone https://github.com/LadnovSasha/zellij-hotbar-manager.git
cd zellij-hotbar-manager
./build.sh
```

Or manually:

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
cp target/wasm32-wasip1/release/zellij-hotbar-manager.wasm ~/.config/zellij/plugins/zellij-hotbar-manager.wasm
```

## Configuration

Add the following to your `~/.config/zellij/config.kdl`:

### Plugin Registration

```kdl
plugins {
    hotbar-manager location="file:~/.config/zellij/plugins/zellij-hotbar-manager.wasm"
}
```

### Load on Startup

```kdl
load_plugins {
    hotbar-manager
}
```

### Keybindings

```kdl
keybinds {
    shared_except "locked" "move" {
        // Switch to hotbar slots 1-5
        bind "Ctrl 1" {
            MessagePlugin "hotbar-manager" { name "switch_slot_1"; }
            SwitchToMode "normal";
        }
        bind "Ctrl 2" {
            MessagePlugin "hotbar-manager" { name "switch_slot_2"; }
            SwitchToMode "normal";
        }
        bind "Ctrl 3" {
            MessagePlugin "hotbar-manager" { name "switch_slot_3"; }
            SwitchToMode "normal";
        }
        bind "Ctrl 4" {
            MessagePlugin "hotbar-manager" { name "switch_slot_4"; }
            SwitchToMode "normal";
        }
        bind "Ctrl 5" {
            MessagePlugin "hotbar-manager" { name "switch_slot_5"; }
            SwitchToMode "normal";
        }

        // Switch to previous session
        bind "Ctrl 0" {
            MessagePlugin "hotbar-manager" { name "open_recent_hotbar"; }
            SwitchToMode "normal";
        }
    }

    session {
        // Toggle hotbar manager UI
        bind "h" {
            MessagePlugin "hotbar-manager" { name "toggle_ui"; }
            SwitchToMode "normal"
        }
    }
}
```

## Usage

### Quick Switching

| Keybinding                | Action                               |
| ------------------------- | ------------------------------------ |
| `Ctrl+1` through `Ctrl+5` | Switch to session in hotbar slot 1-5 |
| `Ctrl+0`                  | Switch to previous session           |
| `Ctrl+o` then `h`         | Open hotbar manager UI               |

### Manager UI Controls

When the UI is open:

| Key         | Action                                 |
| ----------- | -------------------------------------- |
| `↑` / `↓`   | Navigate session list                  |
| `1` - `5`   | Assign selected session to hotbar slot |
| `x`         | Remove selected session from hotbar    |
| `Enter`     | Switch to selected session             |
| `Esc` / `q` | Close UI                               |

The UI displays all available sessions with:

- `[N]` prefix showing which hotbar slot (if assigned)
- `(current)` suffix for the active session
- `▶` marker for the selected item

## Screenshots

_Coming soon_

## Integration with Neovim

When running Neovim inside Zellij, the `Ctrl+1` through `Ctrl+5` keybindings are captured by Neovim before reaching Zellij. To make session switching work seamlessly from within Neovim, add the following to your Neovim configuration.

### Neovim Keymaps (Lua)

Add this to your `~/.config/nvim/lua/config/keymaps.lua` or equivalent:

```lua
-- Zellij session switching keymaps (only when inside Zellij)
if os.getenv('ZELLIJ') then
  -- Create keymaps for Ctrl+1 through Ctrl+5 to switch sessions
  for i = 1, 5 do
    vim.keymap.set({ 'n', 'i', 't', 'v' }, string.format('<C-%d>', i), function()
      vim.fn.system(string.format('zellij action pipe --name "switch_slot_%d"', i))
    end, { silent = true, desc = string.format('Switch to Zellij session %d', i) })
  end

  -- Ctrl+0 to switch to previous session
  vim.keymap.set({ 'n', 'i', 't', 'v' }, '<C-0>', function()
    vim.fn.system('zellij action pipe --name "open_recent_hotbar"')
  end, { silent = true, desc = 'Switch to previous Zellij session' })
end
```

### How It Works

1. The `ZELLIJ` environment variable is automatically set when running inside Zellij
2. When the keymaps are triggered, Neovim uses `zellij action pipe` to send messages to the plugin
3. The plugin receives the message via its `pipe()` method and executes the session switch
4. This works in all Neovim modes: normal, insert, terminal, and visual

### Auto-Lock Integration (Optional)

For the best experience, consider using [zellij-autolock](https://github.com/fresh2dev/zellij-autolock) alongside this plugin. It automatically locks Zellij when Neovim has focus, preventing keybinding conflicts while still allowing the session switching keymaps to work:

```kdl
plugins {
    autolock location="file:~/.config/zellij/plugins/zellij-autolock.wasm" {
        is_enabled true
        triggers "nvim|vim|git|fzf|zoxide|atuin"
        reaction_seconds "0.3"
    }
}

load_plugins {
    autolock
    hotbar-manager
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.
