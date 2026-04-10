use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub struct EvaluationCache {
    entries: HashMap<u32, f32>, // TODO: replace u32 with actual hash
}

impl EvaluationCache {
    pub fn get_or_evaluate(&mut self, node: &dyn Operation) -> f32 {
        0.0
    }
}

pub trait Operation: Debug {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32;
    fn hash(&self) -> u64;
}

#[derive(Debug)]
pub struct AddOp {
    left: Box<dyn Operation>,
    right: Box<dyn Operation>,
}

impl Operation for AddOp {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32 {
        0.0f32
    }
    fn hash(&self) -> u64 {
        0
    }
}

pub fn add(left: Box<dyn Operation>, right: Box<dyn Operation>) -> Box<dyn Operation> {
    Box::new(AddOp { left, right })
}
