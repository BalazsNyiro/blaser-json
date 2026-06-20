use blaser_json::{START, NO_STATE_JUMP,
                  NUM_NEG, NUM_DIG19, NUM_DIG09, NUM_DOT,
                  NUM_DIG_AFTER_DOT, NUM_ZERO,
                  NUM_TRANSITIONS__INT_FLOAT, NUM_CHARS_ACCEPTED_IN_STATES, NUM_NAMES_OF_STATES };
use blaser_json::state_after_char_reading;



// The last 3 params are always the same - wrapper fun to simplify testing
fn assert_transition_change_num_int_float(transition_state: u8, character: char,
                                          wanted_test_result_transition_state_after_char_reading: u8) -> u8 {
    let state_new = state_after_char_reading(transition_state, character, &NUM_TRANSITIONS__INT_FLOAT,
                             &NUM_CHARS_ACCEPTED_IN_STATES, &NUM_NAMES_OF_STATES);
    assert_eq!(state_new, wanted_test_result_transition_state_after_char_reading);
    return state_new;
}

#[test]
fn test_num_int_float_state_of_next_char() {

    let mut trans_state = START;

    // valid input: -1.23
    trans_state = assert_transition_change_num_int_float(trans_state, '-', NUM_NEG);
    trans_state = assert_transition_change_num_int_float(trans_state, '1', NUM_DIG19);
    trans_state = assert_transition_change_num_int_float(trans_state, '.', NUM_DOT);
    trans_state = assert_transition_change_num_int_float(trans_state, '2', NUM_DIG_AFTER_DOT);
    trans_state = assert_transition_change_num_int_float(trans_state, '3', NUM_DIG_AFTER_DOT);

    // the next char is not in the valid flow,
    trans_state = assert_transition_change_num_int_float(trans_state, '+', NO_STATE_JUMP);
}

