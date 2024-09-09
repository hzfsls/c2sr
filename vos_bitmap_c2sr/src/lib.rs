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
}