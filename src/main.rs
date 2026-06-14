mod transitions;
use transitions::{NUM_START, NUM_END, NUM_TRANSITIONS, NUM_ACCEPTED_CHARS_IN_STATES, NUM_NAMES_OF_STATES};

fn main() {
    
    // Read chars one-by-one
    let text = "-123.4";
    let mut transition_state_now = NUM_START;
    let mut transition_jump_counter = 0;

    
    for (id, char_next) in text.chars().enumerate() {
        println!("id: {}, char: {}", id, char_next);
        transition_state_now = state_of_next_char(
                                        transition_state_now, 
                                        char_next,
                                        &NUM_TRANSITIONS,
                                        &NUM_ACCEPTED_CHARS_IN_STATES,
                                        &NUM_NAMES_OF_STATES,
        );
        if transition_state_now == NUM_END {
            break
        } else {
            transition_jump_counter += 1;
        }
    }
    println!("FINAL STATE: {:?}, transition_jump_counter: {:?}", transition_state_now, transition_jump_counter);
}

fn state_of_next_char(
    transition_state_actual: u8, 
    character_maybe_correct: char,
    transition_table: &[&[u8]],
    accepted_chars: &[&[char]],
    state_names: &[&str],
) -> u8 {
    println!("transition_state_actual: {}, character_maybe_correct: {}", transition_state_actual, character_maybe_correct);

    let mut state_of_next_char = NUM_END;

    // usize is used for array indexing, u8 is not a valid number for indexing.
    for transition_possible in transition_table[transition_state_actual as usize] {
        println!("  transition_id_possible: {:?} {:?}", transition_possible, state_names[*transition_possible as usize]);
        for char_valid_in_transition in accepted_chars[*transition_possible as usize] {
            if *char_valid_in_transition == character_maybe_correct {
                println!("    char {:?} is valid in {:?}", char_valid_in_transition, transition_possible);

                state_of_next_char = *transition_possible;
                break
            }
       }
       if state_of_next_char != NUM_END { break }  // no more loop if result is detected
    }
    println!("  state of next char {:?}, {:?}", state_of_next_char, NUM_NAMES_OF_STATES[state_of_next_char as usize]);
    return state_of_next_char;
}
