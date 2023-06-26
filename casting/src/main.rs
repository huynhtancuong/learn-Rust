fn main() {
    // We can convert between types
    let int_u8: u8 = 5;
    let int2_u8: u8 = 5;
    let int3: u32 = (int_u8 as u32) + (int2_u8 as u32);
    let int4: u32 = (int_u8 + int2_u8).into();
}
