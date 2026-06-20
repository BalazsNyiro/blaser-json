use blaser_json::{START, NO_STATE_JUMP,
                  NUM_NEG, NUM_DIG19, NUM_DIG09,  NUM_DOT,
                  NUM_DIG_AFTER_DOT, NUM_ZERO,
                  NUM_TRANSITIONS, NUM_CHARS_ACCEPTED_IN_STATES, NUM_NAMES_OF_STATES };
use blaser_json::state_after_char_reading;

#[test]
fn test_state_of_next_char() {

    let mut transition_state = START;
    let mut char = '-';

    transition_state = state_after_char_reading(transition_state, char, &NUM_TRANSITIONS,
                                                &NUM_CHARS_ACCEPTED_IN_STATES, &NUM_NAMES_OF_STATES);
    assert_eq!(transition_state, NUM_NEG);

    char = '1';  // the previous transation_state is kept!!!
    transition_state = state_after_char_reading(transition_state, char, &NUM_TRANSITIONS,
                                                &NUM_CHARS_ACCEPTED_IN_STATES, &NUM_NAMES_OF_STATES);
    assert_eq!(transition_state, NUM_DIG19);
}

