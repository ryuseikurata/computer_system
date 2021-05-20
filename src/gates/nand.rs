/// 0: false
/// 1: true
/// 基礎の関数
/// これを使用して他の論理関数を作成する
pub fn calc(a: bool, b: bool) -> bool {
    !(a && b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(calc(true, true), false);
        assert_eq!(calc(true, false), true);
        assert_eq!(calc(false, true), true);
        assert_eq!(calc(false, false), true);
    }
}
