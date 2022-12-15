
/// Two dimensional rectangular array of size (I x J)
///
/// First dimension is the fastest moving index, so:
///  - Fast to iterate along first dimension (I)
///  - Fast to expand along second dimension (J)
///
/// Uses isize so we can do arithmetic around coordinates
pub struct Array2d<T: Copy> {
    data: Vec<T>,
    size_i: usize,
    size_j: usize,
}

impl<T: Copy> Array2d<T> {
    #[allow(dead_code)]
    pub fn new(initial: T, size_i: usize, size_j: usize) -> Array2d<T> {
        let mut data = Vec::with_capacity(size_i*size_j);
        for _ in 0..(size_i*size_j) {
            data.push(initial);
        }
        Array2d {
            data: data,
            size_i: size_i,
            size_j: size_j,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> (usize, usize) {
        (self.size_i, self.size_j)
    }
    
    #[allow(dead_code)]
    pub fn size_i(&self) -> (isize, isize) {
        (self.size_i as isize, self.size_j as isize)
    }
    
    #[allow(dead_code)]
    pub fn new_empty() -> Array2d<T> {
        Array2d {
            data: Vec::new(),
            size_i: 0,
            size_j: 0,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, (i, j): (isize, isize)) -> &T {
        &self.data[i as usize + self.size_i*j as usize]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, (i, j): (isize, isize)) -> &mut T {
        &mut self.data[i as usize + self.size_i*j as usize]
    }
    
    /// Create a new row (add 1 to second dimension)
    /// 
    ///  - Fills with items in iterator
    ///  - Either size_i == size_j == 0 or iterator must provide exactly size_i
    ///    elements.
    #[allow(dead_code)]
    pub fn add_row<I: std::iter::Iterator<Item=T>>(&mut self, iterator: I) {
        self.data.reserve(self.size_i);

        let mut count = 0;
        for item in iterator {
            count += 1;
            self.data.push(item);
        }
        if self.size_i == 0 && self.size_j == 0 {
            self.size_i = count;
        } else if count != self.size_i {
            panic!("Unexpect number of items added to Array2d");
        }
        self.size_j += 1;
    }
}

