pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // 创建新栈
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // 压入元素
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // 弹出元素
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // 查看栈顶元素
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // 获取栈大小
    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}
