pub enum Node<T> {
    Item(T),
    List(Vec<Node<T>>),
}

pub type List<T> = Vec<Node<T>>;
