pub mod types {
    pub static INT_SHIFT: i32 = 1;
    pub static INT_TAG: i32 = 0b0;
    pub static BOOL_TAG: i32 = 0b1;
    pub static VAL_TRUE: i32 = 0b01;
    pub static VAL_FALSE: i32 = 0b11;

    pub fn int_to_bits(i: i32) -> i32 {
        return i << INT_SHIFT;
    }

    pub fn bool_to_bits(b: bool) -> i32 {
        if b { return VAL_TRUE }
        return VAL_FALSE
    }
}