use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataTypes {
    String(String),
    I8(i8),
    I16(i16),
    I32(i32),
    Float(f32),
    Doble(f64),
    Operational(usize),
    None,
}

impl DataTypes {
    pub fn convert_to(&self, target_type: &str) -> Result<DataTypes, String> {
        // Extrai o valor interno atual como uma String (ex: "42" ou "3.14")
        let valor_str = match self {
            DataTypes::String(v) => v.clone(),
            DataTypes::I8(v) => v.to_string(),
            DataTypes::I16(v) => v.to_string(),
            DataTypes::I32(v) => v.to_string(),
            DataTypes::Float(v) => v.to_string(),
            DataTypes::Doble(v) => v.to_string(),
            DataTypes::Operational(v) => v.to_string(),
            DataTypes::None => "None".to_string(),
        };

        // Tenta dar parse no formato do tipo de destino
        match target_type {
            "0" => Ok(DataTypes::String(valor_str)),
            "1" => valor_str
                .parse()
                .map(DataTypes::I8)
                .map_err(|_| "Erro i8".into()),
            "2" => valor_str
                .parse()
                .map(DataTypes::I16)
                .map_err(|_| "Erro i16".into()),
            "3" => valor_str
                .parse()
                .map(DataTypes::I32)
                .map_err(|_| "Erro i32".into()),
            "4" => valor_str
                .parse()
                .map(DataTypes::Float)
                .map_err(|_| "Erro f32".into()),
            "5" => valor_str
                .parse()
                .map(DataTypes::Doble)
                .map_err(|_| "Erro f64".into()),
            "6" => valor_str
                .parse()
                .map(DataTypes::Operational)
                .map_err(|_| "Erro usize".into()),
            _ => Err("Tipo de destino inválido".into()),
        }
    }
}

// Apenas um placeholder para o seu enum compilá-lo
#[derive(Debug, Clone)]
pub struct Memory {
    pub data: Vec<HashMap<String, DataTypes>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![HashMap::new()],
        }
    }
    fn create_scope(&mut self) {
        self.data.push(HashMap::new());
    }
    fn remove_scope(&mut self) {
        if self.data.len() > 1 {
            self.data.pop();
        }
    }

    pub fn new_value(&mut self, name: &str) {
        if let Some(current_scope) = self.data.last_mut() {
            current_scope.insert(name.to_string(), DataTypes::None);
        } else {
            panic!(
                "Nenhum escopo ativo encontrado para criar a variável '{}'!",
                name
            );
        }
    }

    pub fn edit_value(&mut self, name: &str, value: DataTypes) -> Result<(), String> {
        for s in self.data.iter_mut().rev() {
            if let Some(v) = s.get_mut(name) {
                *v = value;
                return Ok(());
            }
        }
        Err(format!(
            "Erro de Escopo: Tentativa de alterar a variável '{}', mas ela não foi declarada.",
            name
        ))
    }

    pub fn get_value(&self, name: &str) -> Option<&DataTypes> {
        for s in self.data.iter().rev() {
            if let Some(v) = s.get(name) {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_value(&mut self, name: &str) -> Result<(), String> {
        for s in self.data.iter_mut().rev() {
            if s.remove(name).is_some() {
                return Ok(());
            }
        }

        Err(format!(
            "Erro de Escopo: Tentativa de remover a variável '{}', mas ela não foi encontrada.",
            name
        ))
    }
}
