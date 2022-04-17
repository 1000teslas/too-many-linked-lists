use std::rc::Rc;

struct Node<T> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, val: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                val,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Option<Self> {
        self.head.as_ref().map(|node| List {
            head: node.next.clone(),
        })
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter(self.head.as_deref())
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take().map(Rc::try_unwrap);
        while let Some(Ok(mut node)) = curr {
            curr = node.next.take().map(Rc::try_unwrap);
        }
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail().unwrap();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert!(matches!(list, None));
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn drop() {
        let mut l = List::new();
        for i in 0..1 << 20 {
            l = l.prepend(Box::new([i]));
        }
    }
}
