use std::fmt;
#[derive(Debug)]
pub struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}
impl fmt::Display for ConditionCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "z: {}, s: {}, p: {}, cy: {}, ac: {}, pad: {},",
               self.z, self.s, self.p, self.cy, self.ac, self.pad)
    }
}
impl ConditionCodes {
    pub fn new(z: u8, s: u8, p: u8, cy: u8, ac: u8, pad: u8) -> ConditionCodes {
        ConditionCodes {
            z,
            s,
            p,
            cy,
            ac,
            pad,
        }
    }
}

pub struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: u8,
    cc: ConditionCodes,
    int_enable: u8,
}