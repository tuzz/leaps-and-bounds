struct TreeNode<T> {
    element: T,
    depth: usize,
}

impl<T> TreeNode<T> {
    fn root(element: T) -> Self {
        TreeNode { element, depth: 0 }
    }

    fn child(&self, element: T) -> Self {
        TreeNode { element, depth: self.depth + 1 }
    }
}

#[cfg(test)]
mod test;
