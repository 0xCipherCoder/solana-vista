use anchor_syn::idl::{Idl, IdlType};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnchorError {
    #[error("IDL parsing error: {0}")]
    IdlParseError(String),
    #[error("Account data parsing error: {0}")]
    AccountDataParseError(String),
}

pub struct AnchorParser {
    idls: std::collections::HashMap<String, Idl>,
}

impl AnchorParser {
    pub fn new() -> Self {
        Self {
            idls: std::collections::HashMap::new(),
        }
    }

    pub fn add_idl(&mut self, program_id: &str, idl_json: &str) -> Result<(), AnchorError> {
        let idl: Idl = serde_json::from_str(idl_json)
            .map_err(|e| AnchorError::IdlParseError(e.to_string()))?;
        self.idls.insert(program_id.to_string(), idl);
        Ok(())
    }

    pub fn parse_account_data(&self, program_id: &str, account_type: &str, data: &[u8]) -> Result<Value, AnchorError> {
        let idl = self.idls.get(program_id)
            .ok_or_else(|| AnchorError::IdlParseError(format!("IDL not found for program {}", program_id)))?;

        let account = idl.accounts.iter()
            .find(|a| a.name == account_type)
            .ok_or_else(|| AnchorError::AccountDataParseError(format!("Account type {} not found in IDL", account_type)))?;

        let mut parser = borsh::BorshDeserialize::deserialize(&mut &data[8..])  // Skip the 8-byte discriminator
            .map_err(|e| AnchorError::AccountDataParseError(e.to_string()))?;

        let mut result = serde_json::Map::new();
        for field in &account.fields {
            let value = self.parse_idl_type(&field.ty, &mut parser)?;
            result.insert(field.name.clone(), value);
        }

        Ok(Value::Object(result))
    }

    fn parse_idl_type(&self, ty: &IdlType, parser: &mut borsh::maybestd::io::Cursor<&[u8]>) -> Result<Value, AnchorError> {
        match ty {
            IdlType::Bool => Ok(Value::Bool(borsh::BorshDeserialize::deserialize(parser)
                .map_err(|e| AnchorError::AccountDataParseError(e.to_string()))?)),
            IdlType::U8 => Ok(Value::Number(borsh::BorshDeserialize::deserialize(parser)
                .map_err(|e| AnchorError::AccountDataParseError(e.to_string()))?.into())),
            // Implement other types...
            _ => Err(AnchorError::AccountDataParseError(format!("Unsupported IDL type: {:?}", ty))),
        }
    }
}