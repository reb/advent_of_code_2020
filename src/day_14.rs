/// --- Day 14: Docking Data ---
///
/// As your ferry approaches the sea port, the captain asks for your help again.
/// The computer system that runs this port isn't compatible with the docking
/// program on the ferry, so the docking parameters aren't being correctly
/// initialized in the docking program's memory.
///
/// After a brief inspection, you discover that the sea port's computer system
/// uses a strange bitmask system in its initialization program. Although you
/// don't have the correct decoder chip handy, you can emulate it in software!
///
/// The initialization program (your puzzle input) can either update the bitmask
/// or write a value to memory. Values and memory addresses are both 36-bit
/// unsigned integers. For example, ignoring bitmasks for a moment, a line like
/// mem[8] = 11 would write the value 11 to memory address 8.
///
/// The bitmask is always given as a string of 36 bits, written with the most
/// significant bit (representing 2^35) on the left and the least significant
/// bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied
/// to values immediately before they are written to memory: a 0 or 1 overwrites
/// the corresponding bit in the value, while an X leaves the bit in the value
/// unchanged.
///
/// For example, consider the following program:
///
/// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// mem[8] = 11
/// mem[7] = 101
/// mem[8] = 0
///
/// This program starts by specifying a bitmask (mask = ....). The mask it
/// specifies will overwrite two bits in every written value: the 2s bit is
/// overwritten with 0, and the 64s bit is overwritten with 1.
///
/// The program then attempts to write the value 11 to memory address 8. By
/// expanding everything out to individual bits, the mask is applied as follows:
///
/// value:  000000000000000000000000000000001011  (decimal 11)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001001001  (decimal 73)
///
/// So, because of the mask, the value 73 is written to memory address 8
/// instead. Then, the program tries to write 101 to address 7:
///
/// value:  000000000000000000000000000001100101  (decimal 101)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001100101  (decimal 101)
///
/// This time, the mask has no effect, as the bits it overwrote were already the
/// values the mask tried to set. Finally, the program tries to write 0 to
/// address 8:
///
/// value:  000000000000000000000000000000000000  (decimal 0)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001000000  (decimal 64)
///
/// 64 is written to address 8 instead, overwriting the value that was there
///    previously.
///
/// To initialize your ferry's docking program, you need the sum of all values
/// left in memory after the initialization program completes. (The entire
/// 36-bit address space begins initialized to the value 0 at every address.) In
/// the above example, only two values in memory are not zero - 101 (at address
/// 7) and 64 (at address 8) - producing a sum of 165.
///
/// Execute the initialization program. What is the sum of all values left in
/// memory after it completes?
///
/// --- Part Two ---
///
/// For some reason, the sea port's computer system still can't communicate with
/// your ferry's docking program. It must be using version 2 of the decoder
/// chip!
///
/// A version 2 decoder chip doesn't modify the values being written at all.
/// Instead, it acts as a memory address decoder. Immediately before a value is
/// written to memory, each bit in the bitmask modifies the corresponding bit of
/// the destination memory address in the following way:
///
///   - If the bitmask bit is 0, the corresponding memory address bit is
///     unchanged.
///   - If the bitmask bit is 1, the corresponding memory address bit is
///     overwritten with 1.
///   - If the bitmask bit is X, the corresponding memory address bit is
///     floating.
///
/// A floating bit is not connected to anything and instead fluctuates
/// unpredictably. In practice, this means the floating bits will take on all
/// possible values, potentially causing many memory addresses to be written all
/// at once!
///
/// For example, consider the following program:
///
/// mask = 000000000000000000000000000000X1001X
/// mem[42] = 100
/// mask = 00000000000000000000000000000000X0XX
/// mem[26] = 1
///
/// When this program goes to write to memory address 42, it first applies the
/// bitmask:
///
/// address: 000000000000000000000000000000101010  (decimal 42)
/// mask:    000000000000000000000000000000X1001X
/// result:  000000000000000000000000000000X1101X
///
/// After applying the mask, four bits are overwritten, three of which are
/// different, and two of which are floating. Floating bits take on every
/// possible combination of values; with two floating bits, four actual memory
/// addresses are written:
///
/// 000000000000000000000000000000011010  (decimal 26)
/// 000000000000000000000000000000011011  (decimal 27)
/// 000000000000000000000000000000111010  (decimal 58)
/// 000000000000000000000000000000111011  (decimal 59)
///
/// Next, the program is about to write to memory address 26 with a different
/// bitmask:
///
/// address: 000000000000000000000000000000011010  (decimal 26)
/// mask:    00000000000000000000000000000000X0XX
/// result:  00000000000000000000000000000001X0XX
///
/// This results in an address with three floating bits, causing writes to eight
/// memory addresses:
///
/// 000000000000000000000000000000010000  (decimal 16)
/// 000000000000000000000000000000010001  (decimal 17)
/// 000000000000000000000000000000010010  (decimal 18)
/// 000000000000000000000000000000010011  (decimal 19)
/// 000000000000000000000000000000011000  (decimal 24)
/// 000000000000000000000000000000011001  (decimal 25)
/// 000000000000000000000000000000011010  (decimal 26)
/// 000000000000000000000000000000011011  (decimal 27)
///
/// The entire 36-bit address space still begins initialized to the value 0 at
/// every address, and you still need the sum of all values left in memory at
/// the end of the program. In this example, the sum is 208.
///
/// Execute the initialization program using an emulator for a version 2 decoder
/// chip. What is the sum of all values left in memory after it completes?
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day_14.txt");

