use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ansi_term::Colour::{Red, White};

pub fn extract_genes(contigs: &[PathBuf], refs: &[PathBuf], output: &Path) {
    fs::create_dir_all(output).expect("Failed creating parent directory for file output");
    refs.iter().for_each(|r| {
        contigs.iter().for_each(|con| {
            let refs_name = r
                .file_stem()
                .expect("Failed getting file stem")
                .to_str()
                .expect("Failed parsing file stem");
            let save_path = output.join(refs_name);
            fs::create_dir_all(&save_path).expect("Failed creating output directory");
            let phyluce = run_phyluce(con, r, &save_path);
            check_process_success(&phyluce, con, r);
        });
    });
}

fn run_phyluce(contigs: &Path, refs: &Path, output: &Path) -> Output {
    Command::new("phyluce_assembly_match_contigs_to_barcodes")
        .arg("--contigs")
        .arg(contigs)
        .arg("--barcode")
        .arg(refs)
        .arg("--output")
        .arg(output)
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
