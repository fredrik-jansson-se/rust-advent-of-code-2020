use std::collections::HashMap;

const SUB_DIV: usize = 20201227;

pub fn run() {
    println!("25:1 {}", run_1(17773298, 15530095));
}

fn transform_subject_number(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % SUB_DIV;
    }
    value
}

fn run_1(card_pk: usize, door_pk: usize) -> usize {
    let subject_key = 7;
    // let mut lookup = HashMap::new();
    for loop_size in 1.. {
        let transformed = transform_subject_number(subject_key, loop_size);
        // lookup.insert(loop_size, transformed);
        if transformed == card_pk {
            println!("Found card");
            return transform_subject_number(door_pk, loop_size);
        } else if transformed == door_pk {
            println!("Found door");
            return transform_subject_number(card_pk, loop_size);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc25_run_1() {
        assert_eq!(super::run_1(5764801, 17807724), 14897079);
    }
}
