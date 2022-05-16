use gameboy::Gameboy;
mod gameboy;
use std::env;

fn main() {
    let mut gameboy = Gameboy::new();
    let path = env::current_dir();
    match path {
        Ok(v) => {
            println!("The current directory is {}", v.display());
        }
        Err(e) => {println!("{}:Could not read path!",e)}
    }
    

    gameboy.load_boot_rom("roms/dmg_boot.bin").expect("Failed to load boot ROM");
    gameboy.run();
}
