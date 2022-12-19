
/// Two dimensional rectangular array of size (I x J)
///
/// First dimension is the fastest moving index, so:
///  - Fast to iterate along first dimension (I)
///  - Fast to expand along second dimension (J)
///
/// Uses isize so we can do arithmetic around coordinates
pub struct Array2d<T: Copy> {
    data: Vec<T>,
    size_i: isize,
    size_j: isize,
}

impl<T: Copy> Array2d<T> {
    #[allow(dead_code)]
    pub fn new(initial: T, size_i: isize, size_j: isize) -> Array2d<T> {
        if size_i < 0 || size_j < 0 {
            panic!("Invalid sizes given for Array2d");
        }
        let count = (size_i as usize)*(size_j as usize);
        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            data.push(initial);
        }
        Array2d {
            data: data,
            size_i: size_i,
            size_j: size_j,
        }
    }

    pub fn newu(initial: T, size_i: usize, size_j: usize) -> Array2d<T> {
        if size_i > (isize::MAX as usize) || size_j > (isize::MAX as usize) {
            panic!("Invalid sizes given for Array2d");
        }
        let count = size_i * size_j;
        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            data.push(initial);
        }
        Array2d {
            data: data,
            size_i: size_i as isize,
            size_j: size_j as isize,
        }
    }

    #[allow(dead_code)]
    pub fn size_u(&self) -> (usize, usize) {
        (self.size_i as usize, self.size_j as usize)
    }
    
    #[allow(dead_code)]
    pub fn size_i(&self) -> (isize, isize) {
        (self.size_i, self.size_j)
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
        &self.data[i as usize + (self.size_i as usize)*(j as usize)]
    }

    #[allow(dead_code)]
    pub fn getu(&self, (i, j): (usize, usize)) -> &T {
        &self.data[i + (self.size_i as usize)*j]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, (i, j): (isize, isize)) -> &mut T {
        &mut self.data[i as usize + (self.size_i as usize)*(j as usize)]
    }

    #[allow(dead_code)]
    pub fn get_mutu(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.data[i + (self.size_i as usize)*j]
    }
    
    /// Create a new row (add 1 to second dimension)
    /// 
    ///  - Fills with items in iterator
    ///  - Either size_i == size_j == 0 or iterator must provide exactly size_i
    ///    elements.
    #[allow(dead_code)]
    pub fn add_row<I: std::iter::Iterator<Item=T>>(&mut self, iterator: I) {
        self.data.reserve(self.size_i as usize);

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

