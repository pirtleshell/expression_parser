use std::f64::NAN;

type F64op = fn(f64, f64) -> f64;
pub struct Node {
    value: f64,
    left: Option::<Box<Node>>,
    right: Option::<Box<Node>>,
    op: Option<F64op>,
}

impl Node {
    pub fn number(value: f64) -> Node {
        Node {
            value,
            left: None,
            right: None,
            op: None,
        }
    }

    pub fn operation(op: F64op, left: Node, right: Node, ) -> Node {
        Node {
            value: NAN,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            op: Some(op),
        }
    }

    pub fn eval(&self) -> f64 {
        match self.op {
            Some(func) => func(
                self.left.as_ref().unwrap().eval(),
                self.right.as_ref().unwrap().eval()
            ),
            None => self.value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_nodes_eval_correctly() {
        // tree for 10 * (5 + 7)
        let ten = Node::number(10.0);
        let five = Node::number(5.0);
        let seven = Node::number(7.0);

        let tree = Node::operation(
            |x, y| x * y,
            ten,
            Node::operation(|x, y| x + y, five, seven)
        );

        assert_eq!(tree.eval(), 120.0);
    }
}
