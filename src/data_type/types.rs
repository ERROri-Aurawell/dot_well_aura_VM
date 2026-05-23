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
