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
    /// Searches the file at the provided path for a provided pattern
    #[command(arg_required_else_help = true)]
    Scan {
        pattern: String,
        path: PathBuf,
    },
    /// tracks straight sets, where you perform a number of fixed repetitions for a fixed number of sets
    Set(SetArgs),
    /// tracks a one-rep max, where you lift the maximum amount of weight you safely can
    Max(MaxArgs),
    /// tracks myrorep match sets, where you do as many rest-pauses as necessary to reach a target rep number
    Myo(MyoArgs),
    /// tracks a down set, where you perform a number of repetitions and decrement by 1 for every subsequent set
    Down(DownArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct SetArgs {
    /// the name of the exercise you performed
    exercise: String,
    /// the number of sets you performed
    sets: u8,
    /// the amount of repetitions you performed each set
    reps: u8,
    /// the amount of weight you lifted
    weight: u8,
    /// the reps in reserve, the number of additional reps you could have done
    rir: u8,
    /// the path to the file where you want the log recorded
    path: PathBuf,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct MaxArgs {
    /// the name of the exercise you performed
    exercise: String,
    /// the amount of weight you lifted
    weight:u8,
    /// the path to the file where you want the log recorded
    path: PathBuf
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct MyoArgs {
    /// the name of the exercise you performed
    exercise: String,
    /// the number of sets you performed
    sets: u8,
    /// the target reps that you accumulated each set
    reps: u8,
    /// the average number of rest-pauses you took each set
    rests: u8,
    /// the amount of weight you lifted
    weight: u8,
    /// the path to the file where you want the log recorded
    path: PathBuf
}


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct DownArgs {
    /// the name of the exercise you performed
    exercise: String,
    /// the amount of reps you started with and the amount of sets you performed
    starting_reps: u16,
    /// the weight you lifted
    weight: u8,
    /// the path to the file where you want the log recorded
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