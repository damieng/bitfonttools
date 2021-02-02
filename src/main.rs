use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    match result {
        Err(error) => { println!("{}", error); }
        Ok(content) => { 
            make_bdf_proportional(content);
         }
    }
}

enum Section { StartFont, StartChar }

fn make_bdf_proportional(input: String) {
    let lines = input.lines();
    let mut section = Section::StartFont;
    let mut encoding = 0;

    for line in lines  {
        match section {
            Section::StartFont => {
                if line.starts_with("STARTCHAR") {
                    section = Section::StartChar;
                }
            }
            Section::StartChar => {
                if line.starts_with("ENCODING") {
                    encoding =
                }
                println!("{}", line);
            }
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
