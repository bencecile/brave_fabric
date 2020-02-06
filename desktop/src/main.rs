use std::{
    process::{Command},
};

fn main() {
    make_emulator_command(r"C:\BraveFabric\emulator_games\Metroid Fusion (Japan).gba")
        .spawn()
        .expect("Failed to spawn the emulator")
        .wait()
        .expect("Failed to wait for the emulator");
}

// The emulator MUST always be in the same directory to run correctly
#[cfg(windows)]
fn make_emulator_command(rom_path: &str) -> Command {
    let mut command = Command::new("brave_emulator.exe");
    command.arg(rom_path);
    command
}
