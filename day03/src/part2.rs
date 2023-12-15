use std::fs;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
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
    get_cog_ratios(input).iter().sum()
}

fn get_cog_ratios(input: &str) -> Vec<i32> {
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
            else if c == '*' { // we only care about * symbols
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

    let mut neighbors = 0;
    let mut ratio = 1;
    let mut cogratios: Vec<i32> = vec![];
    for sym in symbols {
        neighbors = 0;
        ratio = 1;
        for num in &numbers {
            if symbol_near_number(&sym, &num) {
                neighbors += 1;
                ratio *= num.num;
            }
        }
        if neighbors == 2 {
            cogratios.push(ratio)
        }
    }
    cogratios
}

fn symbol_near_number(symbol: &Symbol, number: &PartNumber) -> bool { // view number as a rectangle, check if symbol is within the rectangle. Should do diagonals.
    (symbol.x >= number.x - 1 && symbol.x <= number.x + number.len) && (symbol.y >= number.y - 1 && symbol.y <= number.y + 1)
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn full_test() {
        assert_eq!(solve("sample.txt"), 467835);
    }
}