# Hyprrdrop: Hyprland Utilities

I love Hyprland.

## Why?

Experiment. Why not?

## Features

### Special Workspace Toggle

Utilities to manage special workspaces. Create a special workspace with 3 different options:

- Active Window
- Class
- Command (TODO)

#### Active Window

Move the current active window into a special workspace

```sh
# moves the active window into the workspace 'hyprrdrop-term'
hyprrdrop register active term
```

#### Class

Moves all windows that match a WM_CLASS into a special workspace

```sh
# moves all windows with the class kitty-term into the workspace hyprrdrop-term
hyprrdrop register class term "kitty-term"
```

#### Command

Unimplemented.

#### Toggle

Toggles a special workspace in the hyprrdrop namespace.

```sh
# toggles the special workspace hyprrtoggle-term
hyprrdrop toggle term
```

## Building

Ensure Rust and Cargo are installed.

Clone the repo with git or download an archive

```sh
git clone https://github.com/luqmanishere/hyprrdrop.git
```

Build it with cargo:

```sh
cargo b --release # Build the program
cargo r --release # Directly run it
cargo install . # install it to $HOME/.cargo/bin
```
