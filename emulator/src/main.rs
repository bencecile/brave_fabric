use std::{
    env,
    path::{PathBuf},
    thread,
};
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
    while !is_window_closed {
        for event in window.fetch_current_events() {
            match event {
                Event::WindowClosed => is_window_closed = true,
            }
        }
        thread::yield_now();
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
