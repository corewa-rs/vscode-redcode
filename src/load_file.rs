use std::{str::FromStr, vec};

pub const CORE_SIZE: usize = 8000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Mov,
    Dat,
    Jmp,
}

impl Opcode {
    pub fn to_string(self) -> String {
        use self::Opcode::*;
        match self {
            Mov => "MOV",
            Dat => "DAT",
            Jmp => "JMP",
        }.to_owned()
    }
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        use self::Opcode::*;
        match input_str.to_uppercase().as_ref() {
            "MOV" => Ok(Mov),
            "DAT" => Ok(Dat),
            "JMP" => Ok(Jmp),
            _ => Err(format!("Invalid opcode '{}'", input_str)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AddressMode {
    Immediate,
    Direct,
}

impl AddressMode {
    pub fn to_string(self) -> String {
        use self::AddressMode::*;
        match self {
            Immediate => "#",
            Direct => "",
        }.to_owned()
    }
}

impl Default for AddressMode {
    fn default() -> AddressMode {
        AddressMode::Direct
    }
}

impl FromStr for AddressMode {
    type Err = String;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        use self::AddressMode::*;
        match input_str {
            "#" => Ok(Immediate),
            "" => Ok(Direct),
            _ => Err(format!("Invalid address mode '{}'", input_str)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub value: i32,
    pub address_mode: AddressMode,
}

impl Field {
    pub fn to_string(self) -> String {
        format!("{}{}", self.address_mode.to_string(), self.value)
    }
}

impl Default for Field {
    fn default() -> Field {
        Field {
            value: 0,
            address_mode: AddressMode::Direct,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub a: Field,
    pub b: Option<Field>,
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: Opcode::Dat,
            a: Field::default(),
            b: Some(Field::default()),
        }
    }
}

impl Instruction {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}{}",
            self.opcode.to_string(),
            self.a.to_string(),
            match &self.b {
                Some(field) => format!(", {}", field.to_string()),
                None => "".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Program {
    pub instructions: vec::Vec<Instruction>,
}

impl Program {
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn set(&mut self, index: usize, value: Instruction) {
        self.instructions[index] = value;
    }

    pub fn dump(&self) -> String {
        self.instructions
            .iter()
            .filter(|&instruction| *instruction != Instruction::default())
            .fold(String::new(), |result, instruction| {
                result + &instruction.to_string() + "\n"
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn opcode_from_str() {
        assert_eq!(Opcode::from_str("MOV").unwrap(), Opcode::Mov);
        assert_eq!(Opcode::from_str("MoV").unwrap(), Opcode::Mov);
        assert_eq!(Opcode::from_str("mov").unwrap(), Opcode::Mov);

        assert_eq!(Opcode::from_str("dat").unwrap(), Opcode::Dat);
        assert_eq!(Opcode::from_str("jmp").unwrap(), Opcode::Jmp);
    }

    #[test]
    fn address_mode_from_str() {
        assert_eq!(AddressMode::from_str("#").unwrap(), AddressMode::Immediate);
        assert_eq!(AddressMode::from_str("").unwrap(), AddressMode::Direct);
    }

}
