use std::collections::HashMap;
use libloading::{Library, Symbol};
use crate::traits::RpcProvider;
use crate::config::Config;
use crate::traits::StoragePlugin;

pub struct RpcProviderRegistry {
    providers: HashMap<String, Box<dyn RpcProvider>>,
    libraries: Vec<Library>,
}

impl RpcProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            libraries: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn RpcProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    pub fn get_provider(&self, name: &str) -> Option<&Box<dyn RpcProvider>> {
        self.providers.get(name)
    }

    pub fn get_providers(&self) -> Vec<&Box<dyn RpcProvider>> {
        self.providers.values().collect()
    }

    pub fn load_plugins(&mut self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(plugin_dir) = &config.plugin_dir {
            for entry in std::fs::read_dir(plugin_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "so" || ext == "dll") {
                    unsafe {
                        let lib = Library::new(path)?;
                        self.libraries.push(lib);
                        let lib = self.libraries.last().unwrap();
                        let constructor: Symbol<fn() -> Box<dyn RpcProvider>> = lib.get(b"_plugin_create")?;
                        let provider = constructor();
                        self.register_provider(provider);
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct StoragePluginRegistry {
    plugins: HashMap<String, Box<dyn StoragePlugin>>,
}

impl StoragePluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn StoragePlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn StoragePlugin>> {
        self.plugins.get(name)
    }
}