use clap::*;

#[derive(clap)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "hikaru-yamaguchi",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
}
