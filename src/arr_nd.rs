
/// N dimensional rectangular array of size (I x J x ...)
///
/// First dimension is the fastest moving index, so:
///  - Fast to iterate along first dimension (I)
///
/// Uses isize and usize so we can do arithmetic around coordinates
#[derive(Clone,PartialEq)]
pub struct ArrayND<T: Copy, const N_DIM: usize> {
    data: Vec<T>,
    size: [usize; N_DIM],
}

impl<T: Copy, const N_DIM: usize> ArrayND<T, N_DIM> {
    #[allow(dead_code)]
    pub fn new(initial: T, size: [isize; N_DIM]) -> ArrayND<T,N_DIM> {
        let mut size_u = [0; N_DIM];
        for i in 0..N_DIM {
            if size[i] < 0 {
                panic!("Negative size given to ArrayND");
            }
            size_u[i] = size[i] as usize;
        }
        Self::newu(initial, size_u)
    }

    #[allow(dead_code)]
    pub fn newu(initial: T, size: [usize; N_DIM]) -> ArrayND<T,N_DIM> {
        let mut count = 1;
        for i in 0..N_DIM {
            let s = size[i];
            if usize::MAX / s < count {
                panic!("Multi-dimensional array is too large");
            }
            if s > (isize::MAX as usize) {
                panic!(
                    "Multi-dimensional array dimension is too large (dimension {} is {})",
                    i, s);
            }
            count *= s;
        }

        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            data.push(initial);
        }
        ArrayND {
            data: data,
            size: size,
        }
    }

    #[allow(dead_code)]
    pub fn size_u(&self) -> [usize; N_DIM] {
        self.size
    }
    
    #[allow(dead_code)]
    pub fn size_i(&self) -> [isize; N_DIM] {
        let mut size_i = [0; N_DIM];
        for i in 0..N_DIM {
            size_i[i] = self.size[i] as isize;
        }
        size_i
    }
    
    #[allow(dead_code)]
    pub fn get(&self, index: &[isize; N_DIM]) -> &T {
        let i = self.calc_data_index(index);
        &self.data[i]
    }

    #[allow(dead_code)]
    pub fn getu(&self, index: &[usize; N_DIM]) -> &T {
        let i = self.calc_data_indexu(index);
        &self.data[i]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, index: &[isize; N_DIM]) -> &mut T {
        let i = self.calc_data_index(index);
        &mut self.data[i]
    }

    #[allow(dead_code)]
    pub fn get_mutu(&mut self, index: &[usize; N_DIM]) -> &mut T {
        let i = self.calc_data_indexu(index);
        &mut self.data[i]
    }
    
    fn calc_data_index(&self, index: &[isize; N_DIM]) -> usize {
        let mut data_index = 0;
        let mut stride = 1;
        for i in 0..N_DIM {
            let dim_index = index[i];
            if dim_index < 0 || dim_index as usize >= self.size[i] {
                panic!(
                    "Out of bounds access (element {} of dimension {} with size {})",
                    index[i],
                    i,
                    self.size[i]);
            }
            data_index += stride*(dim_index as usize);
            stride *= self.size[i] as usize;
        }
        data_index as usize
    }
    
    fn calc_data_indexu(&self, index: &[usize; N_DIM]) -> usize {
        let mut data_index = 0;
        let mut stride = 1;
        for i in 0..N_DIM {
            let dim_index = index[i];
            if dim_index > self.size[i] as usize {
                panic!(
                    "Out of bounds access (element {} of dimension {} with size {})",
                    dim_index,
                    i,
                    self.size[i]);
            }
            data_index += stride*index[i];
            stride *= self.size[i];
        }
        data_index
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_nd() {
        let mut x: i32 = 0;
        let mut arr = ArrayND::new(x, [2,3,4,5]);
        let mut ind = [0 as isize; 4];
        for i in 0..2 {
            ind[0] = i;
            for j in 0..3 {
                ind[1] = j;
                for k in 0..4 {
                    ind[2] = k;
                    for l in 0..5 {
                        ind[3] = l;
                        *arr.get_mut(&ind) = x;
                        x += 1;
                    }
                }
            }
        }
        let arr = arr;
        x = 0;
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    for l in 0..5 {
                        assert_eq!(*arr.get(&[i,j,k,l]), x);
                        x += 1;
                    }
                }
            }
        }
    }
}

