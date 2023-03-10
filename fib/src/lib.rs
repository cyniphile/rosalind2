use tools::read_input;

pub fn next_generation(
    last_generation_size: u128,
    next_last_generation_size: u128,
    litter_size: u128,
) -> u128 {
    last_generation_size + next_last_generation_size * litter_size
}

pub fn rabbits(generations: u128, litter_size: u128) -> u128 {
    let mut next_last_generation_size = 1;
    let mut last_generation_size = 1;
    for _ in 2..generations {
        let tmp = next_generation(last_generation_size, next_last_generation_size, litter_size);
        next_last_generation_size = last_generation_size;
        last_generation_size = tmp;
    }
    last_generation_size
}

pub fn test_rabbits(filepath: &str) {
    let (generations, litter_size) = read_input(filepath);
    println!("{}", rabbits(generations, litter_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_rabbits("rosalind_fib.txt");
        assert_eq!(rabbits(5, 3), 19);
    }
}
