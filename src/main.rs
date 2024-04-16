use std::path::PathBuf;
use color_eyre::eyre::{eyre, Result, WrapErr};
use structopt::StructOpt;
use digital_garden::write;
use directories::UserDirs;

/// A CLI for the growing and curation of a digital garden
#[derive(Debug, StructOpt)]
#[structopt(
    name = "garden",
    about = "A CLI tool for creating and managing a digital garden."
)]
struct Opt {
    #[structopt(parse(from_os_str), short = "p", long, env = "GARDEN_PATH")]
    garden_path: Option<PathBuf>,
    
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

fn get_deafult_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find user directories"))?;
    
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();

    let garden_path = match  opt.garden_path {
        Some(path) => Ok(path),
        None => get_deafult_garden_dir().wrap_err("Could not find garden path"),
    }?;
    

    match opt.cmd {
        Command::Write { title } => write(garden_path, title)
    }
}
