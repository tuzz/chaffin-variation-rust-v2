include!(concat!(env!("OUT_DIR"), "/constants.rs"));

const N: usize = 6;
const FACT: [usize; 7] = [1, 1, 2, 6, 24, 120, 720];
const ROWS: usize = 1 + 2 + 6 + 24 + 120;

const DIRECTIONS: usize = 2;
const FORWARD: usize = 0;
const BACKWARD: usize = 1;

use lehmer::Lehmer;

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

fn main() {
    let mappings = permutation_id_mappings();

    let mut table = [[[[0; ROWS]; N - 1]; FACT[N] / 2]; DIRECTIONS];
    let mut flips = [[[0; N - 1]; FACT[N] / 2]; DIRECTIONS];

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
                    table[current_direction][i][waste][row] = *next_index;
                }

                let flip_index = next_perms.iter()
                    .position(|(_, direction)| *direction == BACKWARD)
                    .unwrap_or(FACT[head.len()]);

                flips[current_direction][i][waste] = flip_index;
            }
        }
    }
}
