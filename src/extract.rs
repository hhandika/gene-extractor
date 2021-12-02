use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ansi_term::Colour::{Red, White};

use crate::utils;

pub fn extract_genes(contigs: &Path, refs: &[PathBuf], output: &Path) {
    refs.iter().for_each(|r| {
        let refs_name = r
            .file_stem()
            .expect("Failed getting file stem")
            .to_str()
            .expect("Failed parsing file stem");
        let text = format!("Ref. sequence: {}", refs_name);
        utils::print_divider(&text, 60);
        let save_path = output.join(refs_name);
        log::info!("{:18}: {}", "Contig dir", contigs.display());
        log::info!("{:18}: {}", "Ref. sequence", r.display());
        log::info!("{:18}: {}\n", "Output dir", save_path.display());
        let spin = utils::set_spinner();
        spin.set_message("Processing...");
        let phyluce = run_phyluce(contigs, r, &save_path);
        check_process_success(&phyluce, contigs, r);
        spin.finish_with_message(format!("Finished processing reference {}", r.display()));
        println!();
    });

    log::info!("COMPLETED!")
}

fn run_phyluce(contigs: &Path, refs: &Path, output: &Path) -> Output {
    Command::new("phyluce_assembly_match_contigs_to_barcodes")
        .arg("--contigs")
        .arg(contigs)
        .arg("--barcode")
        .arg(refs)
        .arg("--output")
        .arg(output)
        .arg("--no-bold")
        .output()
        .expect("Failed to execute process")
}

fn check_process_success(out: &Output, contig: &Path, refs: &Path) {
    if !out.status.success() {
        log::error!(
            "{}: Phyluce is failed to process contig {} for references {} (See below).",
            White.on(Red).paint("ERROR"),
            contig.display(),
            refs.display()
        );
        log::error!("{}", std::str::from_utf8(&out.stdout).unwrap());
        log::error!("{}", std::str::from_utf8(&out.stderr).unwrap());
    }
}
