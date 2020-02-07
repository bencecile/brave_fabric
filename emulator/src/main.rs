use std::{
    env,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};
use brave_emulator_common::{EmulatorCore, EmulatorCoreError};
use brave_emulator_gba::{GBACore, GBASettingsBuilder};
use brave_windowing::{
    Event, Window,
};

// TODO Be able to pause the emulator (press a key to get in and out)
fn main() -> Result<(), String> {
    // Skip the first argument since it can't be relied upon
    let rom_path = parse_rom_path_from_args()?;

    let mut is_window_closed = false;

    let mut window = Window::new()
        .expect("Failed to create the window");
    let mut emulator_core = find_runnable_core(&rom_path, &window)?;
    let mut last_update = Instant::now();
    let mut until_next_update = Duration::from_secs(0);
    while !is_window_closed {
        for event in window.fetch_current_events() {
            match event {
                Event::WindowClosed => is_window_closed = true,
            }
        }

        let delta = last_update.elapsed();
        if delta > until_next_update {
            until_next_update = emulator_core.on_update(delta);
            last_update = Instant::now();
            // Sleep for as much time as most operating systems will allow without going over
            // Then we will spin for the remaining time
            let rounded_millis = until_next_update.as_millis() as u64 - 1;
            thread::sleep(Duration::from_millis(rounded_millis));
        }
    }

    Ok(())
}

fn parse_rom_path_from_args() -> Result<PathBuf, String> {
    if let Some(rom_path_string) = env::args().skip(1).next() {
        let rom_path = PathBuf::from(&rom_path_string);
        if rom_path.is_file() {
            Ok(rom_path)
        } else {
            Err(format!("The ROM file must exist. Bad arg={}", rom_path_string))
        }
    } else {
        Err("A ROM to be emulated needs to passed as the 1st argument".to_string())
    }
}

fn find_runnable_core(rom_path: &Path, window: &Window) -> Result<Box<dyn EmulatorCore>, String> {
    if let Some(gba_core) = make_gba_core(rom_path, window)? {
        Ok(Box::new(gba_core))
    } else {
        Err(format!("Failed to find a compatible emulator core for {}", rom_path.display()))
    }
}

/// Optional since it may just be an incompatible rom (which isn't a hard error yet)
fn make_gba_core(rom_path: &Path, window: &Window) -> Result<Option<GBACore>, String> {
    let gba_settings = GBASettingsBuilder::new()
        .with_rom_path(rom_path)
        // TODO Read this from the settings file
        .with_bios_dir("emulator_games")
        .build()?;
    match GBACore::create(gba_settings, window) {
        Ok(core) => Ok(Some(core)),
        Err(EmulatorCoreError::IncompatibleRom) => Ok(None),
        Err(e) => Err(format!("Failed to create a GBA core. {:?}", e)),
    }
}
