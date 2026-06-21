

// token kinds
pub const TOKEN_KIND_NUM: u8 = 0;
pub const TOKEN_KIND_STR: u8 = 1;
pub const TOKEN_KIND_BOOL: u8 = 2;
pub const TOKEN_KIND_NULL: u8 = 3;
pub const TOKEN_KIND_ARR_COMMA: u8 = 10;
pub const TOKEN_KIND_ARR_COLON: u8 = 11;
pub const TOKEN_KIND_ARR_OPEN: u8 = 20;
pub const TOKEN_KIND_ARR_CLOSE: u8 = 21;
pub const TOKEN_KIND_OBJ_OPEN: u8 = 30;
pub const TOKEN_KIND_OBJ_CLOSE: u8 = 31;

#[derive(Debug)]
pub struct Token {
    start: usize,
    end: usize,
    kind: u8,
}
