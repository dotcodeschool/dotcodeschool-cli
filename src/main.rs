use clap::{Args, Parser, Subcommand};
use db::PATH_DB;
use monitor::{Monitor, MonitorError, StateMachine};

mod db;
mod lister;
mod monitor;
mod parsing;
mod runner;
mod str_res;
mod validator;

const PATH_COURSE: &str = "./course.json";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long)]
    course: Option<String>,
    #[arg(long)]
    db: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "test")]
    Test(TestArgs),
    #[command(name = "check")]
    Check,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct TestArgs {
    #[arg(group = "exclusive")]
    name: Option<String>,
    #[command(flatten)]
    options: TestOptions,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct TestOptions {
    #[arg(long, group = "exclusive")]
    list: bool,
    #[arg(long, group = "exclusive")]
    staggered: bool,
}

fn main() -> Result<(), MonitorError> {
    let args = Cli::parse();

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(str_res::LOG)?;

    let _ = simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::ConfigBuilder::default()
            .add_filter_allow_str("dotcodeschool_cli")
            .build(),
        file,
    );

    let path_course = match args.course {
        Some(path) => path,
        None => PATH_COURSE.to_string(),
    };

    let path_db = match args.db {
        Some(path) => path,
        None => PATH_DB.to_string(),
    };

    let monitor = Monitor::new(&path_db, &path_course)?;

    match args.command {
        Command::Test(TestArgs { name, options }) => {
            if options.list {
                let mut lister = monitor.into_lister()?;

                while !lister.is_finished() {
                    lister = lister.run();
                }
            } else if options.staggered {
                let mut runner = monitor.into_runner_staggered()?;

                while !runner.is_finished() {
                    runner = runner.run();
                }
            } else {
                let mut runner = monitor.into_runner(name)?;

                while !runner.is_finished() {
                    runner = runner.run();
                }
            }
        }
        Command::Check => {
            let mut validator = monitor.into_validator();

            while !validator.is_finished() {
                validator = validator.run();
            }
        }
    }

    Ok(())
}
