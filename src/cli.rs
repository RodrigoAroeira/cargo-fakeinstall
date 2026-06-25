use clap::Parser;

use crate::uri::Uri;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Cli {
    Fakeinstall(FakeinstallArgs),
}

#[derive(Parser)]
pub struct FakeinstallArgs {
    #[arg(short, long)]
    pub uri: Uri,

    #[arg(short, long)]
    pub bin_name: String,
}
