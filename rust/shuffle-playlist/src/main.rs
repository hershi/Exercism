use std::iter;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut input = ['a', 'b', 'c', 'd','e'];
    let mut permutations = Vec::new();
    generate_permutations(&mut input, 0, &mut permutations);

    let mut counters = Vec::new();
    let mut prefix = Vec::with_capacity(input.len());
    for i in iter::repeat(0).take(input.len()) { prefix.push(i); }
    generate_counter(input.len(), prefix.as_mut_slice(), 0, &mut counters);

    println!("Hey {}", permutations.len());
    for p in permutations {
        println!("{}", p);
    }

    println!("Hey {}", counters.len());
    for c in counters.clone() {
        println!("{:?}", c);
    }

    let mut res = HashMap::new();
    for c in counters {
        let x = get_permutation(&input, c);
        println!("{}", &x);
        *(res.entry(x).or_insert(0)) += 1;
    }

    println!("{:?}", res);
}

fn generate_permutations(input : &mut [char], start : usize, output : &mut std::vec::Vec<String>) {
    if start == input.len() {
        output.push(input.to_vec().into_iter().collect::<String>());
        return;
    }

    for i in start..input.len() {
        input.swap(start, i);
        generate_permutations(input, start + 1, output);
        input.swap(i,start);
    }
}

fn generate_counter(num_digits : usize, prefix : &mut [usize], prefix_len : usize, output : &mut Vec<Vec<usize>>) {
    if prefix_len == prefix.len() {
        output.push(prefix.to_vec());
        return;
    }

    for i in 0..num_digits {
        prefix[prefix_len] = i;
        generate_counter(num_digits, prefix, prefix_len + 1, output);
    }
}

fn get_permutation(input : &[char], c : Vec<usize>) -> String {
    let mut permutation = input.to_vec();
    for i in 0..input.len() {
        permutation.swap(i, c[i]);
    }

    permutation.into_iter().collect::<String>()
}
