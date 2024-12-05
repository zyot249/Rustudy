pub struct DisjointSets {
    subsets: Vec<Subset>,
}

struct Subset {
    parent: usize,
    rank: usize,
}

impl DisjointSets {
    pub fn new(n: usize) -> Self {
        let mut subsets = Self { subsets: vec![] };

        for i in 0..n {
            subsets.subsets.push(Subset {
                parent: i,
                rank: 0
            });
        }

        subsets
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.subsets[x].parent != x {
            self.subsets[x].parent = self.find(self.subsets[x].parent);
        }

        self.subsets[x].parent
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xroot = self.find(x);
        let yroot = self.find(y);

        if self.subsets[xroot].rank < self.subsets[yroot].rank {
            self.subsets[xroot].parent = self.subsets[yroot].parent;
        } else if self.subsets[xroot].rank > self.subsets[yroot].rank {
            self.subsets[yroot].parent = self.subsets[xroot].parent;
        } else {
            self.subsets[xroot].parent = self.subsets[yroot].parent;
            self.subsets[yroot].rank += 1;
        }
    }
}
