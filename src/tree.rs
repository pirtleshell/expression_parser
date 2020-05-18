pub enum Node {
    Leaf(Leaf),
    Binary(BinaryNode),
    Unary(UnaryNode),
}

pub trait Evaluable {
    fn eval(&self) -> f64;
}

impl Evaluable for Node {
    fn eval(&self) -> f64 {
        match self {
            Node::Leaf(node) => node.eval(),
            Node::Binary(node) => node.eval(),
            Node::Unary(node) => node.eval(),
        }
    }
}

// Leaf Nodes
pub struct Leaf(f64);

impl Evaluable for Leaf {
    fn eval(&self) -> f64 { self.0 }
}

impl Leaf {
    pub fn new(value: f64) -> Node {
        Node::Leaf(Leaf(value))
    }
}


// Binary Operation Nodes
type BinOp = fn(f64, f64) -> f64;
pub struct BinaryNode {
    left: Box<Node>,
    right: Box<Node>,
    op: BinOp,
}

impl Evaluable for BinaryNode {
    fn eval(&self) -> f64 {
        let op = self.op;
        op(
            self.left.as_ref().eval(),
            self.right.as_ref().eval(),
        )
    }
}

impl BinaryNode {
    pub fn new(left: Node, right: Node, op: BinOp) -> Node {
        Node::Binary(BinaryNode {
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }
}


// Unary Operation Nodes
type UnOp = fn(f64) -> f64;
pub struct UnaryNode {
    child: Box<Node>,
    op: UnOp,
}

impl Evaluable for UnaryNode {
    fn eval(&self) -> f64 {
        let op = self.op;
        op(self.child.as_ref().eval())
    }
}

impl UnaryNode {
    pub fn new(child: Node, op: UnOp) -> Node {
        Node::Unary(UnaryNode {
            child: Box::new(child),
            op,
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_nodes_eval_correctly() {
        // tree for 10 - 5 + 7
        let ten = Leaf::new(10.0);
        let five = Leaf::new(5.0);
        let seven = Leaf::new(7.0);

        let tree = BinaryNode::new(
            ten,
            BinaryNode::new(five, seven, |x, y| x + y),
            |x, y| x - y
        );

        assert_eq!(tree.eval(), -2.0);
    }
}
