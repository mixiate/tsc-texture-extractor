mod deswizzle;
mod the_sims;

fn save_texture(image: image::RgbaImage, name: &str, output_path: &std::path::Path, specular: bool) {
    if specular {
        let mut diffuse = image::RgbImage::new(image.width(), image.height());
        let mut specular = image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::new(image.width(), image.height());
        let mut output_specular = false;
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                diffuse.put_pixel(x, y, image::Rgb(pixel.0[0..3].try_into().unwrap()));
                specular.put_pixel(x, y, image::Luma([pixel.0[3]]));

                if pixel.0[3] != 255 {
                    output_specular = true;
                }
            }
        }

        diffuse.save(output_path.join(format!("{}.png", name))).unwrap();
        if output_specular {
            specular.save(output_path.join(format!("{} specular.png", name))).unwrap();
        }
    } else {
        let mut has_alpha = false;
        for pixel in image.pixels() {
            if pixel.0[3] != 255 {
                has_alpha = true;
            }
        }

        if has_alpha {
            image.save(output_path.join(format!("{}.png", name))).unwrap();
        } else {
            let mut diffuse = image::RgbImage::new(image.width(), image.height());
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let pixel = image.get_pixel(x, y);
                    diffuse.put_pixel(x, y, image::Rgb(pixel.0[0..3].try_into().unwrap()));
                }
            }

            diffuse.save(output_path.join(format!("{}.png", name))).unwrap();
        }
    }
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(clap::Subcommand)]
enum CliCommands {
    TheSims {
        datasets_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
}

fn main() {
    use clap::Parser;
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::TheSims {
            datasets_path,
            output_path,
        } => {
            the_sims::extract_textures(datasets_path, output_path);
        }
    }
}
