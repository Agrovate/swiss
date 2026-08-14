mod converters;

use clap::{Parser, Subcommand};
use std::{path::PathBuf};
use std::process::Command;
use anyhow::{anyhow, Result};
use converters::{get_converter};


#[derive(Parser, Debug)]
#[command(name = "swiss", about = "Universal file converter", version)]
struct Cli {

    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Convert {input: PathBuf, output: PathBuf},
    List,
}



fn main() -> Result<()> {
    let cli = Cli::parse();


    match cli.command {
        Commands::Convert {input, output} => {
            let converter = get_converter(&input).ok_or(anyhow!("invalid file extentions"))?;
            let args = converter.build_args(&input, &output);

            if cli.verbose {
                println!("Running: {}",args.join(" "));
            }

            run_command(&args)?;
            println!("{} file is created",output.display());
        },

        Commands::List => {
            let groups = [
                ("Images (ImageMagick)",   vec!["jpg", "jpeg", "png", "gif", "webp", "bmp"]),
                ("Video/Audio (FFmpeg)",   vec!["mp4", "mkv", "avi", "mov", "mp3", "wav", "flac"]),
                ("Documents (Pandoc)",     vec!["md", "rst", "html", "tex", "epub"]),
                ("Office (LibreOffice)",   vec!["docx", "xlsx", "pptx", "odt", "pdf"]),
            ];
            println!("Tools available: ");
            for (label, exts) in &groups {
                println!("{}: {}", label, exts.join(", "));
            }
        },
    }

    Ok(())
}

fn run_command(args: &[String]) -> Result<()>{
    let status = Command::new(&args[0]).args(&args[1..]).status()?;

    if !status.success() {
        anyhow::bail!("command has failed: {}",status);
    }
    Ok(())
}
