use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("resources/input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<i32>) {
    let mut memory = input.to_vec();
    memory[1] = 12;
    memory[2] = 2;

    println!("part 1: {}", execute_intcode(&memory)[0]);
}

fn part_two(input: &Vec<i32>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = input.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if execute_intcode(&memory)[0] == 19690720 {
                println!("part 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn execute_intcode(program: &Vec<i32>) -> Vec<i32> {
    let mut memory = program.clone();

    for x in (0..memory.len()).step_by(4) {
        let asgn = memory[x+3] as usize;

        match memory[x] {
            1 => {
                memory[asgn] = memory[memory[x+1] as usize] + memory[memory[x+2] as usize]
            },
            2 => {
                memory[asgn] = memory[memory[x+1] as usize] * memory[memory[x+2] as usize]
            },
            99 => {
                break;
            },
            _ => println!("Invalid opcode: {}", memory[x]),
        }
    }

    memory
}
