use std::fs;

struct Symbol {
    x: i32,
    y: i32,
}

struct PartNumber {
    x: i32,
    y: i32, 
    len: i32,
    num: i32,
}

pub fn solve(input: &str) -> i32 {
    get_part_numbers(input).iter().sum()
}

fn get_part_numbers(input: &str) -> Vec<i32> {
    let plan = fs::read_to_string(input)
    .unwrap();
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<PartNumber> = vec![];
    let mut len = 0;
    let mut tmp_coords: Symbol = Symbol { x: 0, y: 0 };
    let mut tmp_num = 0;

    for (py, l) in plan.lines().enumerate() {
        for (px, c) in l.chars().enumerate() {
            if c.is_digit(10) {
                if len == 0 {
                    tmp_coords = Symbol{x: px as i32, y: py as i32};
                }
                len += 1;
                tmp_num *= 10;
                tmp_num += c.to_digit(10).unwrap() as i32;
            }
            else if c != '.' { // we have a symbol!
                symbols.push(Symbol { x: px as i32, y: py as i32});
                if len != 0 { // reset part number if one was being built
                    numbers.push(PartNumber { x: tmp_coords.x, y: tmp_coords.y, len: len, num: tmp_num });
                    len = 0;
                    tmp_num = 0;
                }
            } else {
                if len != 0 { // reset part number if one was being built
                    numbers.push(PartNumber { x: tmp_coords.x, y: tmp_coords.y, len: len, num: tmp_num });
                    len = 0;
                    tmp_num = 0;
                }
            }
        }
    }

    let mut parts: Vec<i32> = vec![];

    for num in numbers {
        for sym in &symbols {
            if symbol_near_number(&sym, &num) {
                parts.push(num.num);
                break;
            }
        }
    }
    parts
}

fn symbol_near_number(symbol: &Symbol, number: &PartNumber) -> bool { // view number as a rectangle, check if symbol is within the rectangle. Should do diagonals.
    (symbol.x >= number.x - 1 && symbol.x <= number.x + number.len) && (symbol.y >= number.y - 1 && symbol.y <= number.y + 1)
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn full_test() {
        assert_eq!(solve("sample.txt"), 4361);
    }
}