use blaser_json::{START,
                  NUM_NEG, NUM_DIG19, NUM_DIG09, NUM_DOT,
                  NUM_DIG_AFTER_DOT, NUM_ZERO,
                  NUM_TRANSITIONS__INT_FLOAT, NUM_CHARS_ACCEPTED_IN_STATES, NUM_NAMES_OF_STATES };
use blaser_json::state_after_char_reading;



// The last 3 params are always the same - wrapper fun to simplify testing
fn assert_state_change_num_int_float(transition_state: u8, character: char,
                                     wanted_test_result_transition_state_after_char_reading: u8,
                                     wanted_valid_jump_happened: bool ) -> u8 {
    let (valid_transaction_jump_happened, state_new) = state_after_char_reading(
        transition_state, character, &NUM_TRANSITIONS__INT_FLOAT,
        &NUM_CHARS_ACCEPTED_IN_STATES, &NUM_NAMES_OF_STATES);
    assert_eq!(state_new, wanted_test_result_transition_state_after_char_reading);
    assert_eq!(valid_transaction_jump_happened, wanted_valid_jump_happened);
    return state_new;
}

#[test]
fn test_num_int_float_state_of_next_char() {

    let mut trans_state = START;

    // valid input: -12.34
    trans_state = assert_state_change_num_int_float(trans_state, '-', NUM_NEG, true);
    trans_state = assert_state_change_num_int_float(trans_state, '1', NUM_DIG19, true);
    trans_state = assert_state_change_num_int_float(trans_state, '2', NUM_DIG09, true);
    trans_state = assert_state_change_num_int_float(trans_state, '.', NUM_DOT, true);
    trans_state = assert_state_change_num_int_float(trans_state, '3', NUM_DIG_AFTER_DOT, true);
    trans_state = assert_state_change_num_int_float(trans_state, '4', NUM_DIG_AFTER_DOT, true);

    // the next char is not in the valid flow,
    // the state is unchanged because the char was not valid and there was no-state-jump
    trans_state = assert_state_change_num_int_float(trans_state, '+', NUM_DIG_AFTER_DOT, false);


    // test 0.12
    trans_state = START;
    trans_state = assert_state_change_num_int_float(trans_state, '0', NUM_ZERO, true);
    trans_state = assert_state_change_num_int_float(trans_state, '.', NUM_DOT, true);
    trans_state = assert_state_change_num_int_float(trans_state, '1', NUM_DIG_AFTER_DOT, true);
    trans_state = assert_state_change_num_int_float(trans_state, '2', NUM_DIG_AFTER_DOT, true);
}

