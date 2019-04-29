use super::stack::Stack;

pub struct UnreleasableVector<T> {
    content: Vec<T>,
    garbage: Stack<usize>,
}

impl<T> UnreleasableVector<T> {
    #[inline]
    pub fn new() -> Self {
        UnreleasableVector {
            content: Vec::new(),
            garbage: Stack::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, t: T) {
        let content = &mut self.content;
        if let Some(index) = self.garbage.pop() {
            content[index] = t;
        } else {
            content.push(t);
        }
    }
}
