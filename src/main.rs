mod classic;
mod nextgen;

fn main() {
    let patch_result = match get_game_version() {
        GameVersion::Classic => classic::do_patch(),
        GameVersion::NextGen => nextgen::do_patch(),
    };

    match patch_result {
        Err(_) => println!(
            "Can't write file, make sure you have permissions (run as adinistrator if on Windows)"
        ),
        _ => (),
    }

    println!("press ENTER to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

enum GameVersion {
    Classic, // 1.32
    NextGen, // 4.0
}

impl From<String> for GameVersion {
    fn from(value: String) -> Self {
        match value.as_str() {
            "1" => GameVersion::Classic,
            _ => GameVersion::NextGen,
        }
    }
}

fn get_game_version() -> GameVersion {
    println!("Select game version (1: Classic, 2: NextGen). Default: 2. NextGen");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Problem reading input");

    match input.as_str().trim() {
        "1" => GameVersion::Classic,
        "2" => GameVersion::NextGen,
        _ => {
            println!("Unknown input {input}, defaulting to NextGen version");

            GameVersion::NextGen
        }
    }
}
