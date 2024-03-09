/**
 * The fibonacci sequence is a series of numbers in which
 * each number is n = (n-1) + (n-2).
 * 
 * It can only be calculated by the natural numbers.
 */
pub fn sequence(rounds: usize) -> Vec<u64> {
    // Each round will be a index on the list, plus the initial 0.
    let maximum_size = rounds + 1;

    let mut sequence = vec![0; maximum_size];
    sequence[1] = 1;
    
    for index in 2..rounds {
        sequence[index] = sequence[index - 1] + sequence[index - 2];
    }

    println!("Fibonacci sequence: {:?} \n", sequence);
    sequence
}
