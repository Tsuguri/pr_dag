use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Operation: Debug {
    fn evaluate(&self) -> f32;
}

pub fn add(left: Rc<dyn Operation>, right: Rc<dyn Operation>) -> Rc<dyn Operation> {
    Rc::new(AddOp { left, right })
}

pub fn leaf(value: f32) -> Rc<dyn Operation> {
    Rc::new(Leaf { value: value })
}

// computing hash of subtrees is more expensive than computing value of the subtree but I though it
// would be ok as a training/test exercise. But honestly it doesn't make a lot of sense. DAG was
// mentioned so I transformed structure from tree to DAG (Box->Rc) and introduced explicit cache
// node that will compute inner structure only once.
// It's impossible to edit this data structure so it's not possible to introduce cycles.
pub fn cache(inner: Rc<dyn Operation>) -> Rc<dyn Operation> {
    Rc::new(CacheOp {
        inner: inner,
        value: None.into(),
    })
}

#[derive(Debug)]
pub struct AddOp {
    left: Rc<dyn Operation>,
    right: Rc<dyn Operation>,
}

impl Operation for AddOp {
    fn evaluate(&self) -> f32 {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        left + right
    }
}

#[derive(Debug)]
pub struct Leaf {
    pub value: f32,
}

impl Operation for Leaf {
    fn evaluate(&self) -> f32 {
        self.value
    }
}

#[derive(Debug)]
pub struct CacheOp {
    inner: Rc<dyn Operation>,
    value: RefCell<Option<f32>>,
}

impl Operation for CacheOp {
    fn evaluate(&self) -> f32 {
        let value = match *self.value.borrow() {
            Some(x) => x,
            None => {
                let val = self.inner.evaluate();
                *self.value.borrow_mut() = Some(val);
                val
            }
        };
        value
    }
}
