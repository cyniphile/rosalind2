use tools::DNA;

pub fn is_substring(main: DNA, motif: DNA) -> Vec<usize> {
    (0..(main.len() - motif.len()) + 1)
        .filter(|i| main[i.to_owned()..(i + motif.len())] == motif)
        .map(|i| i + 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use tools::{read_substring_input, StringParsable};

    use super::*;

    #[test]
    fn test_is_substring() {
        let v = is_substring(DNA::parse_string("AGCAG"), DNA::parse_string("AG"));
        assert_eq!(v, vec![1, 4])
    }

    #[test]
    fn test_subs() {
        let (main, motif) = read_substring_input("rosalind_subs.txt");
        for item in is_substring(main, motif) {
            print!("{item} ")
        }
    }
}
