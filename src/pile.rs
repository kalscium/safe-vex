use alloc::{vec::Vec, boxed::Box};

/// The addres used by the pile to index logs (limits how many unique logs you can have)
pub type Addr = u8;
pub type AddrCounter = u8;

pub struct Pile<T: PartialEq> {
    namespace: Vec<T>,
    order: Vec<(Addr, AddrCounter)>,
}

impl<T: PartialEq> Pile<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            namespace: Vec::new(),
            order: Vec::new(),
        }
    }

    /// Pushes a specified item onto the top of a pile
    #[inline]
    pub fn push(&mut self, item: T) {
        let addr = self.namespace.iter()
            .enumerate()
            .filter(|(_, x)| *x == &item)
            .map(|(i, _)| i as Addr)
            .collect::<Box<_>>();
        let addr: Addr = if addr.is_empty() {
            if self.namespace.len() > Addr::MAX as usize { return } // just don't log beyond the limit (never panic)
            self.namespace.push(item);
            (self.namespace.len()-1) as Addr // address  
        } else if let Some(addr) = addr.first() { *addr }
        else { return }; // don't panic

        match self.order.last_mut() {
            Some((x, i)) if *x == addr && *i != AddrCounter::MAX => *i += 1,
            _ => self.order.push((addr, 1)),
        }
    }

    /// Flushes the data on a pile while iterating through it (like map)
    #[inline]
    pub fn flush(&mut self, f: impl Fn(&T, AddrCounter)) {
        for (addr, i) in self.order.iter() {
            let x = self.namespace.get(*addr as usize);
            if let Some(x) = x { f(x, *i) }
        }

        self.order = Vec::new(); // clears all of the pile's data
    }
}

impl<T: PartialEq> Default for Pile<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}