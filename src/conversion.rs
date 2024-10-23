//! Conversion tools for converting various formats and structures

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
pub const fn convert_1bpp_5bpp<const S: usize>(data: [u8; S], fg: u8, bg: u8) -> [u32; S] {
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

#[cfg(test)]
mod tests {
    use crate::conversion::convert_1bpp_5bpp;

    #[test]
    pub fn validate_1bpp_5bpp() {
        assert_eq!(
            convert_1bpp_5bpp::<6>(
                [
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
}
