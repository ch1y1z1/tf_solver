pub trait ChunkIterator: Iterator {
    fn chunks_n(self, chunk_size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks {
            iter: self,
            chunk_size,
        }
    }
}

pub struct Chunks<I> {
    iter: I,
    chunk_size: usize,
}

unsafe impl<I: Send> Send for Chunks<I> {}

impl<T, I: Iterator<Item = T>> Iterator for Chunks<I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            if let Some(item) = self.iter.next() {
                chunk.push(item);
            } else {
                break;
            }
        }
        if chunk.is_empty() { None } else { Some(chunk) }
    }
}

impl<I: Iterator> ChunkIterator for I {}

#[test]
fn test_chunks() {
    let it = vec![1, 2, 3, 4, 5].into_iter().chunks_n(2);

    it.for_each(|i| {
        println!("{:?}", i);
    });
}
