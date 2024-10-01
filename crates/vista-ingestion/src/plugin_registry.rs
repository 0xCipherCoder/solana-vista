use std::collections::HashMap;
use libloading::{Library, Symbol};
use crate::traits::RpcProvider;
use crate::error::IngestionError;

pub struct ProviderPluginRegistry {
    providers: HashMap<String, Box<dyn RpcProvider>>,
    libraries: Vec<Library>,
}

impl ProviderPluginRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            libraries: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn RpcProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), IngestionError> {
        unsafe {
            let lib = Library::new(path).map_err(|e| IngestionError::PluginLoadError(e.to_string()))?;
            self.libraries.push(lib);
            let lib = self.libraries.last().unwrap();
            let constructor: Symbol<fn() -> Box<dyn RpcProvider>> = lib.get(b"_create_provider")
                .map_err(|e| IngestionError::PluginLoadError(e.to_string()))?;
            let provider = constructor();
            self.register_provider(provider);
        }
        Ok(())
    }

    pub fn get_provider(&self, name: &str) -> Option<&Box<dyn RpcProvider>> {
        self.providers.get(name)
    }

    pub fn get_providers(&self) -> Vec<&Box<dyn RpcProvider>> {
        self.providers.values().collect()
    }
}