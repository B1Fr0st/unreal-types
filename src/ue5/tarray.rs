#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TArray<T> {
    pub data: usize, // T* Data
    pub count: i32,
    pub max: i32,
    _marker: std::marker::PhantomData<T>,
}

impl <T: 'static + Copy> TArray<T> {
    pub fn is_valid(&self) -> bool {
        self.data != 0 && self.count >= 0 && self.count <= self.max
    }
    pub fn get(&self, index:i32) -> usize where T: 'static + Copy {
        self.data + (index as usize + size_of::<T>())
    }
    pub fn get_all(&self) -> impl Iterator<Item = usize> {
        (0..self.count).map(move |i| self.data + (i as usize * std::mem::size_of::<T>()))
    }
}