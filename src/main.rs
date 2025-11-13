use fasta_fixa::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } else {
        println!("Done")
    }
}
