
pub const NUM_START: u8 = 0;    // START is named now
pub const NUM_END: u8 = 1;
const NUM_NEG: u8 = 2;
const NUM_DIG19: u8 = 3;
const NUM_DIG09: u8 = 4;
const NUM_DOT: u8 = 5;
const NUM_DIG_AFTER_DOT: u8 = 6;
const NUM_ZERO: u8 = 7;


pub const NUM_TRANSITIONS: [&[u8]; 8] = [
    /* START        : 0 */  &[NUM_NEG, NUM_DIG19, NUM_ZERO],
    /* END          : 1 */  &[],
    /* NUM_NEG      : 2 */  &[NUM_ZERO, NUM_DIG19],
    /* DIG19        : 3 */  &[NUM_DIG09, NUM_DOT, NUM_END],
    /* DIG09        : 4 */  &[NUM_DIG09, NUM_DOT, NUM_END],
    /* DOT          : 5 */  &[NUM_DIG_AFTER_DOT],
    /* DIG_AFTER_DOT: 6 */  &[NUM_DIG_AFTER_DOT, NUM_END],
    /* ZERO         : 7 */  &[NUM_DOT, NUM_END],
];

// This table won't be updated anymore
pub const NUM_ACCEPTED_CHARS_IN_STATES: [&[char]; 8] = [
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
pub const NUM_NAMES_OF_STATES: [&str; 8] = [
    /* START        : 0 */  "START",
    /* END          : 1 */  "END",
    /* NUM_NEG      : 2 */  "NUM_NEG",
    /* DIG19        : 3 */  "DIG19",
    /* DIG09        : 4 */  "DIG09",
    /* DOT          : 5 */  "DOT",
    /* DIG_AFTER_DOT: 6 */  "DIG_AFTER_DOT",
    /* ZERO         : 7 */  "ZERO",
];
