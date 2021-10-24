use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,

    COUNT
}

impl Register {
    pub fn is_register(name: &String) -> bool {
        
        let architecture_modifiers = vec![
            "R", "E", ""
        ];
        
        let reg_names = vec![
            "AX", "BX", "CX", "DX",
            "SI", "DI", "SP", "BP"
        ];
        
        let mut registers: Vec<String> = Vec::with_capacity(
            architecture_modifiers.len() * reg_names.len()
        );

        for reg in reg_names {
            for &modifier in &architecture_modifiers {
                registers.push(
                    format!("{}{}", modifier, reg)
                );
            }
        }

        registers.contains(&name.to_uppercase())
    }

    pub fn from_string(name: &String) -> Option<Register> {
        let architecture_modifiers = vec![
            "R", "E", ""
        ];
        
        let reg_names = vec![
            "AX", "BX", "CX", "DX",
            "SI", "DI", "SP", "BP"
        ];
        
        let mut registers: Vec<String> = Vec::with_capacity(
            architecture_modifiers.len() * reg_names.len()
        );

        for reg in reg_names {
            for &modifier in &architecture_modifiers {
                registers.push(
                    format!("{}{}", modifier, reg)
                );
            }
        }

        let primitive = name.to_uppercase();

        let i = registers.iter().position(|ins| &primitive == ins);
        match i {
            Some(idx) => FromPrimitive::from_usize(idx / architecture_modifiers.len()),
            None => None,
        }
    }
}