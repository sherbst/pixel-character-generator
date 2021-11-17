use image::imageops;
use image::io::Reader as ImageReader;
use image::{DynamicImage, RgbImage};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

const LAYERS_PATH: &str = "./layers";
const OUT_PATH: &str = "./out";

#[derive(Debug)]
struct Layer {
    index: u8,
    name: String,
    dir_name: String,
}

struct Selection {
    layer: String,
    filename: String,
}

fn get_dir_contents(path: &Path) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .into_iter()
        .filter_map(|file| {
            let file = file.unwrap();

            match file.file_type().unwrap().is_dir() {
                true => None,
                false => Some(file.file_name().into_string().unwrap()),
            }
        })
        .collect())
}

fn get_layers() -> io::Result<Vec<String>> {
    let mut layers: Vec<Layer> = fs::read_dir(LAYERS_PATH)?
        .into_iter()
        .filter_map(|file| {
            let file = file.unwrap();

            if !file.file_type().unwrap().is_dir() {
                return None;
            }

            let dir_name = file.file_name().into_string().unwrap();

            let index = dir_name[..4].parse::<u8>().unwrap();
            let name = dir_name[4..].to_owned();

            return Some(Layer {
                index,
                name,
                dir_name,
            });
        })
        .collect();

    layers.sort_by(|a, b| a.index.cmp(&b.index));

    let layers = layers.into_iter().map(|layer| layer.dir_name).collect();

    Ok(layers)
}

fn generate_characters(layers: &Vec<String>) -> io::Result<()> {
    let permutations = get_permutations(&Vec::new(), &layers)?;

    println!(
        "Generating {} permutations of characters...",
        permutations.len()
    );

    for (i, permutation) in permutations.iter().enumerate() {
        let mut selections: Vec<Selection> = layers
            .iter()
            .zip(permutation)
            .into_iter()
            .map(|(layer, filename)| Selection {
                layer: layer.to_owned(),
                filename: filename.to_owned(),
            })
            .collect();

        let mut img = DynamicImage::ImageRgb8(RgbImage::new(32, 32));

        for selection in selections {
            let in_path: PathBuf = [LAYERS_PATH, &selection.layer, &selection.filename]
                .iter()
                .collect();

            let overlay = ImageReader::open(in_path)?.decode().unwrap();

            imageops::overlay(&mut img, &overlay, 0, 0);
        }

        let out_path: PathBuf = [OUT_PATH, &format!("{}.png", i)].iter().collect();
        img.save(out_path).unwrap();
    }

    Ok(())
}

fn get_permutations(
    current_layers_selections: &Vec<String>,
    remaining_layers: &Vec<String>,
) -> io::Result<Vec<Vec<String>>> {
    let mut permutations = Vec::new();

    if remaining_layers.len() == 0 {
        return Ok(vec![current_layers_selections.clone()]);
    }

    let layer_dir_path: PathBuf = [LAYERS_PATH, &remaining_layers[0]].iter().collect();
    let layer_options = get_dir_contents(&layer_dir_path)?;

    for option in layer_options {
        let mut new_layers_selections = current_layers_selections.clone();
        new_layers_selections.push(option);

        let mut new_remaining_layers = remaining_layers.clone();
        new_remaining_layers.remove(0);

        let mut additional_permutations =
            get_permutations(&new_layers_selections, &new_remaining_layers)?;

        permutations.append(&mut additional_permutations);
    }

    Ok(permutations)
}

fn main() -> io::Result<()> {
    let layers = get_layers()?;
    generate_characters(&layers)?;

    Ok(())
}
