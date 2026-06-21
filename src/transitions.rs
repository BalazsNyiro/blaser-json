
/////// General states in all transitions ////////
pub const START: u8 = 0;    // START is named now
//////////////////////////////////////////////////

pub const NUM_NEG: u8 = 1;
pub const NUM_DIG19: u8 = 2;
pub const NUM_DIG09: u8 = 3;
pub const NUM_DOT: u8 = 4;
pub const NUM_DIG_AFTER_DOT: u8 = 5;
pub const NUM_ZERO: u8 = 6;

// The simpler transition table: int/float handle only,
// without scientific num representation
pub const NUM_TRANSITIONS__INT_FLOAT: [&[u8]; 7] = [
    /* START            : 0 */  &[NUM_NEG, NUM_DIG19, NUM_ZERO],
    /* NUM_NEG          : 1 */  &[NUM_ZERO, NUM_DIG19],
    /* NUM_DIG19        : 2 */  &[NUM_DIG09, NUM_DOT],
    /* NUM_DIG09        : 3 */  &[NUM_DIG09, NUM_DOT],
    /* NUM_DOT          : 4 */  &[NUM_DIG_AFTER_DOT],
    /* NUM_DIG_AFTER_DOT: 5 */  &[NUM_DIG_AFTER_DOT],
    /* NUM_ZERO         : 6 */  &[NUM_DOT],
];

// This table won't be updated anymore
// can handle the int/float only mode + scientific mode too,
// no need to separate them
pub const NUM_CHARS_ACCEPTED_IN_STATES: [&[char]; 7] = [
    /* START            : 0 */  &[],  // no accepted char
    /* NUM_NEG          : 1 */  &['-'],
    /* NUM_DIG19        : 2 */  &[     '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* NUM_DIG09        : 3 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* NUM_DOT          : 4 */  &['.'],
    /* NUM_DIG_AFTER_DOT: 5 */  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    /* NUM_ZERO         : 6 */  &['0'],

    ///////////////////// Scientific number representation ////////////////////
];


// id -> human readable name
// can handle the int/float only mode + scientific mode too,
// no need to separate them
pub const NUM_NAMES_OF_STATES: [&str; 7] = [
    /* START            : 0 */ "START",
    /* NUM_NEG          : 2 */ "NUM_NEG",
    /* NUM_DIG19        : 3 */ "NUM_DIG19",
    /* NUM_DIG09        : 4 */ "NUM_DIG09",
    /* NUM_DOT          : 5 */ "NUM_DOT",
    /* NUM_DIG_AFTER_DOT: 6 */ "DIG_AFTER_DOT",
    /* NUM_ZERO         : 7 */ "NUM_ZERO",

    ///////////////////// Scientific number representation ////////////////////
];
