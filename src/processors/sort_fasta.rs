use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::generics::{Scaffold, log_scaffolds, read_fasta, read_fasta_to_map};

// SPLIT INPUT ASSEMBLY DESCRIPTION VEC INTO 1 FOR SEX CHROMOSOMES AND 1 FOR OTHER
pub fn split_by_sex_scaff(scaffolds: Vec<Scaffold>) -> (Vec<Scaffold>, Vec<Scaffold>) {
    scaffolds.into_iter().partition(|scaff| scaff.is_sex)
}

// ENCAPSULATE THE SORTING LOGIC FOR BOTH SEX BASED PATHWAYS
// DIRECTION IS PASSED IN TO CHANGE THE ASCENDING/DESCENDING LOGIC
// OUTPUTS SORTED VECS
pub fn sex_based_sorting(
    scaffolds: Vec<Scaffold>,
    direction: &String,
) -> (Vec<Scaffold>, Vec<Scaffold>) {
    let (mut sex_scaffs, mut non_sex_scaffs) = split_by_sex_scaff(scaffolds);

    if direction == "ASCENDING" {
        sex_scaffs.sort_by_key(|scaff| scaff.scaff_length);
        non_sex_scaffs.sort_by_key(|scaff| scaff.scaff_length);
    } else if direction == "DESCENDING" {
        sex_scaffs.sort_by_key(|scaff| std::cmp::Reverse(scaff.scaff_length));
        non_sex_scaffs.sort_by_key(|scaff| std::cmp::Reverse(scaff.scaff_length));
    }

    (sex_scaffs, non_sex_scaffs)
}

// WRITE NEW FASTA BASED ON ORDER OF THE SCAFFOLD
fn write_ordered_fasta<P: AsRef<Path>>(
    scaffolds: &[Scaffold],
    fasta_map: &HashMap<String, Vec<u8>>,
    out_path: P,
    line_length: usize,
) -> std::io::Result<()> {
    let file = File::create(out_path)?;
    let mut writer = BufWriter::new(file);

    for scaff in scaffolds {
        if let Some(seq) = fasta_map.get(&scaff.id) {
            writeln!(writer, ">{}", scaff.id)?;
            let fasta_seqs = String::from_utf8_lossy(seq);

            for chunk in fasta_seqs.as_bytes().chunks(line_length) {
                writeln!(writer, "{}", String::from_utf8_lossy(chunk))?;
            }
        } else {
            eprintln!("Warning: Scaffold {} not found in FASTA", scaff.id);
        }
    }
    Ok(())
}

// MAIN FUNCTION FOR COMMAND
pub fn sort_fasta_main(
    fasta: &String,
    line_length: &usize,
    sort_option: &str,
    direction: &String,
    prefix: &String,
    output: &String,
) {
    // GET SCAFFOLD DATA
    let mut scaffolds = read_fasta(fasta);

    let mut resulting_order = match sort_option {
        "SLT" => {
            println!("SORT BY: SEX_LENGTH_TOP");

            let (mut sex, mut not_sex) = sex_based_sorting(scaffolds, direction);

            // EXTEND THE SEX VEC WITH NOT_SEX (WHICH WILL BE EMPTIED)
            // SEX WILL BE ONTOP
            sex.append(&mut not_sex);

            sex
        }
        "SLB" => {
            println!("SORT BY: SEX_LENGTH_BOTTOM");

            let (mut sex, mut not_sex) = sex_based_sorting(scaffolds, direction);

            // EXTEND THE NOT_SEX VEC WITH SEX (WHICH WILL BE EMPTIED)
            // SEX WILL BE AT THE BOTTOM
            not_sex.append(&mut sex);

            not_sex
        }
        "SL" => {
            println!("SORT BY: SCAFFOLD_LENGTH");
            scaffolds.sort_by_key(|scaff| scaff.scaff_length);

            scaffolds
        }
        _ => unreachable!(),
    };

    // UPDATE THE STRUCT SO THAT THE NEW POSITION IS RECORDED
    for (i, scaff) in resulting_order.iter_mut().enumerate() {
        scaff.sorted_position = i;
    }

    // LOG THE SCAFFOLD RECORDS INTO A LOG FILE
    let log_output = output.to_owned() + "/scaffold_changes.log";
    let _ = log_scaffolds(&resulting_order, &log_output);

    // SEARCH THROUGH FASTA AND OUTPUT TO NEW FASTA
    let fasta_map = read_fasta_to_map(fasta);
    write_ordered_fasta(
        &resulting_order,
        &fasta_map,
        prefix.to_owned() + ".fasta",
        *line_length,
    )
    .expect("Failed to write FASTA");
}
