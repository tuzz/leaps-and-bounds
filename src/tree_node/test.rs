use super::*;

type Subject<T> = TreeNode<T>;

mod root {
    use super::*;

    #[test]
    fn it_builds_a_root_node() {
        let root = Subject::root("root");

        assert_eq!(root.element, "root");
        assert_eq!(root.depth, 0);
    }
}

mod child {
    use super::*;

    #[test]
    fn it_builds_a_child_node() {
        let root = Subject::root("root");
        let child = root.child("child");

        assert_eq!(child.element, "child");
        assert_eq!(child.depth, 1);
    }
}
