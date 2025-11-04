# nixos-tui

A Rust terminal user interface (TUI) application for interactively configuring NixOS systems using only the official repositories owned by [CryptixMC](https://github.com/CryptixMC): [`nixos-desktop`](https://github.com/CryptixMC/nixos-desktop) and [`nixos-server`](https://github.com/CryptixMC/nixos-server).

**nixos-tui** is designed to be a single, self-contained executable that you can run from your terminal—no Rust installation required. It streamlines the process of setting up or reconfiguring a NixOS server or desktop, allowing you to select presets, modules, themes, user parameters, and more in a guided, user-friendly way.

---

## Features

- **One Executable:**
  Download and run a single binary, or install via Nix. No Rust required.
- **Automatic Git Handling:**
  Checks for `git` and offers to install it if missing.
- **Repo Integration:**
  Uses only the official `nixos-desktop` and `nixos-server` repos for modules and configuration.
- **Automatic Updates:**
  Every run, the tool performs a `git pull` to ensure you have the latest modules and presets.
- **Smart Reconfiguration:**
  If an existing configuration is detected, skips initial setup and opens the modules menu for quick reconfiguration.
- **Guided Workflow:**
  Step-by-step selection of profile, host name, modules (preset or custom), extra packages, user, home-manager modules, and theme.
- **Automated Setup:**
  Creates user, clones repo, creates host directory, and writes configs as needed.
- **Summary & Confirmation:**
  Always shows a summary before applying changes.

---

## Workflow

1. **Select Profile**
   - Choose between "server" or "desktop" setup.

2. **Enter Host Name**
   - Type the name for your new host configuration.

3. **Module Selection**
   - Choose a preconfigured module set (e.g., "gaming", "minimal", "dev", etc.) or
   - Customize by toggling individual modules (modules are read from the repo’s `modules/` directory, with `.nix` stripped).

4. **Extra Packages**
   - Input any extra packages you want to include (comma or space separated).

5. **Enter User Name**
   - Type the primary user’s name.

6. **Home-Manager Modules**
   - Select from a list of available home-manager modules.

7. **Select Theme**
   - Choose your default system theme.

8. **Summary**
   - Review a brief summary of your configuration.

9. **Apply**
   - Press Enter to:
     - Create the user (if not already present).
     - Clone the respective repo into the user’s home directory.
     - Create a new host directory named after the host.
     - Set up the configuration files accordingly.

---

## Distribution & Usage

- **Download the latest release** from [GitHub Releases](https://github.com/CryptixMC/nixos-tui/releases)
  or install via Nix:
  ```sh
  nix run github:CryptixMC/nixos-tui
  ```
  or
  ```sh
  nix profile install github:CryptixMC/nixos-tui
  ```

- **No Rust Needed:**
  The distributed binary is fully self-contained. You do not need Rust installed to use it.

- **Automatic Git Handling:**
  On first run, the tool checks for `git` and offers to install it if missing (using Nix or your system package manager).

- **Automatic Updates:**
  Every time you run the tool, it will `git pull` the respective repo to ensure you have the latest modules and presets.

- **Reconfiguration:**
  If an existing configuration is detected, the tool skips the initial setup and opens the modules menu for quick reconfiguration. You can select/deselect modules and review the overview before applying changes.

---

## Example Usage

```sh
nixos-tui
```
or
```sh
nix run github:CryptixMC/nixos-tui
```

---

## Requirements

- NixOS system
- Git (will be installed if missing)
- (Optional) Nix for easiest installation and updates

---

## TODO

- [ ] Implement TUI navigation and selection screens for each step
- [ ] Integrate git operations for repo management (clone, pull, etc.)
- [ ] Discover and list available modules from repo
- [ ] Implement preset and custom module selection
- [ ] Extra packages input UI and logic
- [ ] User and home-manager module selection UI
- [ ] Theme selection UI and logic
- [ ] Generate NixOS configuration files based on selections
- [ ] Apply configuration: create user, clone repo, set up host directory
- [ ] Error handling and user feedback
- [ ] Add tests and CI
- [ ] Polish UI/UX

---

## License

MIT

---

**nixos-tui** is built for reproducible, user-friendly NixOS configuration.
Questions, suggestions, or contributions? [Open an issue or pull request!](https://github.com/CryptixMC/nixos-tui)
