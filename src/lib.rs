use dyn_hash::DynHash;
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct EvaluationCache {
    entries: HashMap<u64, f32>, // TODO: replace u32 with actual hash
}

impl EvaluationCache {
    pub fn get_or_evaluate(&mut self, node: &dyn Operation) -> f32 {
        let mut hasher = DefaultHasher::new();
        node.hash(&mut hasher);
        let hsh = hasher.finish();
        let result = if let Some(val) = self.entries.get(&hsh) {
            *val
        } else {
            let value = node.evaluate(self);
            self.entries.insert(hsh, value);
            value
        };
        result
    }
}

pub trait Operation: Debug + DynHash {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32;
}

dyn_hash::hash_trait_object!(Operation);

#[derive(Debug, Hash)]
pub struct AddOp {
    left: Box<dyn Operation>,
    right: Box<dyn Operation>,
}

impl Operation for AddOp {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32 {
        let left = cache.get_or_evaluate(&*self.left);
        let right = cache.get_or_evaluate(&*self.right);
        left + right
    }
}

#[derive(Debug, Hash)]
pub struct Leaf {
    pub value: f32,
}

impl Operation for Leaf {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32 {}
}

pub fn add(left: Box<dyn Operation>, right: Box<dyn Operation>) -> Box<dyn Operation> {
    Box::new(AddOp { left, right })
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_add() {}
}
