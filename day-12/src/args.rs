use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub input_file: String,

    #[arg(short, long)]
    pub start_from_any_plane: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
