
/////// General states in all transitions ////////
pub const START: u8 = 0;    // START is named now
pub const NO_STATE_JUMP: u8 = 1;
//////////////////////////////////////////////////

pub const NUM_NEG: u8 = 2;
pub const NUM_DIG19: u8 = 3;
pub const NUM_DIG09: u8 = 4;
pub const NUM_DOT: u8 = 5;
pub const NUM_DIG_AFTER_DOT: u8 = 6;
pub const NUM_ZERO: u8 = 7;

// The simpler transition table: int/float handle only,
// without scientific num representation
pub const NUM_TRANSITIONS__INT_FLOAT: [&[u8]; 8] = [
    /* START        : 0 */  &[NUM_NEG, NUM_DIG19, NUM_ZERO],
    /* NO_STATE_JUMP: 1 */  &[],
    /* NUM_NEG      : 2 */  &[NUM_ZERO, NUM_DIG19],
    /* DIG19        : 3 */  &[NUM_DIG09, NUM_DOT, NO_STATE_JUMP],
    /* DIG09        : 4 */  &[NUM_DIG09, NUM_DOT, NO_STATE_JUMP],
    /* DOT          : 5 */  &[NUM_DIG_AFTER_DOT],
    /* DIG_AFTER_DOT: 6 */  &[NUM_DIG_AFTER_DOT, NO_STATE_JUMP],
    /* ZERO         : 7 */  &[NUM_DOT, NO_STATE_JUMP],
];

// This table won't be updated anymore
// can handle the int/float only mode + scientific mode too,
// no need to separate them
pub const NUM_CHARS_ACCEPTED_IN_STATES: [&[char]; 8] = [
    /* START        : 0 */  &[],  // no accepted char
    /* NO_STATE_JUMP: 1 */  &[],  // no accepted char
    /* NUM_NEG      : 2 */  &['-'],
    /* DIG19        : 3 */  &[     '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* DIG09        : 4 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* DOT          : 5 */  &['.'],
    /* DIG_AFTER_DOT: 6 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* ZERO         : 7 */  &['0'],

    ///////////////////// Scientific number representation ////////////////////
];


// id -> human readable name
// can handle the int/float only mode + scientific mode too,
// no need to separate them
pub const NUM_NAMES_OF_STATES: [&str; 8] = [
    /* START        : 0 */  "START",
    /* NO_STATE_JUMP: 1 */  "NO_STATE_JUMP",
    /* NUM_NEG      : 2 */  "NUM_NEG",
    /* DIG19        : 3 */  "DIG19",
    /* DIG09        : 4 */  "DIG09",
    /* DOT          : 5 */  "DOT",
    /* DIG_AFTER_DOT: 6 */  "DIG_AFTER_DOT",
    /* ZERO         : 7 */  "ZERO",

    ///////////////////// Scientific number representation ////////////////////
];
