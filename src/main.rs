use std::path::{PathBuf,Path};
use std::{self, env, process, io, io::Cursor, fs};
use zip_extract;

static MODPACK_URL: &str = "https://dl.vxny.io/lethalcompanymodpack.zip";
fn main() {
    
    let mut game_directory = PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Lethal Company");
    
    if Path::new("Lethal Company.exe").exists() {
        game_directory = env::current_dir().expect("Failed to get current directory");
    }

    let bepinex_folder_path = format!("{}/BepInEx", game_directory.display());

    if Path::new(&bepinex_folder_path).exists() {
        println!("Found existing BepInEx folder. Press enter to delete the folder and continue... ");
        let _ = io::stdin().read_line(&mut String::new());
        fs::remove_dir_all(&bepinex_folder_path).expect("Unable to remove existing BepInEx folder");
    }

    if !game_directory.exists() {
        println!(
            r#"
Could not find Lethal Company directory.

You can instead place this installer in the Lethal Company folder directly.

You can locate the folder by right clicking the game on Steam and pressing 'Manage', then 'Browse local files'
"#
        );

        let _ = io::stdin().read_line(&mut String::new());
        
        process::exit(1)
    }

    println!("Found game files at {:?}", game_directory); 

    println!("Press enter to begin downloading mods... ");
    let _ = io::stdin().read_line(&mut String::new());

    println!("Downloading...");

    let modpack_bytes = download(MODPACK_URL).expect("Failed to download modpack");

    zip_extract::extract(Cursor::new(modpack_bytes), &game_directory, true).expect("Failed to extract modpack");

    println!("Finished installing");
    let _ = io::stdin().read_line(&mut String::new());

}

fn download(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::blocking::get(url).expect("Failed to download");

    response.error_for_status_ref()?;
    
    let bytes = response.bytes()
        .expect("Unable to get modpack bytes")
        .to_vec();

    Ok(bytes)


}