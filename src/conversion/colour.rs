//! Functions to help convert colour and graphics data

/// Convert an 8-pixel array to a 16-colour paletted 5-bit rgb 8-pixel array
///
/// This is useful for games, for example to turn 8x8 pixel tiles into 8x8 colour pixel tiles.
///
/// The Gameboy Advanced console uses this method to display data, so you can store fonts in binary
/// format, and then convert them to foregroun and background colour formats.
///
/// data - array of 8-pixel rows in binary
/// fg - foreground colour (0-15)
/// bg - background colour (0-15)
pub const fn convert_1bpp_5bpp<const S: usize>(data: &[u8; S], fg: u8, bg: u8) -> [u32; S] {
    assert!(
        fg < 16,
        "Foreground can only be one of 16 colours in paletted rgb"
    );
    assert!(
        bg < 16,
        "Background can only be one of 16 colours in paletted rgb"
    );
    let mut ret = [0u32; S];
    let mut i = 0;
    while i < S {
        let mut bit = 0;
        while bit < 8 {
            let input_mask = 1 << bit;
            let output_col = match (input_mask & data[i]) == 0 {
                true => (bg as u32) << (bit * 4),
                false => (fg as u32) << (bit * 4),
            };
            ret[i] |= output_col;
            bit += 1;
        }
        i += 1;
    }
    ret
}

/// Flip a provided array
pub const fn vflip_1bpp_const<const S: usize>(mut data: [u8; S]) -> [u8; S] {
    let mut i = 0;
    while i < data.len() {
        let flipped = vflip_1bpp_single(data[i]);
        data[i] = flipped;
        i += 1;
    }
    data
}

/// Flip 1-bit, 8 pixel value
/// So 0b1010_0000 becomes 0b0000_0101
pub const fn vflip_1bpp_single(data: u8) -> u8 {
    let mut flipped = 0u8;
    let mut bit = 0u8;
    while bit < 8 {
        let shift = 7 - bit;
        let source_mask = 1u8 << shift;
        let val = data & source_mask;
        // Make the value '1' or '0'
        let val = val >> shift;
        // Make the value correct new position
        // We do it this way, because otherwise, there is overflow/change of shift direction
        let val = val << bit;
        flipped |= val;
        bit += 1;
    }
    flipped
}

/// Flip 1-bit, 8-pixel array
pub fn vflip_1bpp_mut(data: &mut [u8]) {
    for i in data.iter_mut() {
        *i = vflip_1bpp_single(*i);
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::colour::{convert_1bpp_5bpp, vflip_1bpp_const};
    use std::format;
    use std::string::String;
    use std::vec::Vec;

    #[test]
    pub fn validate_1bpp_5bpp() {
        assert_eq!(
            convert_1bpp_5bpp::<6>(
                &[
                    0b0000_0000,
                    0b0000_0001,
                    0b0000_1000,
                    0b0001_1000,
                    0b1000_0000,
                    0b1111_1111,
                ],
                0x3,
                0xa
            ),
            [
                0xaaaa_aaaa,
                0xaaaa_aaa3,
                0xaaaa_3aaa,
                0xaaa3_3aaa,
                0x3aaa_aaaa,
                0x3333_3333
            ]
        );
    }

    #[test]
    pub fn test_vflip_1bpp() {
        let mut data: [u8; 4] = [0b1111_0000, 0b0000_1111, 0b1010_1010, 0b0101_0101];
        let data = vflip_1bpp_const(data);
        assert_eq!(
            data,
            [0b0000_1111, 0b1111_0000, 0b0101_0101, 0b1010_1010,],
            "Data was {:?}",
            data.iter()
                .map(|b| format!("{:08b}", b))
                .collect::<Vec<String>>()
        );
    }
}
