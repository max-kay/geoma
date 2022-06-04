struct MultiVector {
    baselist: Vec<ScalarBase>,
}

#[derive(Clone)]
struct ScalarBase {
    scalar: f32,
    base: Vec<u32>,
}

impl ScalarBase {
    fn new(mut scalar: f32, mut base: Vec<u32>) -> Option<ScalarBase> {
        let mut swap_count = 0;
        let mut unsorted = true;
        while unsorted {
            unsorted = false;
            for i in 0..(base.len() - 1) {
                if base[i] == base[i + 1] {
                    return None;
                }
                if base[i] > base[i + 1] {
                    unsorted = true;
                    base.swap(i, i + 1);
                    swap_count += 1;
                }
            }
        }
        if swap_count % 2 == 1 {
            scalar = -scalar;
        }
        Some(ScalarBase { scalar, base })
    }

    fn geo(&self, other: &Self) -> Option<Self> {
        let mut base = self.base.clone();
        base.append(&mut other.base.clone());
        ScalarBase::new(self.scalar * other.scalar, base)
    }

    fn add_unchecked(&self, other: &Self) -> Self {
        Self {
            scalar: self.scalar + other.scalar,
            base: self.base.clone(),
        }
    }

    fn is_basically_the_same(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl MultiVector {
    fn geo(&self, other: &Self) -> Self {
        let x1 = self.baselist.clone();
        let x2 = other.baselist.clone();
        let baselist = x1
            .iter()
            .flat_map(|x1_i| x2.iter().map(|x2_j| x1_i.geo(x2_j)))
            .flatten()
            .collect();
        Self::new(baselist)
    }

    fn new(mut baselist: Vec<ScalarBase>) -> Self {
        let mut indices = Vec::new();
        for i in 0..baselist.len() {
            for j in i + 1..baselist.len() {
                if baselist[i].is_basically_the_same(&baselist[j]) {
                    indices.push(j);
                    baselist[i] = baselist[i].add_unchecked(&baselist[j]);
                }
            }
        }
        for index in indices.into_iter().rev() {
            baselist.remove(index);
        }
        MultiVector { baselist }
    }

    fn from_ndvector(v: Vec<f32>) -> Self {
        let mut baselist = Vec::with_capacity(v.len());
        for (i, x_i) in v.into_iter().enumerate() {
            baselist.push(ScalarBase {
                scalar: x_i,
                base: vec![i as u32],
            });
        }
        Self { baselist }
    }
}

fn main() {
    let x1 = ScalarBase::new(4.0, vec![1, 2]).unwrap();
    let x2 = ScalarBase::new(5.0, vec![2, 3]).unwrap();
    let x3 = ScalarBase::new(6.0, vec![1, 3]).unwrap();
    let v = MultiVector::new(vec![x1, x2, x3]);
}
