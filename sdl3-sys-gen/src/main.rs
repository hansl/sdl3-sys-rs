use std::{
    io::{stderr, IsTerminal},
    path::PathBuf,
    process,
};

fn main() {
    let path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "..", "SDL", "include", "SDL3"]);
    match sdl3_sys_gen::generate(&path) {
        Ok(()) => (),
        Err(e) => {
            if stderr().is_terminal() {
                if cfg!(debug_assertions) {
                    eprintln!("{e:#?}");
                } else {
                    eprintln!("{e:#}");
                }
            } else {
                eprintln!("{e}");
            }
            process::exit(1)
        }
    }
}
