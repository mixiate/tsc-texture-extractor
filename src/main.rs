mod arc;
mod datasets;
mod gamecube;
mod playstation_2;
mod rle_textures;
mod the_sims;
mod the_sims_2;
mod the_sims_2_castaway;
mod the_sims_2_pets;
mod the_sims_3;
mod the_sims_bustin_out;
mod the_urbz;
mod xbox;

pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn u32_from_bytes(&self, bytes: [u8; 4]) -> u32 {
        match self {
            Endianness::Little => u32::from_le_bytes(bytes),
            Endianness::Big => u32::from_be_bytes(bytes),
        }
    }
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

#[derive(Clone, clap::ValueEnum)]
enum Console0 {
    #[clap(name = "ps2")]
    PlayStation2,
    #[clap(name = "gamecube")]
    GameCube,
    Xbox,
}

#[derive(Clone, clap::ValueEnum)]
enum Console1 {
    #[clap(name = "ps2")]
    PlayStation2,
    #[clap(name = "gamecube")]
    GameCube,
    Wii,
}

#[allow(clippy::enum_variant_names)]
#[derive(clap::Subcommand)]
enum CliCommands {
    TheSims {
        console: Console0,
        datasets_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsRle {
        console: Console0,
        rletextures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsBustinOut {
        console: Console0,
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheSimsBustinOutRle {
        console: Console0,
        rletextures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    TheUrbz {
        console: Console0,
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-2")]
    TheSims2 {
        console: Console0,
        textures_path: std::path::PathBuf,
        output_path: std::path::PathBuf,
    },
    #[clap(name = "the-sims-2-pets")]
    TheSims2Pets {
        console: Console1,
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
            console,
            datasets_path,
            output_path,
        } => match console {
            Console0::PlayStation2 => the_sims::extract_playstation_2_textures(datasets_path, output_path),
            Console0::GameCube => the_sims::extract_gamecube_textures(datasets_path, output_path),
            Console0::Xbox => the_sims::extract_xbox_textures(datasets_path, output_path),
        },
        CliCommands::TheSimsRle {
            console,
            rletextures_path,
            output_path,
        } => match console {
            Console0::PlayStation2 => the_sims::extract_rle_textures(rletextures_path, output_path, Endianness::Little),
            Console0::GameCube => the_sims::extract_rle_textures(rletextures_path, output_path, Endianness::Big),
            Console0::Xbox => the_sims::extract_rle_textures(rletextures_path, output_path, Endianness::Little),
        },
        CliCommands::TheSimsBustinOut {
            console,
            textures_path,
            output_path,
        } => match console {
            Console0::PlayStation2 => the_sims_bustin_out::extract_playstation_2_textures(textures_path, output_path),
            Console0::GameCube => the_sims_bustin_out::extract_gamecube_textures(textures_path, output_path),
            Console0::Xbox => the_sims_bustin_out::extract_xbox_textures(textures_path, output_path),
        },
        CliCommands::TheSimsBustinOutRle {
            console,
            rletextures_path,
            output_path,
        } => match console {
            Console0::PlayStation2 | Console0::Xbox => {
                the_sims_bustin_out::extract_rle_textures(rletextures_path, output_path, Endianness::Little)
            }
            Console0::GameCube => {
                the_sims_bustin_out::extract_rle_textures(rletextures_path, output_path, Endianness::Big)
            }
        },
        CliCommands::TheUrbz {
            console,
            textures_path,
            output_path,
        } => match console {
            Console0::PlayStation2 => the_urbz::extract_playstation_2_textures(textures_path, output_path),
            Console0::GameCube => the_urbz::extract_gamecube_textures(textures_path, output_path),
            Console0::Xbox => the_urbz::extract_xbox_textures(textures_path, output_path),
        },
        CliCommands::TheSims2 {
            console,
            textures_path,
            output_path,
        } => match console {
            Console0::PlayStation2 => {
                the_sims_2::extract_playstation_2_textures(textures_path, output_path, &the_sims_2::SPECULAR_FILE_NAMES)
            }
            Console0::GameCube => {
                the_sims_2::extract_gamecube_textures(textures_path, output_path, &the_sims_2::SPECULAR_FILE_NAMES)
            }
            Console0::Xbox => the_sims_2::extract_xbox_textures(textures_path, output_path),
        },
        CliCommands::TheSims2Pets {
            console,
            textures_path,
            output_path,
        } => match console {
            Console1::PlayStation2 => the_sims_2::extract_playstation_2_textures(
                textures_path,
                output_path,
                &the_sims_2_pets::SPECULAR_FILE_NAMES,
            ),
            Console1::GameCube | Console1::Wii => {
                the_sims_2::extract_gamecube_textures(textures_path, output_path, &the_sims_2_pets::SPECULAR_FILE_NAMES)
            }
        },
        CliCommands::TheSims2Castaway {
            textures_path,
            output_path,
        } => {
            the_sims_2::extract_gamecube_textures(
                textures_path,
                output_path,
                &the_sims_2_castaway::SPECULAR_FILE_NAMES,
            );
        }
        CliCommands::TheSims3 {
            textures_path,
            output_path,
        } => {
            the_sims_3::extract_textures(textures_path, output_path);
        }
    }
}
