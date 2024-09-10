pub(crate) mod vos_bitmap;

use crate::vos_bitmap::*;


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vos_bitmapffs() {
        assert_eq!(vos_bitmapffs(0), 31);
    }

    #[test]
    fn test_vos_bitmapffb() {
        let mut puiBmp = [1, 2];
        let mut puiBmp1 = [0, 3];
        assert_eq!(vos_bitmapffb(&mut [], 1), 1);
        assert_eq!(vos_bitmapffb(&mut puiBmp, 2), 0);
        assert_eq!(vos_bitmapffb(&mut puiBmp, 0), 0);
        assert_eq!(vos_bitmapffb(&mut puiBmp1, 1), 1);
    }

    #[test]
    fn test_vos_bitmapff0b() {
        let mut puiBmp = [0, 1];
        let mut puiBmp1 = [1, 2];
        assert_eq!(vos_bitmapff0b(&mut [], 1), 1);
        assert_eq!(vos_bitmapff0b(&mut puiBmp, 0), 0);
        assert_eq!(vos_bitmapff0b(&mut puiBmp, 1), 0);
        assert_eq!(vos_bitmapff0b(&mut puiBmp1, 1), 1);
    }

    // pub fn vos_reverse_byte_bits(uc_byte: u8) -> u8 {
    //     ((((((uc_byte as u32 * 0x0802) & 0x22110) | ((uc_byte as u32 * 0x8020) & 0x88440)) * 0x10101) >>
    //         vos_bitmap_double_byte_bits!()) & vos_null_byte!()) as u8
    // }
    // generate test for vos_reverse_byte_bits

    #[test]
    fn test_vos_reverse_byte_bits() {
        assert_eq!(vos_reverse_byte_bits(0b00000000), 0b00000000);
        assert_eq!(vos_reverse_byte_bits(0b00000001), 0b10000000);
        assert_eq!(vos_reverse_byte_bits(0b00000010), 0b01000000);
        assert_eq!(vos_reverse_byte_bits(0b00000011), 0b11000000);
        assert_eq!(vos_reverse_byte_bits(0b10110011), 0b11001101);
    }
}