pub fn run() {
    let program = parse_program(INPUT);

    let memory = run_program(&program);
    let sum_of_memory: u64 = memory.values().sum();
    println!(
        "The sum of all values left in memory after the initialization program completes is: {}",
        sum_of_memory
    );

    let memory_v2 = run_program_v2(&program);
    let sum_of_memory_v2: u64 = memory_v2.values().sum();
    println!(
        "The sum of all values left in memory after emulating v2 of the decoder chip is: {}",
        sum_of_memory_v2
    );
}

fn run_program(program: &[ProgramStep]) -> Memory {
    let mut mask = [None].iter().cycle().take(36).cloned().collect();
    let mut memory = Memory::new();

    for step in program.iter() {
        match step {
            ProgramStep::SetMask(new_mask) => mask = new_mask.clone(),
            ProgramStep::SetMemory(address, value) => {
                memory.insert(*address, apply_mask(*value, &mask));
            }
        }
    }

    memory
}

fn run_program_v2(program: &[ProgramStep]) -> Memory {
    let mut mask = [None].iter().cycle().take(36).cloned().collect();
    let mut memory = Memory::new();

    for step in program.iter() {
        match step {
            ProgramStep::SetMask(new_mask) => mask = new_mask.clone(),
            ProgramStep::SetMemory(address, value) => {
                for decoded_address in decode_address(*address, &mask).iter() {
                    memory.insert(*decoded_address, *value);
                }
            }
        }
    }

    memory
}

fn apply_mask(value: u64, mask: &Mask) -> u64 {
    let value_bit_vec = to_bit_vec(value);
    to_u64(
        mask.iter()
            .zip(value_bit_vec.into_iter())
            .map(|(&mask_bit, value_bit)| match mask_bit {
                None => value_bit,
                Some(n) => n,
            })
            .collect(),
    )
}

fn decode_address(address: u64, mask: &Mask) -> HashSet<u64> {
    let address_bit_vec = to_bit_vec(address);

    let masked_address = mask
        .iter()
        .zip(address_bit_vec.into_iter())
        .map(|(&mask_bit, address_bit)| match mask_bit {
            None => None,
            Some(true) => Some(true),
            Some(false) => Some(address_bit),
        })
        .collect();

    expand_masked_address(masked_address)
}

fn expand_masked_address(masked_address: Mask) -> HashSet<u64> {
    masked_address
        .into_iter()
        .fold(vec![0].into_iter().collect(), |addresses, bit| {
            addresses
                .into_iter()
                .flat_map(|address| {
                    let shifted_address = address << 1;
                    match bit {
                        None => vec![shifted_address + 1, shifted_address],
                        Some(a) => vec![shifted_address + (a as u64)],
                    }
                })
                .collect()
        })
}

fn to_bit_vec(mut n: u64) -> Vec<bool> {
    let mut bit_vec = Vec::new();
    // create a vector of 36 bits
    for _ in 0..36 {
        bit_vec.push(n % 2 == 1);
        n /= 2;
    }
    bit_vec.reverse();
    bit_vec
}

fn to_u64(bit_vec: Vec<bool>) -> u64 {
    bit_vec
        .into_iter()
        .fold(0, |acc, val| (acc << 1) + (val as u64))
}

#[derive(Debug, PartialEq)]
enum ProgramStep {
    SetMask(Mask),
    SetMemory(u64, u64),
}

