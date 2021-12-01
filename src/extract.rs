use std::path::PathBuf;

pub fn extract_genes(contigs: &[PathBuf], refs: &[PathBuf]) {
    contigs.iter().for_each(|contig| {
        refs.iter().for_each(|r| {
            println!("{}", contig.display());
            println!("{}", r.display());
        });
    });
}
