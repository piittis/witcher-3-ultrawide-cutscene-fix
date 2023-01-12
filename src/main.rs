use std::fs;
use std::io;
use std::path::Path;

// The patch: find a specific 4 byte pattern, then replace the 4 preceding bytes.

fn main() {
    match do_patch() {
        Err(_) => println!(
            "Can't write file, make sure you have permissions (run as adinistrator if on Windows)"
        ),
        _ => (),
    }

    println!("press ENTER to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn do_patch() -> Result<(), io::Error> {
    if !Path::new("./witcher3.exe").exists() {
        println!("Can't access 'witcher3.exe' in current directory. Make sure the tool is in the same directory as 'witcher3.exe' (<game_root>/bin/x64/) and you have permissions (run as administrator if on Windows).");
        return Ok(());
    }

    fs::copy("witcher3.exe", "witcher3_backup.exe")?;
    println!("Backed up 'witcher3' as 'witcher3_backup.exe'");

    let patch = get_patch_bytes();

    println!("processing...");

    let mut original_buffer = fs::read("witcher3.exe").expect("Failed to read witcher3.exe");
    let patch_starts = get_patch_starts(&original_buffer);

    for start in patch_starts {
        let _ = std::mem::replace(&mut original_buffer[start], patch[0]);
        let _ = std::mem::replace(&mut original_buffer[start + 1], patch[1]);
        let _ = std::mem::replace(&mut original_buffer[start + 2], patch[2]);
        let _ = std::mem::replace(&mut original_buffer[start + 3], patch[3]);
    }

    fs::write("witcher3.exe", original_buffer).expect("Failed to overwrite Witcher3.exe");

    println!("success.");
    Ok(())
}

fn get_patch_bytes() -> [u8; 4] {
    loop {
        let mut input = String::new();
        println!(
          "Select resolution (1: 3440x1440, 2: 2560x1080, 3: 5120x1440 or 3840x1080, 4: 3840x1600): "
        );
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

fn get_patch_starts(buffer: &Vec<u8>) -> Vec<usize> {
    buffer
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| match w {
            [0x39, 0x8E, 0xE3, 0x3F] => Some(i),
            _ => None,
        })
        .collect()
}
