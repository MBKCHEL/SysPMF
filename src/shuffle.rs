use rand::seq::SliceRandom;

pub struct Shuffle {
    order: Vec<usize>,
    pos: usize,
}

impl Shuffle {
    pub fn new(len: usize, first: Option<usize>) -> Self {
        let mut order: Vec<usize> = (0..len).collect();
        order.shuffle(&mut rand::rng());
        if let Some(f) = first {
            if let Some(p) = order.iter().position(|&x| x == f) {
                order.swap(0, p);
            }
        }
        Self { order, pos: 0 }
    }

    pub fn current(&self) -> usize {
        self.order.get(self.pos).copied().unwrap_or(0)
    }

    pub fn next(&mut self) -> usize {
        if self.order.is_empty() {
            return 0;
        }
        if self.pos + 1 < self.order.len() {
            self.pos += 1;
        } else {
            let last = self.order[self.pos];
            self.order.shuffle(&mut rand::rng());
            if self.order.len() > 1 && self.order[0] == last {
                let end = self.order.len() - 1;
                self.order.swap(0, end);
            }
            self.pos = 0;
        }
        self.order[self.pos]
    }

    pub fn prev(&mut self) -> usize {
        self.pos = self.pos.saturating_sub(1);
        self.current()
    }
}
