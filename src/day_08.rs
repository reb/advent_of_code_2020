/// --- Day 8: Handheld Halting ---
///
/// Your flight to the major airline hub reaches cruising altitude without
/// incident. While you consider checking the in-flight menu for one of those
/// drinks that come with a little umbrella, you are interrupted by the kid
/// sitting next to you.
///
/// Their handheld game console won't turn on! They ask if you can take a look.
///
/// You narrow the problem down to a strange infinite loop in the boot code
/// (your puzzle input) of the device. You should be able to fix it, but first
/// you need to be able to run the code in isolation.
///
/// The boot code is represented as a text file with one instruction per line of
/// text. Each instruction consists of an operation (acc, jmp, or nop) and an
/// argument (a signed number like +4 or -20).
///
///     acc increases or decreases a single global value called the accumulator
///     by the value given in the argument. For example, acc +7 would increase
///     the accumulator by 7. The accumulator starts at 0. After an acc
///     instruction, the instruction immediately below it is executed next.
///     jmp jumps to a new instruction relative to itself. The next instruction
///     to execute is found using the argument as an offset from the jmp
///     instruction; for example, jmp +2 would skip the next instruction, jmp +1
///     would continue to the instruction immediately below it, and jmp -20
///     would cause the instruction 20 lines above to be executed next.
///     nop stands for No OPeration - it does nothing. The instruction
///     immediately below it is executed next.
///
/// For example, consider the following program:
///
/// nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6
///
/// These instructions are visited in this order:
///
/// nop +0  | 1
/// acc +1  | 2, 8(!)
/// jmp +4  | 3
/// acc +3  | 6
/// jmp -3  | 7
/// acc -99 |
/// acc +1  | 4
/// jmp -4  | 5
/// acc +6  |
///
/// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to
/// 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the
/// bottom. After it increases the accumulator from 1 to 2, jmp -4 executes,
/// setting the next instruction to the only acc +3. It sets the accumulator to
/// 5, and jmp -3 causes the program to continue back at the first acc +1.
///
/// This is an infinite loop: with this sequence of jumps, the program will run
/// forever. The moment the program tries to run any instruction a second time,
/// you know it will never terminate.
///
/// Immediately before the program would run an instruction a second time, the
/// value in the accumulator is 5.
///
/// Run your copy of the boot code. Immediately before any instruction is
/// executed a second time, what value is in the accumulator?
use itertools::Itertools;
use std::collections::HashSet;
use Operation::*;
use ProgramResult::*;

const INPUT: &str = include_str!("../input/day_08.txt");

pub fn run() {
    let program = load_program(INPUT);

    let (_, result_before_loop) = run_program(&program);
    println!(
        "The value of the accumulator before any instruction is executed again is: {}",
        result_before_loop
    )
}

fn run_program(program: &Vec<Instruction>) -> (ProgramResult, i32) {
    let mut index = 0;
    let mut accumulator = 0;
    // keep track of visited location to prevent infinite loops
    let mut visited = HashSet::new();

    loop {
        if index == program.len() {
            return (Finished, accumulator);
        }
        if visited.contains(&index) {
            return (InfiniteLoop, accumulator);
        }
        visited.insert(index);

        match program.get(index) {
            Some(Instruction {
                operation: NoOperation,
                ..
            }) => {
                index += 1;
            }
            Some(Instruction {
                operation: Accumulate,
                argument,
            }) => {
                index += 1;
                accumulator += argument;
            }
            Some(Instruction {
                operation: Jump,
                argument,
            }) => {
                index = (index as i32 + argument) as usize;
            }
            _ => {
                return (InstructionFailure, accumulator);
            }
        };
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    NoOperation,
    Accumulate,
    Jump,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug, PartialEq)]
enum ProgramResult {
    InfiniteLoop,
    Finished,
    InstructionFailure,
}

fn load_program(input: &str) -> Vec<Instruction> {
    input
        .split_whitespace()
        .tuples::<(_, _)>()
        .map(|(op, arg)| match (op, arg.parse()) {
            ("nop", Ok(argument)) => Instruction {
                operation: NoOperation,
                argument,
            },
            ("acc", Ok(argument)) => Instruction {
                operation: Accumulate,
                argument,
            },
            ("jmp", Ok(argument)) => Instruction {
                operation: Jump,
                argument,
            },
            _ => panic!("Could not parse instruction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_program() {
        let input = "\
            nop +0\n\
            acc +1\n\
            jmp +4\n\
            acc +3\n\
            jmp -3\n\
            acc -99\n\
            acc +1\n\
            jmp -4\n\
            acc +6";

        let expected_program = vec![
            Instruction {
                operation: NoOperation,
                argument: 0,
            },
            Instruction {
                operation: Accumulate,
                argument: 1,
            },
            Instruction {
                operation: Jump,
                argument: 4,
            },
            Instruction {
                operation: Accumulate,
                argument: 3,
            },
            Instruction {
                operation: Jump,
                argument: -3,
            },
            Instruction {
                operation: Accumulate,
                argument: -99,
            },
            Instruction {
                operation: Accumulate,
                argument: 1,
            },
            Instruction {
                operation: Jump,
                argument: -4,
            },
            Instruction {
                operation: Accumulate,
                argument: 6,
            },
        ];

        assert_eq!(load_program(input), expected_program);
    }

    #[test]
    fn test_run_program() {
        let program = vec![
            Instruction {
                operation: NoOperation,
                argument: 0,
            },
            Instruction {
                operation: Accumulate,
                argument: 1,
            },
            Instruction {
                operation: Jump,
                argument: 4,
            },
            Instruction {
                operation: Accumulate,
                argument: 3,
            },
            Instruction {
                operation: Jump,
                argument: -3,
            },
            Instruction {
                operation: Accumulate,
                argument: -99,
            },
            Instruction {
                operation: Accumulate,
                argument: 1,
            },
            Instruction {
                operation: Jump,
                argument: -4,
            },
            Instruction {
                operation: Accumulate,
                argument: 6,
            },
        ];

        assert_eq!(run_program(&program), (InfiniteLoop, 5));
    }
}
