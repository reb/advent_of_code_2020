/// --- Day 15: Rambunctious Recitation ---
///
/// You catch the airport shuttle and try to book a new flight to your vacation
/// island. Due to the storm, all direct flights have been cancelled, but a
/// route is available to get around the storm. You take it.
///
/// While you wait for your flight, you decide to check in with the Elves back
/// at the North Pole. They're playing a memory game and are ever so excited to
/// explain the rules!
///
/// In this game, the players take turns saying numbers. They begin by taking
/// turns reading from a list of starting numbers (your puzzle input). Then,
/// each turn consists of considering the most recently spoken number:
///
///   - If that was the first time the number has been spoken, the current
///     player says 0.
///   - Otherwise, the number had been spoken before; the current player
///     announces how many turns apart the number is from when it was previously
///     spoken.
///
/// So, after the starting numbers, each turn results in that player speaking
/// aloud either 0 (if the last number is new) or an age (if the last number is
/// a repeat).
///
/// For example, suppose the starting numbers are 0,3,6:
///
///   - Turn 1: The 1st number spoken is a starting number, 0.
///   - Turn 2: The 2nd number spoken is a starting number, 3.
///   - Turn 3: The 3rd number spoken is a starting number, 6.
///   - Turn 4: Now, consider the last number spoken, 6. Since that was the
///     first time the number had been spoken, the 4th number spoken is 0.
///   - Turn 5: Next, again consider the last number spoken, 0. Since it had
///     been spoken before, the next number to speak is the difference between
///     the turn number when it was last spoken (the previous turn, 4) and the
///     turn number of the time it was most recently spoken before then (turn
///     1). Thus, the 5th number spoken is 4 - 1, 3.
///   - Turn 6: The last number spoken, 3 had also been spoken before, most
///     recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
///   - Turn 7: Since 3 was just spoken twice in a row, and the last two turns
///     are 1 turn apart, the 7th number spoken is 1.
///   - Turn 8: Since 1 is new, the 8th number spoken is 0.
///   - Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is
///     the difference between them, 4.
///   - Turn 10: 4 is new, so the 10th number spoken is 0.
///
/// (The game ends when the Elves get sick of playing or dinner is ready,
/// whichever comes first.)
///
/// Their question for you is: what will be the 2020th number spoken? In the
/// example above, the 2020th number spoken will be 436.
///
/// Here are a few more examples:
///
///   - Given the starting numbers 1,3,2, the 2020th number spoken is 1.
///   - Given the starting numbers 2,1,3, the 2020th number spoken is 10.
///   - Given the starting numbers 1,2,3, the 2020th number spoken is 27.
///   - Given the starting numbers 2,3,1, the 2020th number spoken is 78.
///   - Given the starting numbers 3,2,1, the 2020th number spoken is 438.
///   - Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
///
/// Given your starting numbers, what will be the 2020th number spoken?
use std::collections::HashMap;

pub fn run() {
    let start_sequence = vec![2, 0, 6, 12, 1, 3];

    let spoken_at_turn_2020 = play_memory_game(&start_sequence, 2020);

    println!("The 2020th number spoken is: {}", spoken_at_turn_2020);
}

fn play_memory_game(start_sequence: &[u32], goal: usize) -> u32 {
    // keep track of which turn a number was said on last
    let mut memory = HashMap::new();

    let mut number = *start_sequence.get(0).expect("Start sequence is empty");

    for turn in 1..goal {
        let new_number = match start_sequence.get(turn) {
            Some(&starting_number) => starting_number,
            None => match memory.get(&number) {
                Some(turn_number_was_last_said) => (turn - turn_number_was_last_said) as u32,
                None => 0,
            },
        };
        memory.insert(number, turn);
        number = new_number;
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_memory_game_first_10_turns() {
        let start_sequence = vec![0, 3, 6];
        assert_eq!(play_memory_game(&start_sequence, 1), 0);
        assert_eq!(play_memory_game(&start_sequence, 2), 3);
        assert_eq!(play_memory_game(&start_sequence, 3), 6);
        assert_eq!(play_memory_game(&start_sequence, 4), 0);
        assert_eq!(play_memory_game(&start_sequence, 5), 3);
        assert_eq!(play_memory_game(&start_sequence, 6), 3);
        assert_eq!(play_memory_game(&start_sequence, 7), 1);
        assert_eq!(play_memory_game(&start_sequence, 8), 0);
        assert_eq!(play_memory_game(&start_sequence, 9), 4);
        assert_eq!(play_memory_game(&start_sequence, 10), 0);
    }

    #[test]
    fn test_play_memory_game_long_1() {
        let start_sequence = vec![0, 3, 6];
        assert_eq!(play_memory_game(&start_sequence, 2020), 436);
    }

    #[test]
    fn test_play_memory_game_long_2() {
        // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
        let start_sequence = vec![1, 3, 2];
        assert_eq!(play_memory_game(&start_sequence, 2020), 1);
    }

    #[test]
    fn test_play_memory_game_long_3() {
        // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
        let start_sequence = vec![2, 1, 3];
        assert_eq!(play_memory_game(&start_sequence, 2020), 10);
    }

    #[test]
    fn test_play_memory_game_long_4() {
        // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
        let start_sequence = vec![1, 2, 3];
        assert_eq!(play_memory_game(&start_sequence, 2020), 27);
    }

    #[test]
    fn test_play_memory_game_long_5() {
        // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
        let start_sequence = vec![2, 3, 1];
        assert_eq!(play_memory_game(&start_sequence, 2020), 78);
    }

    #[test]
    fn test_play_memory_game_long_6() {
        // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
        let start_sequence = vec![3, 2, 1];
        assert_eq!(play_memory_game(&start_sequence, 2020), 438);
    }

    #[test]
    fn test_play_memory_game_long_7() {
        // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
        let start_sequence = vec![3, 1, 2];
        assert_eq!(play_memory_game(&start_sequence, 2020), 1836);
    }
}
