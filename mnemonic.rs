#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mnemonic {
}

impl Mnemonic {
    pub fn parse(val: &str) -> Result<Mnemonic, ()> {
        match val {
            _ => Err(())
        }
    }
}
