use core::fmt;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, fs};

pub fn read_input(filepath: &str) -> (u128, u128) {
    let data = fs::read_to_string(filepath).expect("Should have been able to read the file");
    data.split_whitespace()
        .take(2)
        .map(|x| x.parse::<u128>().ok().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn read_substring_input(filepath: &str) -> (DNA, DNA) {
    fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .trim()
        .split('\n')
        .map(DNA::parse_string)
        .collect_tuple()
        .unwrap()
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

impl fmt::Display for DnaNucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", DNA::to_char(self))
    }
}

pub type DNA = Vec<DnaNucleotide>;
pub type DNASlice = [DnaNucleotide];

pub trait Nucleotide {
    fn complement(&self) -> Self;
}

// TODO: is there exhaustivity checking for Bijective maps?
impl Nucleotide for DnaNucleotide {
    fn complement(&self) -> DnaNucleotide {
        match self {
            DnaNucleotide::A => DnaNucleotide::T,
            DnaNucleotide::T => DnaNucleotide::A,
            DnaNucleotide::C => DnaNucleotide::G,
            DnaNucleotide::G => DnaNucleotide::C,
        }
    }
}

pub trait StringParsable {
    type Item;
    fn parse_string(seq: &str) -> Self;
    fn to_string(&self) -> String;
    fn to_char(base: &Self::Item) -> char;
}

impl StringParsable for DNA {
    type Item = DnaNucleotide;
    fn parse_string(seq: &str) -> DNA {
        let parser = |base| match base {
            // TODO: make these bijective maps
            'A' => DnaNucleotide::A,
            'C' => DnaNucleotide::C,
            'G' => DnaNucleotide::G,
            'T' => DnaNucleotide::T,
            _ => panic!("\"{}\" is not a recognized DNA base.", base),
        };
        seq.chars().map(parser).collect()
    }
    fn to_char(base: &DnaNucleotide) -> char {
        match base {
            DnaNucleotide::A => 'A',
            DnaNucleotide::C => 'C',
            DnaNucleotide::G => 'G',
            DnaNucleotide::T => 'T',
        }
    }
    fn to_string(&self) -> String {
        self.iter().map(Self::to_char).collect()
    }
}

pub fn read_and_parse_string_file<T: StringParsable>(path: &str) -> T {
    let file = fs::read_to_string(path).expect("Can't parse file to into a string.");
    let seq = file.to_uppercase().trim().to_string();
    T::parse_string(&seq)
}

pub fn read_string_file(path: &str) -> String {
    let file = fs::read_to_string(path).expect("fuck");
    file.to_uppercase().trim().to_string()
}

pub type FASTA = HashMap<String, DNA>;

pub fn read_fasta(path: &str) -> FASTA {
    let input = File::open(path).expect("Could not read file.");
    let lines = BufReader::new(input).lines();
    let mut output: FASTA = HashMap::new();
    let mut current_read = String::new();
    let mut current_dna: String = String::new();
    for line in lines {
        let line = line.expect("Couldn't read line");
        if line.starts_with('>') {
            if !current_read.is_empty() {
                output.insert(current_read, DNA::parse_string(&current_dna));
            }
            current_read = line;
            current_dna = String::new();
        } else {
            current_dna.push_str(&line);
        }
    }
    output.insert(current_read, DNA::parse_string(&current_dna));
    output
}

pub fn base_counts<T: Eq + std::hash::Hash>(seq: &[T]) -> HashMap<&T, u32> {
    let mut counts = HashMap::new();
    for item in seq {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    // TODO: organize above code into a module to avoid the many imports here
    use crate::base_counts;
    use crate::read_fasta;
    use crate::read_input;
    use crate::read_substring_input;
    use crate::DnaNucleotide;
    use crate::StringParsable;
    use crate::DNA;

    #[test]
    fn test_read_input() {
        let (one, two) = read_input("rosalind_fib.txt");
        assert_eq!(one, 28);
        assert_eq!(two, 2);
    }

    #[test]
    fn test_read_fasta() {
        let h = read_fasta("rosalind_gc.txt");
        // dbg!(h);
        assert_eq!(
            h.get(">Rosalind_0808").unwrap(), 
            &DNA::parse_string("CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT")
        );
    }

    #[test]
    fn test_base_counts() {
        let seq = DNA::parse_string(
            &"AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".to_string(),
        );
        let answer = base_counts(&seq);
        let answer = [
            answer.get(&DnaNucleotide::A).unwrap().to_owned(),
            answer.get(&DnaNucleotide::C).unwrap().to_owned(),
            answer.get(&DnaNucleotide::G).unwrap().to_owned(),
            answer.get(&DnaNucleotide::T).unwrap().to_owned(),
        ];
        assert_eq!(answer, [20, 12, 17, 21]);
    }

    #[test]
    fn test_read_subs() {
        let (a, b) = read_substring_input("rosalind_subs.txt");
        assert_eq!(a, DNA::parse_string("GATATATGCATATACTT"));
        assert_eq!(b, DNA::parse_string("ATAT"));
    }
}
