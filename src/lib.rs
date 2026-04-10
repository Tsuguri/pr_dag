mod operations;

pub use operations::*;

// #[no_mangle]
// pub extern "C" fn create_leaf(value: f32) -> *mut dyn Operation {
//     std::ptr::null_mut()
// }

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
