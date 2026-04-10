use dyn_hash::DynHash;
use ordered_float::OrderedFloat;
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

impl EvaluationCache {
    pub fn create() -> Self {
        Self {
            entries: HashMap::new(),
        }
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
    pub value: OrderedFloat<f32>,
}

impl Operation for Leaf {
    fn evaluate(&self, cache: &mut EvaluationCache) -> f32 {
        *self.value
    }
}

pub fn add(left: Box<dyn Operation>, right: Box<dyn Operation>) -> Box<dyn Operation> {
    Box::new(AddOp { left, right })
}

pub fn leaf(value: f32) -> Box<dyn Operation> {
    Box::new(Leaf {
        value: value.into(),
    })
}

pub fn eval(graph: Box<dyn Operation>) -> f32 {
    let mut cache = EvaluationCache::create();

    graph.evaluate(&mut cache)
}

#[cfg(test)]
mod tests {

    use super::{Operation, add, eval, leaf};
    #[test]
    fn basic_add() {
        let operation = add(leaf(10.0), leaf(12.0));

        assert_eq!(eval(operation), 22.0);
    }

    #[test]
    fn custom_op() {
        // WARNING: this operation on floats doesn't make any sense!
        #[derive(Debug, Hash)]
        struct XOROp {
            left: Box<dyn Operation>,
            right: Box<dyn Operation>,
        }

        impl Operation for XOROp {
            fn evaluate(&self, cache: &mut crate::EvaluationCache) -> f32 {
                let left = cache.get_or_evaluate(&*self.left);
                let right = cache.get_or_evaluate(&*self.right);

                return f32::from_bits(left.to_bits() ^ right.to_bits());
            }
        }

        let operation = Box::new(XOROp {
            left: add(leaf(1.0), leaf(3.0)),
            right: leaf(0.12),
        });

        let result = eval(operation);
    }
}
