use lehmer::Lehmer;
use chrono::Local;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

// const N: usize = 6;
// const FACT: [usize; 7] = [1, 1, 2, 6, 24, 120, 720];
// const ROWS: usize = 1 + 2 + 6 + 24 + 120;
//
// const DIRECTIONS: usize = 2;
// const FORWARD: usize = 0;
// const BACKWARD: usize = 1;
//
// const TABLE: [[[[usize; ROWS]; N - 1]; FACT[N] / 2]; DIRECTIONS]
// const FLIPS: [[[usize; N - 1]; FACT[N] / 2]; DIRECTIONS]
// const MAPPINGS: [usize; FACT[N] / 2]

const BENCHMARK: bool = false;

fn main() {
    let mut seen = [false; FACT[N] / 2];

    let mut best_perms = [1024; FACT[N] / 2];
    let mut max_perms = 0;
    let mut string = [(0, FORWARD); FACT[N] * 2];

    for max_waste in 0.. {
        search(&mut seen, 0, FORWARD, 0, max_waste, 0, &mut max_perms, &mut best_perms, &mut string, 0);
        best_perms[max_waste] = max_perms;

        print_timestamp();
        println!("{} wasted characters, at most {} permutations", max_waste, max_perms);
        println!();

        if BENCHMARK && max_waste == 52 {
            break;
        }

        if max_perms == FACT[N] / 2 {
            println!("shortest superpermutation: {}", ((N - 1) + FACT[N] / 2 + max_waste) * 2 - 1);
            break;
        }
    }
}

fn search(seen: &mut [bool; FACT[N] / 2], index: usize, direction: usize, cur_waste: usize, max_waste: usize, cur_perms: usize, max_perms: &mut usize, best_perms: &mut [usize; FACT[N] / 2], string: &mut [(usize, usize); FACT[N] * 2], cursor: usize) {
    seen[index] = true;
    string[cursor] = (index, direction);

    if *max_perms == cur_perms {
        *max_perms += 1;

        print_timestamp();
        print_string(string, cursor);
    }

    let next_perms = cur_perms + 1;
    let next_cursor = cursor + 1;

    let table = TABLE[direction][index];
    let flips = FLIPS[direction][index];

    for waste in 0..N - 1 {
        let next_waste = cur_waste + waste;

        if next_waste > max_waste {
            continue;
        }

        let possible_perms = next_perms + best_perms[max_waste - next_waste];

        if possible_perms <= *max_perms {
            continue;
        }

        let flip = flips[waste];
        let rows = table[waste];

        for i in 0..flip {
            let next_index = rows[i];

            if seen[next_index] {
                continue;
            }

            search(seen, next_index, FORWARD, next_waste, max_waste, next_perms, max_perms, best_perms, string, next_cursor);
        }

        for i in flip..ROWS {
            let next_index = rows[i];

            if seen[next_index] {
                continue;
            }

            search(seen, next_index, BACKWARD, next_waste, max_waste, next_perms, max_perms, best_perms, string, next_cursor);
        }
    }

    seen[index] = false;
}

fn print_string(string: &[(usize, usize); FACT[N] * 2], cursor: usize) {
    let mut last = 0;
    print!("1");

    for c in 0..cursor {
        let (index, direction) = string[c];
        let id = MAPPINGS[index];

        let mut perm = Lehmer::from_decimal(id, N).to_permutation();

        if direction == BACKWARD {
            perm.reverse();
        }

        for p in perm.iter().skip_while(|p| **p != last).skip(1) {
            print!("{}", p + 1);
        }

        last = *perm.last().unwrap();
    }

    println!();
    println!();
}

fn print_timestamp() {
    println!("{}", Local::now().format("%Y-%m-%d:%H:%M:%S"));
}
