use std::collections::HashMap;

struct Node {
    valor: i32,
    proximo: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add_node(&mut self, valor: i32) {
        let node = Node {
            valor: valor,
            proximo: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    fn print_list(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            println!("{}", node.valor);
            current = &node.proximo;
        }
    }

    fn conta_repetidos(&self) -> HashMap<i32, i32> {
        // Passa uma copía do valor do cabeçalho
        let mut current_node = &self.head;
        let mut count_map: HashMap<i32, i32> = HashMap::new();

        while let Some(node) = current_node {
            *count_map.entry(node.valor).or_insert(0) += 1;
            current_node = &node.proximo;
        }

        count_map
    }

    fn mostrar_repetidos(&self) {
        let mut count_map: HashMap<i32, i32> = self.conta_repetidos();

        for (value, count) in count_map.iter() {
            if *count > 1 {
                println!("O valor {} aparece {} na lista.", value, count);
            }
        }
    }
}

fn main () {
    let mut list = LinkedList::new();

    list.add_node(1);
    list.add_node(1);
    list.add_node(2);
    list.add_node(3);
    list.add_node(3);
    list.add_node(3);
    list.add_node(3);

    list.print_list();
    list.mostrar_repetidos();
}