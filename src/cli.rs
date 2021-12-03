use std::io::Result;
use std::path::{Path, PathBuf};

use clap::{crate_description, crate_name, App, AppSettings, Arg, ArgMatches};
use glob::glob;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::extract;
use crate::utils;

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
                        .help("Path to contig directory")
                        .takes_value(true)
                        .required(true)
                        .value_name("PATH"),
                )
                .arg(
                    Arg::with_name("refs")
                        .short("r")
                        .long("refs")
                        .help("Path to reference fasta files")
                        .takes_value(true)
                        .required(true)
                        .value_name("PATH"),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("Path to output files")
                        .takes_value(true)
                        .required(true)
                        .value_name("PATH"),
                ),
        )
        .get_matches()
}

pub fn parse_cli(version: &str) {
    let args = get_args(version);
    setup_logger().expect("Failed setting up a log file.");
    match args.subcommand() {
        ("extract", Some(extract_matches)) => parse_extract_cli(extract_matches, version),
        ("check", Some(_)) => display_app_info(version),
        _ => unreachable!(),
    }
}

fn parse_extract_cli(matches: &ArgMatches, version: &str) {
    let contigs = Path::new(
        matches
            .value_of("dir")
            .expect("Failed get contig directory path."),
    );
    let ref_path = matches
        .value_of("refs")
        .expect("Failed to get reference directory path.");
    let output = Path::new(
        matches
            .value_of("output")
            .expect("Failed get output directory path."),
    );
    let refs = get_reference_path(ref_path);
    display_app_info(version);
    extract::extract_genes(&contigs, &refs, output);
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

fn get_reference_path(path: &str) -> Vec<PathBuf> {
    find_files(Path::new(path))
}

fn find_files(path: &Path) -> Vec<PathBuf> {
    glob(&format!("{}/*.fa*", path.display()))
        .expect("Failed globbing files")
        .filter_map(|ok| ok.ok())
        .collect::<Vec<PathBuf>>()
}

fn display_app_info(version: &str) {
    log::info!("{} v{}", crate_name!(), version);
    log::info!("{}", crate_description!());
    log::info!("Developed by Heru Handika\n");
    utils::system_info();
}

// fn print_complete() {
//     log::info!("COMPLETED!");
//     log::info!("Please, check each program log for commands and other details!\n")
// }

// fn log_input(contigs: &Path, ref_path: &str) {
//     log::info!("{:18}: {}", "Contig Dir", contigs.display());
//     log::info!("{:18}: {}", "Reference Dir", ref_path);
//     // match params {
//     //     Some(param) => log::info!("{:18}: {}", "Opt params", param),
//     //     None => log::info!("{:18}: None", "Params"),
//     // }
// }

fn setup_logger() -> Result<()> {
    let log_dir = std::env::current_dir()?;
    let target = log_dir.join("gene-extractor.log");
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
