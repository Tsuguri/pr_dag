mod operations;

use std::rc::Rc;

pub use operations::*;

// This node is exposed through FFI to language-specific bindings.
// We cannot directly expose Operation pointers because 1. Rc, 2. fat pointer
pub struct OperationNode {
    inner: Rc<dyn Operation>,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_leaf(value: f32) -> *mut OperationNode {
    let node = Box::new(OperationNode { inner: leaf(value) });
    Box::into_raw(node)
}

// Does NOT take ownership of parameter node. Allows reuse of DAG node in multiple places
#[unsafe(no_mangle)]
pub extern "C" fn clone_operation(node: *mut OperationNode) -> *mut OperationNode {
    let copy = Box::new(OperationNode {
        inner: unsafe { (*node).inner.clone() },
    });
    //
    Box::into_raw(copy)
}

// Takes ownership of both arguments.
// Similar functions should be created for all library-provided operations
// If there are a lot N-argument nodes, macro should be used
#[unsafe(no_mangle)]
pub extern "C" fn create_add_node(
    left: *mut OperationNode,
    right: *mut OperationNode,
) -> *mut OperationNode {
    let left = unsafe { Box::from_raw(left) };
    let right = unsafe { Box::from_raw(right) };
    let op = Box::new(OperationNode {
        inner: add(left.inner, right.inner),
    });
    Box::into_raw(op)
}

// All Nodes that were created and still owned by host-language should be destroyed after usage.
#[unsafe(no_mangle)]
pub extern "C" fn destroy_node(node: *mut OperationNode) {
    let _ = unsafe { Box::from_raw(node) };
}

// Cache node. Honestly I would cache any non-trivial node that will be used in multiple places (later cloned
// with clone_operation) OR is host-language operation.
#[unsafe(no_mangle)]
pub extern "C" fn cache_node(node: *mut OperationNode) -> *mut OperationNode {
    let node = unsafe { Box::from_raw(node) };
    let op = Box::new(OperationNode {
        inner: cache(node.inner),
    });
    Box::into_raw(op)
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use super::{Operation, add, leaf};
    #[test]
    fn basic_add() {
        let operation = add(leaf(10.0), leaf(12.0));

        assert_eq!(operation.evaluate(), 22.0);
    }

    #[test]
    fn custom_op() {
        // WARNING: this operation on floats doesn't make any sense!
        #[derive(Debug)]
        struct XOROp {
            left: Rc<dyn Operation>,
            right: Rc<dyn Operation>,
        }

        impl Operation for XOROp {
            fn evaluate(&self) -> f32 {
                let left = self.left.evaluate();
                let right = self.right.evaluate();

                return f32::from_bits(left.to_bits() ^ right.to_bits());
            }
        }

        let operation = Rc::new(XOROp {
            left: add(leaf(1.0), leaf(3.0)),
            right: leaf(0.12),
        });

        let _result = operation.evaluate();
    }
}
