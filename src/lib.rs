/// GenOffice Sheets: named range registry and column statistics engine.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NamedRange {
    pub name: String,
    pub sheet: String,
    pub rows: (u32, u32),
    pub cols: (u32, u32),
}

impl NamedRange {
    pub fn new(name: &str, sheet: &str, r0: u32, r1: u32, c0: u32, c1: u32) -> Self {
        Self {
            name: name.into(),
            sheet: sheet.into(),
            rows: (r0, r1),
            cols: (c0, c1),
        }
    }

    pub fn cell_count(&self) -> u64 {
        (self.rows.1 - self.rows.0 + 1) as u64 * (self.cols.1 - self.cols.0 + 1) as u64
    }

    pub fn a1_notation(&self) -> String {
        fn col_name(c: u32) -> String {
            let mut s = String::new();
            let mut n = c;
            loop {
                s.insert(0, (b'A' + (n % 26) as u8) as char);
                if n < 26 {
                    break;
                }
                n = n / 26 - 1;
            }
            s
        }
        format!(
            "{}{}:{}{}",
            col_name(self.cols.0),
            self.rows.0 + 1,
            col_name(self.cols.1),
            self.rows.1 + 1
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_count_aaa() {
        let r = NamedRange::new("Sales", "Sheet1", 0, 9, 0, 3);
        assert_eq!(r.cell_count(), 40);
    }

    #[test]
    fn test_a1_notation_simple_aaa() {
        let r = NamedRange::new("Data", "Sheet1", 0, 4, 0, 1);
        assert_eq!(r.a1_notation(), "A1:B5");
    }

    #[test]
    fn test_single_cell_range_aaa() {
        let r = NamedRange::new("Cell", "Sheet1", 2, 2, 2, 2);
        assert_eq!(r.cell_count(), 1);
    }

    #[test]
    fn test_a1_col_c_aaa() {
        let r = NamedRange::new("R", "S", 0, 0, 2, 2);
        assert!(r.a1_notation().starts_with("C1"));
    }
}
