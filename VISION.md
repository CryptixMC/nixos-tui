# VISION.md

## nixos-tui: Vision & Workflow

This document outlines the exact workflow, user experience, and technical ideas for the `nixos-tui` project, as envisioned by the author.

---

### 1. **Distribution & Execution**

- **Single Executable:**  
  - The tool is distributed as a single, self-contained binary.
  - Users can run it directly from the terminal, or install it via Nix for easy updates.
  - No Rust installation is required on the user's system.

- **Git Handling:**  
  - On first run, the tool checks for `git` and offers to install it if missing (using Nix or the system package manager).
  - Every time the tool runs, it performs a `git pull` on the configuration repo to ensure the latest modules and presets.

- **Repo Integration:**  
  - Only the official `nixos-server` and `nixos-desktop` repositories are supported.
  - The tool automatically selects the correct repo based on the user's profile choice.

---

### 2. **Smart Reconfiguration**

- If the tool detects that it is already installed and configured on the system:
  - It skips the initial setup steps.
  - It opens directly to the modules menu, allowing the user to select/deselect modules and review the configuration overview.
  - The user can quickly reconfigure their system and apply changes.

---

### 3. **Guided Workflow**

#### **Step-by-Step Process**

1. **Select Profile**
   - User chooses between "server" or "desktop".

2. **Enter Host Name**
   - User is prompted to enter a name for the host configuration.

3. **Module Selection**
   - User can select a preconfigured module set (e.g., "gaming", "minimal", "dev", etc.) OR
   - User can customize by toggling individual modules.
   - The list of modules is read from the repo’s `modules/` directory, with `.nix` stripped from filenames.

4. **Extra Packages**
   - User can input any extra packages to be included (comma or space separated).

5. **Enter User Name**
   - User is prompted to enter the primary user's name.

6. **Home-Manager Modules**
   - User can select from a list of available home-manager modules (either hardcoded or read from the repo).

7. **Select Theme**
   - User chooses a default system theme.

8. **Summary**
   - The tool displays a brief summary of all selections and configuration.

9. **Apply**
   - On confirmation:
     - The tool creates the user if not already present.
     - Clones the respective repo into the user's home directory.
     - Creates a new host directory named after the host.
     - Sets up the configuration files accordingly.

---

### 4. **User Experience Goals**

- **Fast and Friendly:**  
  - Minimal required input, with sensible defaults and presets.
  - Clear navigation and feedback at every step.
  - Always shows a summary before making changes.

- **Reproducible and Updatable:**  
  - Uses only official, version-controlled NixOS configuration repositories.
  - Ensures the latest modules and presets are always available via `git pull`.
  - Can be updated itself via Nix or by downloading a new release.

- **Safe:**  
  - Never overwrites existing user data without confirmation.
  - Provides clear error messages and guidance if something goes wrong.

---

### 5. **Technical Notes**

- **Module Discovery:**  
  - The tool reads the `modules/` directory from the selected repo, stripping `.nix` to present module names.
- **Preset Module Sets:**  
  - Predefined sets (e.g., "gaming", "minimal") are available for quick selection.
- **Config Generation:**  
  - The tool generates NixOS configuration files based on user selections.
- **Automation:**  
  - Handles user creation, repo cloning, and config setup automatically.

---

### 6. **Future Ideas**

- **Self-update feature:**  
  - The tool could offer to update itself to the latest release.
- **More Presets:**  
  - Additional module sets and themes could be added over time.
- **Community Contributions:**  
  - Eventually, allow for community-submitted module sets or themes (with curation).

---

**nixos-tui** aims to make NixOS configuration accessible, reproducible, and enjoyable for everyone—from new users to advanced sysadmins.
