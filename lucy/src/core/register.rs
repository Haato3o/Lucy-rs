#[derive(Clone, Copy, Debug)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
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
}