use color_eyre::eyre::Result;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden
#[derive(Debug, StructOpt)]
#[structopt(
    name = "garden",
    about = "A CLI tool for creating and managing a digital garden."
)]
struct Opt {
    #[structopt(subcommand)]
    #[allow(dead_code)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// write something in yur garden
    ///
    /// This command will open your $EDITOR, wait for you to write something, and then save it to your garden.
    Write {
        /// Optionally specify a title for what you're writing about
        #[structopt(short, long)]
        #[allow(dead_code)]
        title: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);

    todo!();
}
