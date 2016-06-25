pub fn primes_up_to(num : u32) -> Vec<u32> {
    let mut input = (2..num+1).collect::<Vec<u32>>();
    let mut current_index = 0;

    while current_index < input.len() {
        let current = input[current_index]; 
        input.retain(|&x| x == current || x % current != 0);
        current_index += 1;
    }

    input
}