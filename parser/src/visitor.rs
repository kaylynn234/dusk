use std::ops::Deref;

/// Represents a type that can "visit" nodes of type `Node`.
pub trait Visitor<Node> {
    fn visit<F>(&self, op: &mut F)
    where
        F: FnMut(&Node);

    fn visit_root<T, F>(root: &T, mut op: F)
    where
        T: Visitor<Node>,
        F: FnMut(&Node),
    {
        root.visit(&mut op);
    }
}

impl<Node> Visitor<Node> for Vec<Node> {
    fn visit<F>(&self, op: &mut F)
    where
        F: FnMut(&Node),
    {
        for node in self.iter() {
            op(node)
        }
    }
}

impl<Node, T> Visitor<Node> for Option<T>
where
    T: Visitor<Node>,
{
    fn visit<F>(&self, op: &mut F)
    where
        F: FnMut(&Node),
    {
        if let Some(visitor) = self {
            visitor.visit(op);
        }
    }
}

impl<Node, T> Visitor<Node> for Box<T>
where
    T: Visitor<Node>,
{
    fn visit<F>(&self, op: &mut F)
    where
        F: FnMut(&Node),
    {
        self.deref().visit(op)
    }
}
