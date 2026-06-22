use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Cli {
    Fakeinstall(FakeinstallArgs),
}

#[derive(Parser)]
pub struct FakeinstallArgs {
    #[arg(long = "url")]
    pub url: String,

    #[arg(long = "bin-name")]
    pub bin_name: String,
}
