<<<<<<< HEAD
# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
=======
# FleetDesk

A realistic freight dispatch tool for **Euro Truck Simulator 2** — built with [Tauri](https://tauri.app), [Rust](https://www.rust-lang.org), and [React](https://react.dev).  
Designed to feel like a real-world trucking dispatcher console, not a mod menu.

## Features (MVP)
- Scan ETS2 save files for companies, cargos, trailers, and cities
- Suggest realistic freight jobs with pay and XP calculation
- Create custom jobs with full economy data
- Safe write to save files with timestamped backups
- Works fully offline (no internet required)

## Tech Stack
- **UI:** React + TypeScript (Vite) inside Tauri
- **Core Engine:** Rust (MSVC toolchain)
- **Parsing:** Hand-rolled `.sii` parser + `.scs` reader
- **Packaging:** Tauri bundler (Windows-first)
>>>>>>> f9ce1194029d16488f65eb01c7ab42acc71770b1
