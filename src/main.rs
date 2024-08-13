mod arc;
mod deswizzle;
mod gamecube;
mod rle_textures;
mod the_sims;
mod the_sims_2;
mod the_sims_2_castaway;
mod the_sims_2_pets;
mod the_sims_3;
mod the_sims_bustin_out;
mod the_urbz;

fn decompress_bc1(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut decompressed_pixels = vec![0u8; width * height * 4];
    texpresso::Format::Bc1.decompress(bytes, width, height, &mut decompressed_pixels);
    let image = image::RgbaImage::from_raw(width as u32, height as u32, decompressed_pixels).unwrap();
    image::imageops::flip_vertical(&image)
}

fn decompress_bc2(bytes: &[u8], width: usize, height: usize) -> image::RgbaImage {
    let mut decompressed_pixels = vec![0u8; width * height * 4];
    texpresso::Format::Bc2.decompress(bytes, width, height, &mut decompressed_pixels);
    let image = image::RgbaImage::from_raw(width as u32, height as u32, decompressed_pixels).unwrap();
    image::imageops::flip_vertical(&image)
}

fn save_texture(image: image::RgbaImage, name: &str, output_path: &std::path::Path, specular: bool) {
    if specular {
        let mut diffuse = image::RgbImage::new(image.width(), image.height());
        let mut specular = image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::new(image.width(), image.height());
        let mut alpha_0_count = 0;
        let mut alpha_255_count = 0;
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                diffuse.put_pixel(x, y, image::Rgb(pixel.0[0..3].try_into().unwrap()));
                specular.put_pixel(x, y, image::Luma([pixel.0[3]]));

                if pixel.0[3] == 0 {
                    alpha_0_count += 1
                }

                if pixel.0[3] == 255 {
                    alpha_255_count += 1
                }
            }
        }

        let pixel_count = image.width() * image.height();

        diffuse.save(output_path.join(format!("{}.png", name))).unwrap();
        if !(alpha_0_count == pixel_count || alpha_255_count == pixel_count) {
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

#[allow(clippy::enum_variant_names)]
#[derive(clap::Subcommand)]
enum CliCommands {
    TheSims {
        datasets_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsRle {
        rletextures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsBustinOut {
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsBustinOutRle {
        rletextures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheUrbz {
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-2")]
    TheSims2 {
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-2-pets")]
    TheSims2Pets {
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-2-castaway")]
    TheSims2Castaway {
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-3")]
    TheSims3 {
        textures_path: std::path::PathBuf,
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
        CliCommands::TheSimsRle {
            rletextures_path,
            output_path,
        } => {
            the_sims::extract_rle_textures(rletextures_path, output_path);
        }
        CliCommands::TheSimsBustinOut {
            textures_path,
            output_path,
        } => {
            the_sims_bustin_out::extract_textures(textures_path, output_path);
        }
        CliCommands::TheSimsBustinOutRle {
            rletextures_path,
            output_path,
        } => {
            the_sims_bustin_out::extract_rle_textures(rletextures_path, output_path);
        }
        CliCommands::TheUrbz {
            textures_path,
            output_path,
        } => {
            the_urbz::extract_textures(textures_path, output_path);
        }
        CliCommands::TheSims2 {
            textures_path,
            output_path,
        } => {
            the_sims_2::extract_textures(textures_path, output_path);
        }
        CliCommands::TheSims2Pets {
            textures_path,
            output_path,
        } => {
            the_sims_2_pets::extract_textures(textures_path, output_path, &the_sims_2_pets::SPECULAR_FILE_NAMES);
        }
        CliCommands::TheSims2Castaway {
            textures_path,
            output_path,
        } => {
            the_sims_2_pets::extract_textures(textures_path, output_path, &the_sims_2_castaway::SPECULAR_FILE_NAMES);
        }
        CliCommands::TheSims3 {
            textures_path,
            output_path,
        } => {
            the_sims_3::extract_textures(textures_path, output_path);
        }
    }
}
