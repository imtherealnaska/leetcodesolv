// Define a binary search tree in Dafny
class BST {
    var root: Node?;

    constructor() {
        root := null;
    }

    method Insert(value: int) {
        if root == null {
            root := new Node(value);
        } else {
            root.Insert(value);
        }
    }

    method Search(value: int) returns (found: bool) {
        if root == null {
            return false;
        }
        return root.Search(value);
    }
}

// Define the structure of a node in the BST
class Node {
    var value: int;
    var left: Node?;
    var right: Node?;

    constructor(value: int) {
        this.value := value;
        left := null;
        right := null;
    }

    method Insert(newValue: int) {
        if newValue < value {
            if left == null {
                left := new Node(newValue);
            } else {
                left.Insert(newValue);
            }
        } else if newValue > value {
            if right == null {
                right := new Node(newValue);
            } else {
                right.Insert(newValue);
            }
        }
    }

    method Search(searchValue: int) returns (found: bool) {
        if searchValue == value {
            return true;
        } else if searchValue < value {
            if left == null {
                return false;
            } else {
                return left.Search(searchValue);
            }
        } else {
            if right == null {
                return false;
            } else {
                return right.Search(searchValue);
            }
        }
    }
}
