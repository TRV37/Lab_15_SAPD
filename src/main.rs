#[derive(Clone)]
struct Node {
    leaf: bool,
    keys: Vec<i32>,
    children: Vec<Option<Box<Node>>>,
}

struct BTree {
    degree: usize,
    root: Option<Box<Node>>,
}

impl Node {
    fn new(leaf: bool) -> Self {
        Node {
            leaf,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    fn search(&self, key: i32) -> bool {
        let mut i = 0;
        while i < self.keys.len() && key > self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && key == self.keys[i] {
            true
        } else if self.leaf {
            false
        } else {
            if let Some(ref child) = self.children[i] {
                child.search(key)
            } else {
                false
            }
        }
    }

    fn insert_non_full(&mut self, key: i32, degree: usize) {
        let mut i = self.keys.len();
        if self.leaf {
            self.keys.push(0);
            while i > 0 && key < self.keys[i - 1] {
                self.keys[i] = self.keys[i - 1];
                i -= 1;
            }
            self.keys[i] = key;
        } else {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }
            if let Some(ref mut child) = self.children[i] {
                if child.keys.len() == (2 * degree) - 1 {
                    self.split_child(i, degree);
                    if key > self.keys[i] {
                        i += 1;
                    }
                }
                if let Some(ref mut child) = self.children[i] {
                    child.insert_non_full(key, degree);
                }
            }
        }
    }

    fn split_child(&mut self, i: usize, degree: usize) {
        let y = self.children[i].as_ref().unwrap().clone();
        let mut z = Node::new(y.leaf);
        z.keys = y.keys[degree..].to_vec();
        self.keys[i..degree - 1].clone_from_slice(&y.keys[..degree - 1]);
        if !y.leaf {
            z.children = y.children[degree..].to_vec();
            self.children[i..degree].clone_from_slice(&y.children[..degree]);
        }
        self.children.insert(i + 1, Some(Box::new(z)));
        self.keys.insert(i, y.keys[degree - 1]);
    }
}

impl BTree {
    fn new(degree: usize) -> Self {
        BTree {
            degree,
            root: None,
        }
    }

    fn search(&self, key: i32) -> bool {
        if let Some(ref root) = self.root {
            root.search(key)
        } else {
            false
        }
    }

    fn insert(&mut self, key: i32) {
        match self.root {
            Some(ref mut root) => {
                if root.keys.len() == (2 * self.degree) - 1 {
                    let mut new_root = Node::new(false);
                    new_root.children.push(Some(Box::new(Node {
                        leaf: root.leaf,
                        keys: root.keys.clone(),
                        children: root.children.clone(),
                    })));
                    new_root.split_child(0, self.degree);
                    let i = if new_root.keys[0] < key { 1 } else { 0 };
                    if let Some(ref mut child) = new_root.children[i] {
                        child.insert_non_full(key, self.degree);
                    }
                    self.root = Some(Box::new(new_root));
                } else {
                    root.insert_non_full(key, self.degree);
                }
            },
            None => {
                let mut root = Node::new(true);
                root.keys.push(key);
                self.root = Some(Box::new(root));
            },
        }
    }
    
}

fn main() {
    // Создание и заполнение первого B-дерева
    let mut btree1 = BTree::new(3);
    for i in &[1, 3, 5, 7, 9] {
        btree1.insert(*i);
    }

    // Создание и заполнение второго B-дерева
    let mut btree2 = BTree::new(3);
    for i in &[2, 3, 5, 8, 10] { // здесь мы добавили 3 и 5, которые также присутствуют в первом дереве
        btree2.insert(*i);
    }

    // Проверяем одинаковые ключи в двух деревьях
    println!("Keys common to both trees:");
    for i in 1..=10 {
        if btree1.search(i) && btree2.search(i) {
            println!("{}", i);
        }
    }
}


