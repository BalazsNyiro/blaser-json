const START: u8 = 0;    // START is named now
const END: u8 = 1;
const NUM_NEG: u8 = 2;
const DIG19: u8 = 3;
const DIG09: u8 = 4;
const DOT: u8 = 5;
const DIG_AFTER_DOT: u8 = 6;
const ZERO: u8 = 7;

const NUM_TRANSITIONS: [&[u8]; 8] = [
    /* START        : 0 */  &[NUM_NEG, DIG19, ZERO],
    /* END          : 1 */  &[],
    /* NUM_NEG      : 2 */  &[ZERO, DIG19],
    /* DIG19        : 3 */  &[DIG09, DOT, END],
    /* DIG09        : 4 */  &[DIG09, DOT, END],
    /* DOT          : 5 */  &[DIG_AFTER_DOT],
    /* DIG_AFTER_DOT: 6 */  &[DIG_AFTER_DOT, END],
    /* ZERO         : 7 */  &[DOT, END],
];

// This table won't be updated anymore
const NUM_ACCEPTED_CHARS_IN_STATES: [&[char]; 8] = [
    /* START        : 0 */  &[],  // no accepted char
    /* END          : 1 */  &[],  // no accepted char
    /* NUM_NEG      : 2 */  &['-'],
    /* DIG19        : 3 */  &[     '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* DIG09        : 4 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* DOT          : 5 */  &['.'],
    /* DIG_AFTER_DOT: 6 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* ZERO         : 7 */  &['0'],
];


// id -> human readable name 
const NUM_NAMES_OF_STATES: [&str; 8] = [
    /* START        : 0 */  "START",
    /* END          : 1 */  "END",
    /* NUM_NEG      : 2 */  "NUM_NEG",
    /* DIG19        : 3 */  "DIG19",
    /* DIG09        : 4 */  "DIG09",
    /* DOT          : 5 */  "DOT",
    /* DIG_AFTER_DOT: 6 */  "DIG_AFTER_DOT",
    /* ZERO         : 7 */  "ZERO",
];


fn main() {
    
    // Read chars one-by-one
    let text = "-123.4";
    let mut transition_state_now = START;
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
        if transition_state_now == END {
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

    let mut state_of_next_char = END;

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
       if state_of_next_char != END{ break }  // no more loop if result is detected
    }
    println!("  state of next char {:?}, {:?}", state_of_next_char, NUM_NAMES_OF_STATES[state_of_next_char as usize]);
    return state_of_next_char;
}
