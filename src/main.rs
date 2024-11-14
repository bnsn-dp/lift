use clap::{Args, Parser, Subcommand};
use std::{io::Write, path::PathBuf};
use chrono::Local;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "lift")]
#[command(about = "A plain text workout log that saves to human-readable .md", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Prints the file
    #[command(arg_required_else_help = true)]
    Scan {
        pattern: String,
        path: PathBuf,
    },
    /// tracks a straight set
    Set(SetArgs),
    /// tracks a one-rep max
    Max(MaxArgs),
    /// tracks a myrorep match set
    Myo(MyoArgs),
    /// tracks a down set
    Down(DownArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct SetArgs {
    exercise: String,
    sets: u8,
    reps: u8,
    weight: u8,
    rir: u8,
    path: PathBuf,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct MaxArgs {
    exercise: String,
    weight:u8,
    path: PathBuf
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct MyoArgs {
    exercise: String,
    sets: u8,
    reps: u8,
    rests: u8,
    weight: u8,
    path: PathBuf
}


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct DownArgs {
    exercise: String,
    starting_reps: u16,
    weight: u8,
    path: PathBuf
}

fn valid_date(path: &PathBuf){
    let mut today = false;
    let date = Local::now().format("%Y-%m-%d").to_string();
    let contents = std::fs::read_to_string(path).expect("Could not read file");
    for line in contents.lines() {
        if line.contains(&date) {
            today = true;
        }
    }
    if !today {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Could not open file");
        writeln!(file, "{}", date).expect("Write failed");
    }
}


fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Scan { pattern, path } => {
            let contents = std::fs::read_to_string(&path).expect("Could not read file");
            for line in contents.lines() {
                if line.contains(&pattern) {
                    println!("{}", line);
                }
            }
        },
        Commands::Set(set_args) => {
            println!("logging '#set {}: {}x{} [{}lbs] ({} RIR)'", set_args.exercise, set_args.sets, set_args.reps, set_args.weight, set_args.rir);
            valid_date(&set_args.path);
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&set_args.path)
                .expect("could not open file");
            writeln!(file, "    #set {}: {}x{} [{}lbs] ({} RIR)", set_args.exercise, set_args.sets, set_args.reps, set_args.weight, set_args.rir).expect("write failed");
        },
        Commands::Max(max_args) => {
            println!("logging '#max {}: [{}lbs]'", max_args.exercise, max_args.weight);
            valid_date(&max_args.path);
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&max_args.path)
                .expect("could not open file");
            writeln!(file, "    #max {}: [{}lbs]", max_args.exercise, max_args.weight).expect("write failed");
        },
        Commands::Myo(myo_args) => {
            println!("logging '#myo {}: {}x{} ({} rests) [{} lbs]'", myo_args.exercise, myo_args.sets, myo_args.reps, myo_args.rests, myo_args.weight);
            valid_date(&myo_args.path);
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&myo_args.path)
                .expect("could not open file");
            writeln!(file, "    #myo {}: {}x{} ({} rests) [{}lbs]", myo_args.exercise, myo_args.sets, myo_args.reps, myo_args.rests, myo_args.weight).expect("write failed");
        },
        Commands::Down(down_args) => {
            println!("logging '#down {}: {} => {} [{}lbs]'", down_args.exercise, (down_args.starting_reps * (down_args.starting_reps+1))/2, down_args.starting_reps, down_args.weight);
            valid_date(&down_args.path);
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&down_args.path)
                .expect("could not open file");
            writeln!(file, "    #down {}: {} => {} [{}lbs]", down_args.exercise, (down_args.starting_reps * (down_args.starting_reps+1))/2, down_args.starting_reps, down_args.weight).expect("write failed");
        },
    }
}