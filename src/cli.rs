use std::io::Result;

use crate::utils;
use clap::{crate_description, crate_name, App, AppSettings, Arg, ArgMatches};

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn get_args(version: &str) -> ArgMatches {
    App::new(crate_name!())
        .version(version)
        .about(crate_description!())
        .author("Heru Handika <hhandi1@lsu.edu>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("check").about("Check dependencies"))
        .subcommand(
            App::new("extract")
                .about("Batch gene tree estimation using IQ-Tree")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs folder path to locus alignment")
                        .takes_value(true)
                        .value_name("DIR"),
                ),
        )
        .subcommand(
            App::new("deps")
                .about("Solves dependency issues")
                .subcommand(
                    App::new("astral")
                        .about("Fixes astral dependency issues")
                        .arg(
                            Arg::with_name("jar")
                                .short("-j")
                                .long("jar")
                                .help("Inputs path to the ASTRAL jar file.")
                                .takes_value(true)
                                .required(true)
                                .value_name("ASTRAL-JAR-PATH"),
                        ),
                ),
        )
        .get_matches()
}

pub fn parse_cli(version: &str) {
    let args = get_args(version);
    setup_logger().expect("Failed setting up a log file.");
    match args.subcommand() {
        ("extract", Some(extract_matches)) => parse_extract_cli(extract_matches, &version),
        ("check", Some(_)) => display_app_info(version),
        _ => unreachable!(),
    }
}

fn parse_extract_cli(matches: &ArgMatches, version: &str) {
    let dir = matches.value_of("dir").unwrap();
    println!("DIR: {}", dir);
    display_app_info(version)
}
// fn parse_input_fmt(matches: &ArgMatches) -> InputFmt {
//     let input_fmt = matches
//         .value_of("input-fmt")
//         .expect("CANNOT READ FORMAT INPUT");
//     match input_fmt {
//         "fasta" => InputFmt::Fasta,
//         "nexus" => InputFmt::Nexus,
//         "phylip" => InputFmt::Phylip,
//         _ => unreachable!("Specify input format"),
//     }
// }

fn get_path<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("dir").expect("CANNOT GET DIRECTORY PATH")
}

fn display_app_info(version: &str) {
    log::info!("{} v{}", crate_name!(), version);
    log::info!("{}", crate_description!());
    log::info!("Developed by Heru Handika\n");
    utils::system_info();
}

fn print_complete() {
    log::info!("COMPLETED!");
    log::info!("Please, check each program log for commands and other details!\n")
}

fn log_input(path: &str, params: &Option<String>) {
    log::info!("{:18}: {}", "Input", path);
    match params {
        Some(param) => log::info!("{:18}: {}", "Opt params", param),
        None => log::info!("{:18}: None", "Params"),
    }
}

fn setup_logger() -> Result<()> {
    let log_dir = std::env::current_dir()?;
    let target = log_dir.join("genx.log");
    let tofile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)} - {l} - {m}\n",
        )))
        .build(target)?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(tofile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    Ok(())
}
