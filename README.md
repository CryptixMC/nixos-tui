# nixos-tui

A Rust terminal user interface (TUI) application for interactively configuring NixOS systems using **only the repositories owned by [CryptixMC](https://github.com/CryptixMC)**: [`nix-dots`](https://github.com/CryptixMC/nix-dots) and [`nixos-server`](https://github.com/CryptixMC/nixos-server).

> **Note:** This tool is hardcoded to use the above repositories. It does not support arbitrary or third-party configuration repositories.

This tool streamlines the process of setting up a NixOS server or desktop, allowing you to select presets, modules, themes, and user parameters in a guided, user-friendly way.

---

## Features

- **Profile Selection:** Choose between "server" or "desktop" setup.
- **Repo Integration:** Automatically pulls the appropriate configuration repo (`nix-dots` for desktop, `nixos-server` for server).
- **Preset or Custom Host:** Select a preset host configuration or create a new one.
- **Module Selection:** Pick from a list of available NixOS modules to enable.
- **Theme Selection:** Choose a default theme for your system.
- **User Parameters:** Set the username and other custom parameters.

---

## Usage

1. **Run the Application**
   ```sh
   cargo run --release
   ```

2. **Select Profile**
   - Choose either "server" or "desktop".

3. **Repository Pull**
   - The app will automatically clone or update the corresponding repo.

4. **Host Configuration**
   - Select a preset host or create a new host configuration.

5. **Module Selection**
   - Browse and select which NixOS modules you want to enable.

6. **Theme Selection**
   - Pick your preferred default theme.

7. **User Parameters**
   - Enter your username and any other required parameters.

8. **Apply Configuration**
   - The app will generate and apply the configuration, ready for NixOS rebuild.

---

## Requirements

- Rust (latest stable)
- Git
- NixOS system

---

## TODO

- [ ] Set up Rust project structure and dependencies (e.g., `tui`, `crossterm`, `git2`)
- [ ] Implement TUI navigation and selection screens
- [ ] Integrate git operations for repo management (clone, pull, etc.)
- [ ] Discover and list available hosts, modules, and themes from repos
- [ ] Implement preset host selection and custom host creation
- [ ] Module selection UI and logic
- [ ] Theme selection UI and logic
- [ ] User parameter input (username, etc.)
- [ ] Generate NixOS configuration files based on selections
- [ ] Optionally trigger `nixos-rebuild` or provide instructions
- [ ] Error handling and user feedback
- [ ] Add tests and CI
- [ ] Polish UI/UX
