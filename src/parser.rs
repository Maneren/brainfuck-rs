#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    CellInc, CellDec,
    PtrInc, PtrDec,
    Print, Read,
    BlkPsh, BlkPop,
    Invalid
}

impl Op {
    fn is_valid(&self) -> bool {
        self != &Op::Invalid
    }
}

impl From<char> for Op {
    fn from(ch: char) -> Self {
        match ch {
            '+' => Op::CellInc,
            '-' => Op::CellDec,
            '>' => Op::PtrInc,
            '<' => Op::PtrDec,
            '.' => Op::Print,
            ',' => Op::Read,
            '[' => Op::BlkPsh,
            ']' => Op::BlkPop,
            _ => Op::Invalid,
        }
    }
}

pub fn parse(string: &str) -> Vec<Op> {
    string
        .chars()
        .map(Op::from)
        .filter(Op::is_valid)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_simple() {
        let program = "+[------->++<]>--.+++.---.";
        let parsed = super::parse(program);

        println!("{:?}", parsed);
    }
}
