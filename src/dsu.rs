use std::{collections::HashMap, hash::Hash, mem::swap};

pub struct DSU<T> {
    pub par: HashMap<T, T>,
    pub size: HashMap<T, usize>,
}
impl<T: Eq + Hash + Clone> DSU<T> {
    pub fn new() -> Self {
        Self {
            par: HashMap::new(),
            size: HashMap::new(),
        }
    }
    pub fn get_par(&mut self, child: &T) -> T {
        if self.par.entry(child.clone()).or_insert(child.clone()) == child {
            return child.clone();
        }
        let childs_parent = self.par[child].clone();
        let set_master = self.get_par(&childs_parent);
        self.par.insert(child.clone(), set_master.clone());
        set_master
    }

    pub fn new_entry(&mut self, a: &T) {
        self.par.insert(a.clone(), a.clone());
        self.size.insert(a.clone(), 1);
    }

    pub fn union(&mut self, a: &T, b: &T) {
        let mut a_master = self.get_par(a);
        let mut b_master = self.get_par(b);
        if a_master == b_master {
            return;
        }

        if self.size[&a_master] > self.size[&b_master] {
            swap(&mut a_master, &mut b_master);
        }
        self.par.insert(a_master.clone(), b_master.clone());
        self.size.insert(
            b_master.clone(),
            self.size[&b_master] + self.size[&a_master],
        );
    }
}
