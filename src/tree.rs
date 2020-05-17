trait Node {
    fn eval(&self) -> f64;
}

struct NumberNode { pub number: f64 }
struct BinaryNode<T: Node, U: Node> {
    left: T,
    right: U,
    op: fn(f64, f64) -> f64,
}

impl Node for NumberNode {
    fn eval(&self) -> f64 {
        self.number
    }
}

impl Node for BinaryNode<NumberNode, NumberNode> {
    fn eval(&self) -> f64 {
        (&self.op)(self.left.eval(), self.right.eval())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_nodes_eval_correctly() {
        let tree = BinaryNode {
            left: NumberNode { number: 10.0 },
            right: NumberNode { number: 12.0 },
            op: |x, y| x * y,
        };
        assert_eq!(tree.eval(), 120.0);
    }
}
