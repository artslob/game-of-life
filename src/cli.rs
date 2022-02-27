use clap::Parser;

pub fn _parse() -> Cli {
    Cli::parse()
}

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, arg_enum, default_value_t = CellShape::Square)]
    pub cell_shape: CellShape,
}

#[derive(clap::ArgEnum, Debug, Copy, Clone)]
pub enum CellShape {
    Circle,
    Square,
}
