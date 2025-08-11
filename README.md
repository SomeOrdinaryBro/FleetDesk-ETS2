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
