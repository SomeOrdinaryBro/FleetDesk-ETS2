use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::{Path, PathBuf},
};
use regex::Regex;

#[derive(Serialize)]
struct ProfileInfo {
  name: String,
  path: String,
}

fn decode_hex_name(os: &str) -> String {
  // ETS2 profile dirs are hex-encoded UTF-8. If anything looks off, fall back to raw.
  if os.len() % 2 != 0 {
    return os.to_string();
  }
  let mut bytes = Vec::with_capacity(os.len() / 2);
  for i in (0..os.len()).step_by(2) {
    match u8::from_str_radix(&os[i..i + 2], 16) {
      Ok(b) => bytes.push(b),
      Err(_) => return os.to_string(),
    }
  }
  String::from_utf8(bytes).unwrap_or_else(|_| os.to_string())
}

#[tauri::command]
fn find_profiles() -> Vec<ProfileInfo> {
  let mut out = vec![];
  let Some(docs) = dirs::document_dir() else { return out };

  // Classic local profiles + Steam Cloud profiles
  let roots = [
    docs.join("Euro Truck Simulator 2").join("profiles"),
    docs.join("Euro Truck Simulator 2").join("steam_profiles"),
  ];

  // Helpful logs in case nothing shows up
  eprintln!("Fleet Desk: scanning profile roots:");
  for r in &roots {
    eprintln!("  - {:?}", r);
  }

  for base in roots {
    if let Ok(entries) = fs::read_dir(&base) {
      for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
          let raw = p.file_name().unwrap().to_string_lossy().to_string();
          let name = decode_hex_name(&raw);
          out.push(ProfileInfo {
            name,
            path: p.to_string_lossy().to_string(),
          });
        }
      }
    }
  }
  out
}

#[derive(Serialize)]
struct DlcFlags {
  base: bool,
  heavy_cargo: bool,
  special_transport: bool,
  east: bool,
  north: bool,
  fr: bool,
  it: bool,
  balt: bool,
  rbs: bool,
  iberia: bool,
  wb: bool,
  greece: bool,
}

fn ets2_install_from_registry() -> Option<PathBuf> {
  use winreg::enums::HKEY_LOCAL_MACHINE;
  let hk = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
  // Steam base install path
  let steam = hk.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam").ok()?;
  let steam_path: String = steam.get_value("InstallPath").ok()?;
  let common = Path::new(&steam_path).join("steamapps").join("common");
  let ets2 = common.join("Euro Truck Simulator 2");
  if ets2.exists() {
    Some(ets2)
  } else {
    None
  }
}

#[tauri::command]
fn detect_dlcs(explicit_path: Option<String>) -> DlcFlags {
  let base = explicit_path
    .map(PathBuf::from)
    .or_else(ets2_install_from_registry)
    .unwrap_or_else(|| {
      PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Euro Truck Simulator 2")
    });

  let dlc = base.join("dlc");
  let has = |name: &str| dlc.join(name).exists();

  DlcFlags {
    base: true,
    heavy_cargo: has("dlc_heavy_cargo.scs"),
    special_transport: has("dlc_oversize.scs"),
    east: has("dlc_east.scs"),
    north: has("dlc_north.scs"),
    fr: has("dlc_fr.scs"),
    it: has("dlc_it.scs"),
    balt: has("dlc_balt.scs"),
    rbs: has("dlc_blke.scs"),
    iberia: has("dlc_iberia.scs"),
    wb: has("dlc_westbalkans.scs"),
    greece: has("dlc_greece.scs"),
  }
}

#[derive(Serialize)]
struct PlayerState {
  level: u32,
  skills: Vec<String>,
  discovered_cities: Vec<String>,
  owned_trailers: Vec<String>,
  save_format_ok: bool,
}

fn read_text(path: &Path) -> Option<String> {
  fs::read_to_string(path).ok()
}

#[tauri::command]
fn read_player_state(profile_path: String) -> Result<PlayerState, String> {
  let prof = PathBuf::from(profile_path);

  // Check config for g_save_format "2"
  let docs = dirs::document_dir().ok_or("No documents dir")?;
  let cfg_path = docs
    .join("Euro Truck Simulator 2")
    .join("config.cfg");
  let cfg = read_text(&cfg_path).unwrap_or_default();
  let save_format_ok = cfg.contains(r#"uset g_save_format "2""#);

  // Try common save slots
  let candidates = [
    prof.join("save").join("quick").join("game.sii"),
    prof.join("save").join("autosave").join("game.sii"),
    prof.join("save").join("autosave_job").join("game.sii"),
  ];

  let mut save_txt: Option<String> = None;
  for c in &candidates {
    if let Some(t) = read_text(c) {
      save_txt = Some(t);
      break;
    }
  }
  let save_txt = save_txt.ok_or("Couldn't read save. Make a quick save in-game.")?;

  // Minimal scrapes (replace with a real SII parser later)
  let level = Regex::new(r#"(?m)level:\s*(\d+)"#)
    .unwrap()
    .captures(&save_txt)
    .and_then(|c| c.get(1))
    .and_then(|m| m.as_str().parse::<u32>().ok())
    .unwrap_or(1);

  let skills = Regex::new(r#"(?m)skill_points\[\d+\]:\s*(\d+)"#)
    .unwrap()
    .captures_iter(&save_txt)
    .map(|c| c[1].to_string())
    .collect::<Vec<_>>();

  let discovered_cities = Regex::new(r#"(?m)city_discovered\[\d+\]:\s*([a-z0-9_\.]+)"#)
    .unwrap()
    .captures_iter(&save_txt)
    .map(|c| c[1].to_string())
    .collect::<Vec<_>>();

  let owned_trailers = Regex::new(r#"(?m)owned_trailer\[\d+\]:\s*([a-z0-9_\.]+)"#)
    .unwrap()
    .captures_iter(&save_txt)
    .map(|c| c[1].to_string())
    .collect::<Vec<_>>();

  Ok(PlayerState {
    level,
    skills,
    discovered_cities,
    owned_trailers,
    save_format_ok,
  })
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct JobSpec {
  source_city: String,
  source_company: String,
  dest_city: String,
  dest_company: String,
  cargo: String,
  trailer_type: String,
}

#[tauri::command]
fn export_job_spec(profile_path: String, job: JobSpec) -> Result<String, String> {
  // MVP: write JSON next to the profile; writer step comes next.
  let path = PathBuf::from(profile_path).join("fleet_desk_job.json");
  fs::write(&path, serde_json::to_vec_pretty(&job).unwrap()).map_err(|e| e.to_string())?;
  Ok(path.to_string_lossy().to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      find_profiles,
      detect_dlcs,
      read_player_state,
      export_job_spec
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}
