//! Plugin System
//!
//! Plugin system for extending BPMN engine functionality.

use crate::activity::{Activity, ActivityFactory};
use crate::model::ProcessElement;
use std::collections::HashMap;
use std::sync::Arc;

/// Plugin Trait
///
/// Trait for BPMN engine plugins.
pub trait Plugin: Send + Sync + std::fmt::Debug {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str;

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), PluginError>;

    /// Get custom activity factories provided by this plugin
    fn get_activity_factories(&self) -> HashMap<String, Arc<dyn ActivityFactory>>;

    /// Get custom activity types provided by this plugin
    fn get_custom_activity_types(&self) -> Vec<String>;
}

/// Plugin Error
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
    #[error("Plugin not found: {0}")]
    NotFound(String),
}

/// Plugin Registry
///
/// Registry for managing BPMN engine plugins.
pub struct PluginRegistry {
    plugins: Arc<tokio::sync::RwLock<HashMap<String, Arc<dyn Plugin>>>>,
    activity_factories: Arc<tokio::sync::RwLock<HashMap<String, Arc<dyn ActivityFactory>>>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            activity_factories: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin
    pub async fn register(&self, plugin: Arc<dyn Plugin>) -> Result<(), PluginError> {
        let plugin_name = plugin.name().to_string();
        
        // Initialize plugin
        // Note: We need mutable access, but Arc doesn't allow that
        // In a real implementation, we'd need to handle this differently
        // For now, we'll assume plugins are initialized before registration

        // Register plugin
        {
            let mut plugins = self.plugins.write().await;
            plugins.insert(plugin_name.clone(), plugin.clone());
        }

        // Register activity factories from plugin
        {
            let mut factories = self.activity_factories.write().await;
            for (activity_type, factory) in plugin.get_activity_factories() {
                factories.insert(activity_type, factory);
            }
        }

        Ok(())
    }

    /// Get activity factory for a custom activity type
    pub async fn get_activity_factory(&self, activity_type: &str) -> Option<Arc<dyn ActivityFactory>> {
        let factories = self.activity_factories.read().await;
        factories.get(activity_type).cloned()
    }

    /// Get all registered plugins
    pub async fn get_plugins(&self) -> Vec<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

