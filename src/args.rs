use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap()]
    pub file: String,
}