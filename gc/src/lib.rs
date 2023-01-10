use tools::{DNA, FASTA};

fn get_gc_percentage(seq: &DNA) -> f64 {
    let gc_count = seq.iter().fold(0, |acc, nucleotide| {
        acc + match nucleotide {
            tools::DnaNucleotide::C => 1,
            tools::DnaNucleotide::G => 1,
            tools::DnaNucleotide::A => 0,
            tools::DnaNucleotide::T => 0,
        }
    });
    gc_count as f64 / seq.len() as f64
}

pub fn show_highest_gc_seq(seqs: FASTA) {
    let mut highest_gc_seq: &str = "";
    let mut highest_gc_percentage: f64 = 0.0;
    for (k, v) in seqs.iter() {
        let gc_percentage = get_gc_percentage(v);
        if gc_percentage > highest_gc_percentage {
            highest_gc_percentage = gc_percentage;
            highest_gc_seq = k;
        }
    }
    println!("{}", &highest_gc_seq[1..]);
    println!("{}", highest_gc_percentage * 100.0);
}

use std::cmp::Ordering::Equal;

// solution from rosalind
pub fn gc() -> String {
    include_str!("../rosalind_gc.txt")
        .trim()
        .split('>')
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut split = x.splitn(2, '\n');
            let name = split.next().unwrap();
            let dna = split.next().unwrap().replace('\n', "");
            (name, dna)
        })
        .map(|x| {
            let gc_count = x.1.matches('G').count() + x.1.matches('C').count();
            let percentage = gc_count as f32 / x.1.len() as f32 * 100.0;
            (x.0, percentage)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal))
        .map(|x| format!("{}\n{}", x.0, x.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::gc;
    use crate::get_gc_percentage;
    use crate::show_highest_gc_seq;
    use tools::StringParsable;
    use tools::{read_fasta, DNA};

    #[test]
    fn test_gc_content() {
        let result = get_gc_percentage(&DNA::parse_string("AGCTATAG"));
        assert_eq!(result, 0.375);
    }

    #[test]
    fn test_show_highest_gc_content() {
        let input = read_fasta("rosalind_gc.txt");
        show_highest_gc_seq(input);
        println!("{}", gc())
    }
}
