#[derive(Debug)]
pub enum Op {
    CellInc, CellDec,
    PtrInc, PtrDec,
    Print, Read,
    BlkPsh, BlkPop,
}

pub fn parse(string: String) -> Vec<Op> {
    let mut ops = Vec::new();
    string
        .chars()
        .for_each(|char| {
            let op = match char {
                '+' => Some(Op::CellInc),
                '-' => Some(Op::CellDec),
                '>' => Some(Op::PtrInc),
                '<' => Some(Op::PtrDec),
                '.' => Some(Op::Print),
                ',' => Some(Op::Read),
                '[' => Some(Op::BlkPsh),
                ']' => Some(Op::BlkPop),
                _ => None,
            };

            if let Some(op) = op { ops.push(op); }
        });

    ops
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_simple() {
        let program = "+[------->++<]>--.+++.---.";
        let parsed = super::parse(program.to_owned());

        println!("{:?}", parsed);

    }
}