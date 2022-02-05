use self::ParseMoneyError::*;

#[derive(Debug, PartialEq)]
pub enum ParseMoneyError {
    SymbolErr,
    NoStringErr,
    TwoPointsErr,
    NonDigitErr(char),
    TooFarErr,
}


pub fn parse_sym_money(s: &str, sym: char, dpoint: usize) -> Result<i32, ParseMoneyErr> {
    let (c, v) = parse_sym_money(s, dpoint)?;
    if c != sym {
        return Err(ParseMoneyError::SymbolErr);
    }
    Ok(v)
}


pub fn parse_money(s: &str, dpoint: usize) -> Result<(char, i32), ParseMoneyError> {
    let mut it = s.trim().chars();
    let mut neg = false;

    let mut r_sym = it.next().ok_ir(NoStringErr)?;
    if '-'
}

