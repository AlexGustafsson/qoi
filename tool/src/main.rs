use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::{AppSettings, Parser, Subcommand};

use show_image::{create_window, event, Color, ImageInfo, ImageView, WindowOptions};

use qoi;

#[derive(Parser)]
#[clap(name = "qoi")]
#[clap(about = "A Quite OK Image Format viewer")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View an image
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    View {
        /// The file to view
        file: String,
    },
    /// Display information about the image
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Info {
        /// The files to inspect
        files: Vec<String>,
    },
}

#[show_image::main]
fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Info { files } => {
            for (i, file) in files.iter().enumerate() {
                if i > 0 {
                    println!("");
                }

                let file_path = Path::new(file);
                let file = File::open(file).unwrap_or_else(|error| {
                    eprintln!("failed to open file: {}", error);
                    std::process::exit(1)
                });
                let mut reader = BufReader::new(file);
                let header = qoi::Header::from_reader(&mut reader).unwrap_or_else(|error| {
                    eprintln!("failed to parse file: {}", error);
                    std::process::exit(1)
                });
                println!("{}", file_path.file_name().unwrap().to_str().unwrap());
                println!("width: {}", header.width);
                println!("height: {}", header.height);
                println!("channels: {} ({})", header.channels, header.channels_name());
                println!(
                    "colorspace: {} ({})",
                    header.colorspace,
                    header.colorspace_name()
                );
            }
        }
        Commands::View { file } => {
            let file_path = Path::new(file);
            let file = File::open(file).unwrap_or_else(|error| {
                eprintln!("failed to open file: {}", error);
                std::process::exit(1)
            });
            let mut reader = BufReader::new(file);
            let image = qoi::Image::from_reader(&mut reader).unwrap_or_else(|error| {
                eprintln!("failed to parse file: {}", error);
                std::process::exit(1)
            });

            let view = ImageView::new(
                ImageInfo::rgba8(image.header.width as u32, image.header.height as u32),
                &image.buffer[..],
            );

            let name = file_path
                .file_stem()
                .and_then(|x| x.to_str())
                .unwrap_or("image");

            let window_options = WindowOptions {
                preserve_aspect_ratio: true,
                background_color: Color::rgb(0.1568627451, 0.1568627451, 0.1568627451),
                start_hidden: false,
                size: Some([image.header.width as u32, image.header.height as u32]),
                resizable: true,
                borderless: false,
                overlays_visible: true,
                default_controls: true,
            };
            let window = create_window(name, window_options).unwrap();
            window.set_image(name, view).unwrap();

            for event in window.event_channel().map_err(|e| e.to_string()).unwrap() {
                if let event::WindowEvent::KeyboardInput(event) = event {
                    let escape_pressed = !event.is_synthetic
                        && event.input.key_code == Some(event::VirtualKeyCode::Escape)
                        && event.input.state.is_pressed();

                    let cmd_w_pressed = !event.is_synthetic
                        && event.input.key_code == Some(event::VirtualKeyCode::W)
                        && event.input.modifiers == event::ModifiersState::LOGO
                        && event.input.state.is_pressed();

                    if escape_pressed || cmd_w_pressed {
                        break;
                    }
                }
            }
        }
    }
}
