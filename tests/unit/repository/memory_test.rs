//! Unit tests for MemoryRepository

use bpmn_engine::repository::{Repository, MemoryRepository};
use bpmn_engine::{ProcessInstance, ProcessDefinition};
use test_log::test;
use tokio_test;
use std::sync::Arc;

#[tokio::test]
async fn test_memory_repository_save_get() {
    let repo = MemoryRepository::new();
    let definition = Arc::new(ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap());
    
    let instance = Arc::new(ProcessInstance::new(definition, "instance1".to_string()));
    repo.save(instance.clone()).await.unwrap();
    
    let retrieved = repo.get("instance1").await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id(), "instance1");
}

#[tokio::test]
async fn test_memory_repository_delete() {
    let repo = MemoryRepository::new();
    let definition = Arc::new(ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap());
    
    let instance = Arc::new(ProcessInstance::new(definition, "instance1".to_string()));
    repo.save(instance.clone()).await.unwrap();
    
    repo.delete("instance1").await.unwrap();
    
    let retrieved = repo.get("instance1").await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_memory_repository_list_ids() {
    let repo = MemoryRepository::new();
    let definition = Arc::new(ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap());
    
    let instance1 = Arc::new(ProcessInstance::new(definition.clone(), "instance1".to_string()));
    let instance2 = Arc::new(ProcessInstance::new(definition.clone(), "instance2".to_string()));
    
    repo.save(instance1).await.unwrap();
    repo.save(instance2).await.unwrap();
    
    let ids = repo.list_ids().await.unwrap();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"instance1".to_string()));
    assert!(ids.contains(&"instance2".to_string()));
}

#[tokio::test]
async fn test_memory_repository_exists() {
    let repo = MemoryRepository::new();
    let definition = Arc::new(ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap());
    
    let instance = Arc::new(ProcessInstance::new(definition, "instance1".to_string()));
    repo.save(instance).await.unwrap();
    
    assert!(repo.exists("instance1").await.unwrap());
    assert!(!repo.exists("nonexistent").await.unwrap());
}

