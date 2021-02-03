use structopt::StructOpt;
use std::process::exit;
use std::collections::hash_map::Entry;

fn main() {
    let args = Cli::from_args();

    match bdf::open(&args.inFileName).as_mut()
    {
        Ok(font) => {
            let glyphs = font.glyphs_mut();
            for gl in glyphs {
                let glyph = gl.1;
                let mut width = glyph.width();

                for x in 0..glyph.width() {
                    for y in 0..glyph.height() {
                        if glyph.get(glyph.width() - x - 1, y) {
                            width = x;
                            break;
                        }
                    }
                }

                println!("{}", width);
                glyph.set_device_width(Some((width, 0)));
            }

            bdf::save(&args.outFileName, &font);
        }
        Err(e) => {
            eprintln!("{}", &*e);
            exit(1);
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    inFileName: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    outFileName: std::path::PathBuf,
}
