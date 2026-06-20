// compile transitions.rs like a module
pub mod transitions;

// pub use ... re-export these constants, to re-use them in external tests
pub use transitions::{START, NO_STATE_JUMP,
                      NUM_NEG, NUM_DIG19, NUM_DIG09,  NUM_DOT,
                      NUM_DIG_AFTER_DOT, NUM_ZERO,
                      NUM_TRANSITIONS, NUM_CHARS_ACCEPTED_IN_STATES, NUM_NAMES_OF_STATES};

/*
    TODO: test/coverage

    Run tests: cargo test

*/

pub fn parse_json(text: &str) {
    
    // Read chars one-by-one
    let text = "-123.4";
    let mut transition_state = START;
    let mut transition_jump_counter = 0;

    
    for (id, char) in text.chars().enumerate() {
        println!("id: {}, char: {}", id, char);
        transition_state = state_after_char_reading(
            transition_state, char,
            &NUM_TRANSITIONS, &NUM_CHARS_ACCEPTED_IN_STATES, &NUM_NAMES_OF_STATES,
        );
        if transition_state == NO_STATE_JUMP {
            break
        } else {
            transition_jump_counter += 1;
        }
    }
    println!("FINAL STATE: {:?}, transition_jump_counter: {:?}", transition_state, transition_jump_counter);
}

pub fn state_after_char_reading(
    transition_state_actual: u8,
    character: char,
    transition_table: &[&[u8]],
    chars_accepted: &[&[char]],
    state_names: &[&str],
) -> u8 {
    println!("transition_state_actual: {}, character next: {}", transition_state_actual, character);

    let mut state_after_char_reading_and_transation_analyse = NO_STATE_JUMP;

    // usize is used for array indexing, u8 is not a valid number for indexing.
    for transition_possible in transition_table[transition_state_actual as usize] {
        println!("  transition_id_possible: {:?} {:?}", transition_possible, state_names[*transition_possible as usize]);
        for char_valid_in_transition in chars_accepted[*transition_possible as usize] {
            if *char_valid_in_transition == character {
                println!("    char {:?} is valid in {:?}", char_valid_in_transition, transition_possible);

                state_after_char_reading_and_transation_analyse = *transition_possible;
                break
            }
       }
       if state_after_char_reading_and_transation_analyse != NO_STATE_JUMP { break }  // no more loop if result is detected
    }
    println!("  state of next char {:?}, {:?}", state_after_char_reading_and_transation_analyse, NUM_NAMES_OF_STATES[state_after_char_reading_and_transation_analyse as usize]);
    return state_after_char_reading_and_transation_analyse;
}
