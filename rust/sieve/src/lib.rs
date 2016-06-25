pub fn primes_up_to(num : u32) -> Vec<u32> {
    let mut input = (2..num+1).collect::<Vec<u32>>();
    let mut result = Vec::new();

    while !input.is_empty() {
        let current = input[0];
        result.push(current);
        input.retain(|&x| x % current != 0);
    }

    result
}