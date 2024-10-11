use std::fmt::{format, Display, Formatter, Result};
enum LinkedList {
    Node { value: i32, next: Box<LinkedList> },
    Nil,
}

impl LinkedList {
    fn new() -> LinkedList {
        return LinkedList::Nil;
    }

    fn add(self, value: i32) -> LinkedList {
        return LinkedList::Node {
            value,
            next: Box::new(self),
        };
    }

    fn remove(self, val: i32) -> LinkedList {
        match self {
            Self::Node { value, next } => {
                if value == val {
                    return *next;
                } else {
                    return LinkedList::Node {
                        value,
                        next: Box::new(next.remove(val)),
                    };
                }
            }
            Self::Nil => {
                return LinkedList::Nil;
            }
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Node { value, next } => {
                return format!(" {} -> {}", value, next.stringify());
            }
            Self::Nil => {
                return format!("{}", "Nil");
            }
        }
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.stringify())
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list = linked_list.add(23);
    linked_list = linked_list.add(45);
    linked_list = linked_list.add(123);
    println!("The linked list looks like : {}", linked_list);
    linked_list = linked_list.remove(23);
    println!("The linked list looks like : {}", linked_list);
}
