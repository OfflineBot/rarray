use crate::Array2;

impl<T> Array2<T> {
    #[allow(unused)]
    pub fn get(&self, row: usize, col: usize) -> T {
        unsafe {
            let ptr = self.array.offset(row as isize * self.cols as isize + col as isize);
            std::ptr::read(ptr)
        }
    }

    #[allow(unused)]
    pub fn get_array(&self) -> *mut T {
        self.array
    }
}