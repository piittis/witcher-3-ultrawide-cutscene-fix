use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::Path;

pub fn do_patch() -> Result<(), io::Error> {
    println!("Running Classic ultrawide patch");

    if !Path::new("./witcher3.exe").exists() {
        println!("Can't access 'witcher3.exe' in current directory. Make sure the tool is in the same directory as 'witcher3.exe' (<game_root>/bin/x64/) and you have permissions (run as administrator if on Windows).");
        return Ok(());
    }

    fs::copy("witcher3.exe", "witcher3_backup.exe")?;
    println!("Backed up 'witcher3' as 'witcher3_backup.exe'");

    let patch = get_patch_bytes();

    println!("processing...");
    let offset = get_patch_start();
    if offset.is_none() {
        println!("Can't find correct position to modify in 'witcher3.exe.'");
        return Ok(());
    }

    //println!("offset: {}", offset.unwrap());

    let mut file = OpenOptions::new().write(true).open("witcher3.exe")?;
    file.seek(io::SeekFrom::Start(offset.unwrap() as u64))?;
    file.write(&patch)?;

    println!("success.");
    Ok(())
}

fn get_patch_bytes() -> [u8; 4] {
    loop {
        let mut input = String::new();
        println!("Select resolution (1: 3440x1440, 2: 2560x1080, 3: 5120x1440 or 3840x1080, 4: 3840x1600): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Problem reading input");
        match input.trim() {
            "1" => return [0x8E, 0xE3, 0x18, 0x40],
            "2" => return [0x24, 0xB4, 0x17, 0x40],
            "3" => return [0x39, 0xBE, 0x63, 0x40],
            "4" => return [0x9a, 0x99, 0x19, 0x40],
            _ => println!("Wrong input, please select 1, 2, 3 or 4"),
        }
    }
}

fn get_patch_start() -> Option<usize> {
    let file = fs::read("witcher3.exe").unwrap();
    for (i, w) in file.windows(4).enumerate() {
        match w {
            [0x55, 0x55, 0x15, 0x40] => return Some(i - 4),
            _ => {}
        }
    }
    None
}
