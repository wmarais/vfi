use crate::Value;

pub struct Stack {
    pub memory: Vec<u8>
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let mut memory = Vec::new();
        memory.reserve_exact(size);
        
        Self {
            memory
        }
    }

    #[inline(always)]
    pub fn push<const K: usize, V: Value<K>>(&mut self, value: V) {
        self.memory.extend(value.to_ne_bytes());
    }

    // #[inline(always)]
    // pub fn push_bytes<const K: usize>(&mut self, bytes: [u8; K]) {
    //     self.memory.extend(bytes);
    // }

    #[inline(always)]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.memory.extend(bytes);
    }

    #[inline(always)]
    pub fn pop<const K: usize, V: Value<K>>(&mut self) -> V {
        // Calculate where the value is located in memory.
        let end = self.memory.len();
        let beg = end - K;

        // Read the value.
        let value = V::from_ne_bytes(self.memory[beg..end].try_into().unwrap());
        
        // Reduce the size of the stack.
        self.memory.truncate(beg);
        value
    }

}