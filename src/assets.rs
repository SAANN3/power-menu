pub struct LocalAssets {}
use std::{env, fs::{create_dir, create_dir_all}, path::{Path, PathBuf}};

use include_dir::{include_dir, Dir};

impl LocalAssets {
    fn get_assets_path() -> PathBuf {
        dirs::config_dir().expect("Failed to get path to config dir").join("power-menu")
    }

    pub fn extract_assets() {
        // seems like there is no known way to make dioxus work without extracting assets :/
        const ASSETS: Dir = include_dir!("./assets");
        let mut path = LocalAssets::get_assets_path();
        let path: PathBuf = path.join("assets");
        if !path.exists() {
            create_dir_all(&path).expect(&format!("Can't create folder: {:?}", &path));
            ASSETS.extract(&path).expect(&format!("Can't extract assets to {:?}", &path));
        }
    }

    pub fn get_path<T: AsRef<Path>>(relative: T) -> String {
        let mut path = LocalAssets::get_assets_path();
        let path = path.join(relative);
        path.to_str().unwrap().to_string()
    }
}