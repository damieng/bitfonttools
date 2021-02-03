use structopt::StructOpt;
use std::process::exit;

fn main() {
    println!("make-bdf-proportional v0.0.1");

    let args = Cli::from_args();

    let mut file = bdf::open(&args.in_filename);

    match file.as_mut()
    {
        Ok(font) => {
            let glyphs = font.glyphs_mut();
            let mut space_width = 0;
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

                if gl.0 == &'M' {
                    space_width = width;
                }

                glyph.set_device_width(Some((width, 0)));
            }

            let space = font.glyphs_mut().get_mut(&' ');
            match space {
                Some(space_glyph) => {
                    space_glyph.set_device_width(Some((space_width, 0)));
                }
                None => {}
            }

            match bdf::save(&args.out_filename, &font) {
                Ok(_) => {
                    println!("Proportional version created.");
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    in_filename: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    out_filename: std::path::PathBuf,
}
