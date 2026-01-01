use clap::Args;

#[derive(Args, Debug)]
pub struct EditArgs {
    /// Edit the config file.
    #[arg(short, long)]
    pub edit: bool,
}