use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use needletail::parse_fastx_file;

use std::collections::HashMap;

// READ FASTA INTO HASHMAP
pub fn read_fasta_to_map<P: AsRef<Path>>(path: P) -> HashMap<String, Vec<u8>> {
    let mut reader = parse_fastx_file(path).expect("Invalid FASTA file");
    let mut map = HashMap::new();

    while let Some(record) = reader.next() {
        let rec = record.expect("Error reading FASTA record");
        let id = String::from_utf8_lossy(rec.id()).to_string();
        let seq = rec.seq().to_vec();
        map.insert(id, seq);
    }
    map
}

// LOG SCAFFOLDS DATA
pub fn log_scaffolds(scaffolds: &[Scaffold], path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for scaff in scaffolds {
        writeln!(writer, "{}", scaff)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct Scaffold {
    pub id: String,
    pub id_suffix: String,
    pub is_sex: bool,
    pub scaff_length: usize,
    pub original_position: usize,
    pub sorted_position: usize,
}

impl std::fmt::Display for Scaffold {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "ID: {}\n\t-SUFFIX: {}\n\t-IS_SEX: {}\n\t-SCAFFOLD_LENGTH: {}\n\t-ORIGINAL_POSITION: {}\n\t-SORTED_POSITION: {}\n",
            self.id,
            self.id_suffix,
            self.is_sex,
            self.scaff_length,
            self.original_position,
            self.sorted_position
        )
    }
}

// RETURN BOOL WHETHER ID_SUFFIX CONTAINS A SEX CHROMO ID
fn contains_sex_scaff(chromosome_id: &str) -> bool {
    ["X", "x", "Y", "y", "Z", "z", "W", "w"]
        .iter()
        .any(|scaff| chromosome_id.contains(scaff))
}

// READ FASTA INTO MEMORY AS AN INDEX (SCAFFOLD: ID, LEN)
pub fn read_fasta<P: AsRef<Path>>(path: P) -> Vec<Scaffold> {
    let mut reader = parse_fastx_file(path).expect("Invalid FASTA file");
    let mut scaffolds = Vec::new();
    let mut position = 0;

    while let Some(record) = reader.next() {
        let rec = record.expect("Error reading FASTA record");
        let scaff_id = String::from_utf8_lossy(rec.id()).to_string();

        // IF CONTAINS '_' THEN SPLIT IT AND RETURN THE SUFFIX
        // ELSE RETURN THE FULL NAME
        let scaff_suffix = if scaff_id.contains('_') {
            scaff_id.split('_').last().unwrap()
        } else {
            "NA"
        };

        // POPULATE STRUCT
        scaffolds.push(Scaffold {
            id: scaff_id.clone(),
            id_suffix: scaff_suffix.to_string(),
            is_sex: contains_sex_scaff(scaff_suffix),
            scaff_length: rec.seq().to_vec().len(),
            original_position: position,
            sorted_position: 0,
        });

        position += 1;
    }

    scaffolds
}
