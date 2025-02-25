use bumpalo::Bump;

use crate::ast::Node;

pub struct Arena {
    bump: Bump,
}

impl Arena {
    pub fn new() -> Self {
        Arena { bump: Bump::new() }
    }

    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn push<'arena>(&'arena self, node: Node<'arena>) -> &'arena mut Node<'arena> {
        // This fails if the bump allocator is out of memory.
        self.bump
            .try_alloc_with(|| node)
            .unwrap_or_else(|_| std::process::abort())
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn push<'arena>(&'arena self, node: Node<'arena>) -> &'arena mut Node<'arena> {
        self.bump.alloc_with(|| node)
    }

    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn push_slice<'arena>(
        &'arena self,
        nodes: &[&'arena Node<'arena>],
    ) -> &'arena [&'arena Node<'arena>] {
        // This fails if the bump allocator is out of memory.
        self.bump
            .try_alloc_slice_copy(nodes)
            .unwrap_or_else(|_| std::process::abort())
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn push_slice<'arena>(
        &'arena self,
        nodes: &[&'arena Node<'arena>],
    ) -> &'arena [&'arena Node<'arena>] {
        self.bump.alloc_slice_copy(nodes)
    }

    fn alloc_str(&self, src: &str) -> &str {
        self.bump
            .try_alloc_str(src)
            .unwrap_or_else(|_| std::process::abort())
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Buffer(String);

impl Buffer {
    pub fn new(size_hint: usize) -> Self {
        Buffer(String::with_capacity(size_hint))
    }

    pub fn get_builder(&mut self) -> StringBuilder<'_> {
        StringBuilder::new(self)
    }
}

/// A helper type to safely build a string in the buffer from multiple pieces.
///
/// It takes an exclusive reference to the buffer and clears everything in the
/// buffer before we start building. This guarantees that upon finishing, the
/// buffer contains only what we wrote to it.
pub struct StringBuilder<'buffer> {
    buffer: &'buffer mut Buffer,
}

impl<'buffer> StringBuilder<'buffer> {
    pub fn new(buffer: &'buffer mut Buffer) -> Self {
        // Clear the buffer before we start building.
        buffer.0.clear();
        StringBuilder { buffer }
    }

    #[inline]
    pub fn push_str(&mut self, src: &str) {
        self.buffer.0.push_str(src)
    }

    pub fn push_char(&mut self, c: char) {
        self.buffer.0.push(c)
    }

    pub fn finish(self, arena: &Arena) -> &str {
        arena.alloc_str(&self.buffer.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_test() {
        let arena = Arena::new();
        let node = Node::Space("Hello, world!");
        let reference = arena.push(node);
        assert!(matches!(reference, Node::Space("Hello, world!")));
    }

    #[test]
    fn buffer_extend() {
        let arena = Arena::new();
        let mut buffer = Buffer::new(0);
        let mut builder = buffer.get_builder();
        builder.push_char('H');
        builder.push_char('i');
        let str_ref = builder.finish(&arena);
        assert_eq!(str_ref, "Hi");
    }

    #[test]
    fn buffer_manual_reference() {
        let arena = Arena::new();
        let mut buffer = Buffer::new(0);
        let mut builder = buffer.get_builder();
        assert_eq!(builder.buffer.0.len(), 0);
        builder.push_char('H');
        builder.push_char('i');
        builder.push_char('↩'); // This is a multi-byte character.
        assert_eq!(builder.buffer.0.len(), 5);
        let str_ref = builder.finish(&arena);
        assert_eq!(str_ref.len(), 5);
        assert_eq!(str_ref, "Hi↩");
    }

    struct CycleParticipant<'a> {
        val: i32,
        next: Option<&'a mut CycleParticipant<'a>>,
    }

    #[test]
    fn basic_arena() {
        let arena = Bump::new();

        let a = arena.alloc(CycleParticipant { val: 1, next: None });
        let b = arena.alloc(CycleParticipant { val: 2, next: None });
        a.next = Some(b);
        let c = arena.alloc(CycleParticipant { val: 3, next: None });
        a.next.as_mut().unwrap().next = Some(c);

        // for (i, node) in arena.iter_mut().enumerate() {
        //     match i {
        //         0 => assert_eq!(node.val, 1),
        //         1 => assert_eq!(node.val, 2),
        //         2 => assert_eq!(node.val, 3),
        //         _ => panic!("Too many nodes"),
        //     }
        // }

        assert_eq!(a.val, 1);
        assert_eq!(a.next.as_ref().unwrap().val, 2);
        assert_eq!(a.next.as_ref().unwrap().next.as_ref().unwrap().val, 3);
    }
}
