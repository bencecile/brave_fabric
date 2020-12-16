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

    let mut window = Window::new()
        .expect("Failed to create the window");
    let mut emulator_core = find_runnable_core(&rom_path, &window)?;
    let mut last_update = Instant::now();
    let mut until_next_update = Duration::from_secs(0);
    'main_loop: loop {
        for event in window.fetch_current_events() {
            match event {
                Event::WindowClosed => break 'main_loop,
            }
        }

        let delta = last_update.elapsed();
        if delta > until_next_update {
            until_next_update = match emulator_core.on_update(delta - until_next_update) {
                Ok(next_update) => next_update,
                Err(e) => {
                    // TODO Log this properly
                    println!("Error during update: {:?}", e);
                    Duration::from_secs(0)
                },
            };
            last_update = Instant::now();
            // Sleep for as much time as most operating systems will allow without going over
            // Then we will spin for the remaining time (limit of 1ms)
            let rounded_millis = until_next_update.as_millis() as u64;
            // Don't let it underflow. Sleeping for 0ms is also useless
            if rounded_millis > 1 {
                thread::sleep(Duration::from_millis(rounded_millis - 1));
            }
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
fn parse_bios_path_from_args() -> Result<PathBuf, String> {
    if let Some(bios_path_string) = env::args().skip(2).next() {
        let bios_path = PathBuf::from(&bios_path_string);
        if bios_path.is_file() {
            Ok(bios_path)
        } else {
            Err(format!("The BIOS file must exist. Bad arg={}", bios_path_string))
        }
    } else {
        Err("A BIOS for emulation needs to passed as the 2nd argument".to_string())
    }
}

fn find_runnable_core(rom_path: &Path, window: &Window) -> Result<Box<dyn EmulatorCore>, String> {
    let mut error: Option<String> = None;
    match parse_bios_path_from_args() {
        Ok(bios_path) => {
            if let Some(gba_core) = make_gba_core(rom_path, &bios_path, window)? {
                return Ok(Box::new(gba_core));
            }
        },
        Err(e) => error = Some(e)
    }
    // Try the other emulators that don't need a BIOS before throwing up an error

    match error {
        Some(e) => Err(format!("{}. Also failed to find an emulator core", e)),
        None => Err(format!("Failed to find a compatible emulator core for {}", rom_path.display()))
    }
}

/// Optional since it may just be an incompatible rom (which isn't a hard error yet)
fn make_gba_core(rom_path: &Path, bios_path: &Path, window: &Window)
-> Result<Option<GBACore>, String> {
    let gba_settings = GBASettingsBuilder::new()
        .with_rom_path(rom_path)
        // TODO Read this from the settings file
        .with_bios_path(bios_path)
        .build()?;
    match GBACore::create(gba_settings, window) {
        Ok(core) => Ok(Some(core)),
        Err(EmulatorCoreError::IncompatibleRom) => Ok(None),
        Err(e) => Err(format!("Failed to create a GBA core. {:?}", e)),
    }
}
