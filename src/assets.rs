// # Important stuff
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use rust_embed::Embed;
use std::borrow;

#[derive(Embed)]
#[folder = "assets/images"]
pub struct images;


// # Main logic
pub fn find_image(target_folder: &str, folder: &str, name: &str, randomize: bool) -> Option<Vec<u8>> {
    let folder_clean = folder.replace(" ", "");
    let prefix = format!("{}/{}/", target_folder, folder_clean).to_lowercase();

    if name.is_empty() || randomize {
        let matches: Vec<_> = images::iter()
            .filter(|path: &borrow::Cow<str>| path.as_ref().to_lowercase().replace(" ", "").starts_with(&prefix))
            .collect();
        if matches.is_empty() {
            return None;
        }
        let random_index = rand::random_range(0..matches.len());
        return images::get(matches[random_index].as_ref()).map(|file| file.data.to_vec());
    }

    let target = format!("{}{}.png", prefix, name.replace(" ", "")).to_lowercase();
    images::iter()
        .find(|path: &borrow::Cow<str>| {
            path.as_ref().to_lowercase().replace(" ", "") == target
        })
        .and_then(|path: borrow::Cow<str>| images::get(path.as_ref()))
        .map(|file| file.data.to_vec())
}