struct List {
    value: i32,
    next: Option<Box<List>>
}

impl List {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            next: None
        }
    }

    fn has_next_elem(elem: &Option<Box<List>>) -> bool {
        if let Some(node) = elem {
            if let Some(next) = &node.next {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn append(&mut self, value: i32) {
        if let None = self.next {
            self.next = Some(Box::new(List::new(value)));
        } else {
            let mut temp = &mut self.next;
            loop {
                if !List::has_next_elem(temp) {
                    temp.as_mut().unwrap().next = Some(Box::new(List::new(value)));
                    break;
                }
                temp = &mut temp.as_mut().unwrap().next;
            }
        }
    }

    pub fn display(&self) {
        println!("{}", self.value);
        let mut temp = &self.next;
        loop {
            if let Some(node) = temp {
                println!("{}", node.value);
                //let temp = &node.next;
                temp = &node.next;
            } else {
                break;
            }
        }
    }
}

fn main () {
    let mut l = List::new(5);

    l.append(6);
    l.append(7);
    l.append(8);
    l.append(9);
    l.append(10);

    l.display();
}
