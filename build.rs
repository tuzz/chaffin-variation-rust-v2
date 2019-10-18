use build_const::ConstWriter;
use lehmer::Lehmer;
use std::thread::Builder;

const FACT: [usize; 7] = [1, 1, 2, 6, 24, 120, 720];

const N: usize = 6;
const ROWS: usize = 1 + 2 + 6 + 24 + 120;

//const N: usize = 5;
//const ROWS: usize = 1 + 2 + 6 + 24;

const DIRECTIONS: usize = 2;
const FORWARD: usize = 0;
const BACKWARD: usize = 1;

fn main() {
    Builder::new() // Increase the stack size.
        .stack_size(32 * 1024 * 1024)
        .spawn(run).unwrap()
        .join().unwrap();
}

fn run() {
    let mut table = [FactTable([[RowTable([0; ROWS]); N - 1]; FACT[N] / 2]); DIRECTIONS];
    let mut flips = [FactTable([[0; N - 1]; FACT[N] / 2]); DIRECTIONS];

    let mappings = permutation_id_mappings();

    for &current_direction in &[FORWARD, BACKWARD] {
        for i in 0..FACT[N] / 2 {
            let current_id = mappings[i];
            let mut current = Lehmer::from_decimal(current_id, N).to_permutation();

            if current_direction == BACKWARD {
                current.reverse();
            }

            for waste in 0..N - 1 {
                let head = &current[0..=waste];
                let tail = &current[waste + 1..N];

                let mut next_perms = (0..FACT[head.len()]).map(|j| {
                    let pattern = Lehmer::from_decimal(j, head.len()).to_permutation();
                    let new_tail = pattern.iter().map(|p| head[*p as usize]).collect::<Vec<_>>();

                    let mut next = tail.to_vec(); next.extend(new_tail);
                    lookup_mapping_index(&mut next, &mappings)
                }).collect::<Vec<_>>();

                next_perms.sort_by(|(a_index, a_direction), (b_index, b_direction)| {
                    if a_direction == b_direction {
                        a_index.cmp(b_index)
                    } else {
                        a_direction.cmp(b_direction)
                    }
                });

                for (row, (next_index, _)) in next_perms.iter().enumerate() {
                    table[current_direction].0[i][waste].0[row] = *next_index;
                }

                let flip_index = next_perms.iter()
                    .position(|(_, direction)| *direction == BACKWARD)
                    .unwrap_or(FACT[head.len()]);

                flips[current_direction].0[i][waste] = flip_index;
            }
        }
    }

    let mut consts = ConstWriter::for_build("constants").unwrap().finish_dependencies();

    consts.add_value("N", "usize", N);
    consts.add_value("FACT", "[usize; 7]", FACT);
    consts.add_value("ROWS", "usize", ROWS);

    consts.add_value("DIRECTIONS", "usize", DIRECTIONS);
    consts.add_value("FORWARD", "usize", FORWARD);
    consts.add_value("BACKWARD", "usize", BACKWARD);

    consts.add_value("TABLE", "[[[[usize; ROWS]; N - 1]; FACT[N] / 2]; DIRECTIONS]", table);
    consts.add_value("FLIPS", "[[[usize; N - 1]; FACT[N] / 2]; DIRECTIONS]", flips);
    consts.add_value("MAPPINGS", "[usize; FACT[N] / 2]", FactTable(mappings));
}

fn permutation_id_mappings() -> [usize; FACT[N] / 2] {
    let mut mappings = [0; FACT[N] / 2];
    let mut counter = 0;

    for id in 0..FACT[N] {
        let current = Lehmer::from_decimal(id, N).to_permutation();

        if current[0] > current[N - 1] {
            continue;
        }

        mappings[counter] = id;

        counter += 1;
    }

    return mappings;
}

fn lookup_mapping_index(perm: &mut Vec<u8>, mappings: &[usize; FACT[N] / 2]) -> (usize, usize) {
    let mut direction = FORWARD;

    if perm[0] > perm[N - 1] {
        direction = BACKWARD;
        perm.reverse();
    }

    let id_to_find = Lehmer::from_permutation(perm).to_decimal();

    for (index, id) in mappings.iter().enumerate() {
        if *id == id_to_find {
            return (index, direction);
        }
    }

    panic!("unreachable");
}


// Rust doesn't implement Debug for arrays larger than 32 elements so wrap the
// specific array sizes we need in structs and implement Debug explicitly.

use std::fmt;

#[derive(Copy, Clone)]
struct FactTable<T>([T; FACT[N] / 2]);
impl<T: fmt::Debug> fmt::Debug for FactTable<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(formatter)
    }
}

#[derive(Copy, Clone)]
struct RowTable<T>([T; ROWS]);
impl<T: fmt::Debug> fmt::Debug for RowTable<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(formatter)
    }
}
