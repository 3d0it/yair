use crate::configuration::Config;
use image::{imageops::FilterType, GenericImageView};
use std::{error::Error, path::Path};

#[derive(Debug)]
pub struct ImageDescriptor {
    name: String,
    extension: String,
}

pub fn calculate_file_name_extension(path: &str) -> Result<ImageDescriptor, Box<dyn Error>> {
    let file_path = Path::new(path);

    let extension = file_path
        .extension()
        .ok_or("Image must have an extension")?
        .to_str()
        .ok_or("Unable to convert image extension to str")?
        .to_string();

    let name = file_path
        .with_extension("")
        .file_name()
        .ok_or("Unable to retrive image name")?
        .to_str()
        .ok_or("Unable to convert image name to str")?
        .to_string();

    Ok(ImageDescriptor { name, extension })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Load {}", config.image_path);

    let img = image::open(&config.image_path)?;
    let (width, height) = img.dimensions();

    let image_descriptor = calculate_file_name_extension(&config.image_path)?;

    println!("Start resizing...");

    let img_resized = img.resize(
        (width * config.percentage / 100) as u32,
        (height * config.percentage / 100) as u32,
        FilterType::Lanczos3,
    );

    println!("Save resized image...");
    img_resized.save(format!(
        "{}_{}.{}",
        image_descriptor.name, config.percentage, image_descriptor.extension
    ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn get_file_name_extension() {
        let img_path = ".\\image.jpg";

        let img = calculate_file_name_extension(img_path).unwrap();
        assert_eq!(String::from("jpg"), img.extension);
        assert_eq!(String::from("image"), img.name);
    }

    #[test]
    fn get_file_name_extension_not_found() {
        let img_path = ".\\image";

        assert_err!(calculate_file_name_extension(img_path));
    }

    #[test]
    fn run_resizer() {
        let config = Config {
            image_path: String::from(".\\sample\\bear-combat.jpg"),
            percentage: 25,
        };

        assert_ok!(run(config));
    }
}