type Mask = Vec<Option<bool>>;
type Memory = HashMap<u64, u64>;

fn parse_program(input: &str) -> Vec<ProgramStep> {
    input.lines().filter_map(convert_to_program_step).collect()
}

fn convert_to_program_step(line: &str) -> Option<ProgramStep> {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"^mask = ([10X]{36})$").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
    }
    if let Some(captures) = MASK_RE.captures(line) {
        return create_mask_program_step(captures);
    }
    if let Some(captures) = MEM_RE.captures(line) {
        return create_mem_program_step(captures);
    }
    None
}

fn create_mask_program_step(captures: regex::Captures) -> Option<ProgramStep> {
    match captures
        .get(1)
        .map(|mask_match| create_mask(mask_match.as_str()))
    {
        Some(mask) => Some(ProgramStep::SetMask(mask)),
        _ => None,
    }
}

fn create_mask(mask_str: &str) -> Mask {
    mask_str
        .chars()
        .map(|c| match c {
            '1' => Some(true),
            '0' => Some(false),
            'X' => None,
            _ => panic!("Unknown character in mask"),
        })
        .collect()
}

fn create_mem_program_step(captures: regex::Captures) -> Option<ProgramStep> {
    match (
        captures
            .get(1)
            .and_then(|address_match| address_match.as_str().parse().ok()),
        captures
            .get(2)
            .and_then(|value_match| value_match.as_str().parse().ok()),
    ) {
        (Some(address), Some(value)) => Some(ProgramStep::SetMemory(address, value)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let input = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mem[8] = 0";

        let expected_program = vec![
            ProgramStep::SetMask(vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(true),
                None,
                None,
                None,
                None,
                Some(false),
                None,
            ]),
            ProgramStep::SetMemory(8, 11),
            ProgramStep::SetMemory(7, 101),
            ProgramStep::SetMemory(8, 0),
        ];

        assert_eq!(parse_program(input), expected_program);
    }

    #[test]
    fn test_run_program() {
        let program = vec![
            ProgramStep::SetMask(vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(true),
                None,
                None,
                None,
                None,
                Some(false),
                None,
            ]),
            ProgramStep::SetMemory(8, 11),
            ProgramStep::SetMemory(7, 101),
            ProgramStep::SetMemory(8, 0),
        ];

        let mut expected_memory = Memory::new();
        expected_memory.insert(7, 101);
        expected_memory.insert(8, 64);

        assert_eq!(run_program(&program), expected_memory);
    }

    #[test]
    fn test_run_program_v2() {
        // mask = 000000000000000000000000000000X1001X
        // mem[42] = 100
        // mask = 00000000000000000000000000000000X0XX
        // mem[26] = 1
        let program = vec![
            ProgramStep::SetMask(vec![
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                None,
                Some(true),
                Some(false),
                Some(false),
                Some(true),
                None,
            ]),
            ProgramStep::SetMemory(42, 100),
            ProgramStep::SetMask(vec![
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                Some(false),
                None,
                Some(false),
                None,
                None,
            ]),
            ProgramStep::SetMemory(26, 1),
        ];

        let mut expected_memory = Memory::new();
        expected_memory.insert(16, 1);
        expected_memory.insert(17, 1);
        expected_memory.insert(18, 1);
        expected_memory.insert(19, 1);
        expected_memory.insert(24, 1);
        expected_memory.insert(25, 1);
        expected_memory.insert(26, 1);
        expected_memory.insert(27, 1);
        expected_memory.insert(58, 100);
        expected_memory.insert(59, 100);

        assert_eq!(run_program_v2(&program), expected_memory);
    }

    #[test]
    fn test_to_bit_vec() {
        let n = 101;

        let expected_bit_vec = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, true, false, false, true, false, true,
        ];

        assert_eq!(to_bit_vec(n), expected_bit_vec);
    }

    #[test]
    fn test_to_u64() {
        let bit_vec = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, true, false, false, true, false, true,
        ];
        let expected_n = 101;

        assert_eq!(to_u64(bit_vec), expected_n);
    }

    #[test]
    fn test_expand_masked_address() {
        let masked_address = vec![
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            None,
            Some(true),
            Some(true),
            Some(false),
            Some(true),
            None,
        ];

        let mut expected_addresses = HashSet::new();
        expected_addresses.insert(26);
        expected_addresses.insert(27);
        expected_addresses.insert(58);
        expected_addresses.insert(59);

        assert_eq!(expand_masked_address(masked_address), expected_addresses)
    }
}
