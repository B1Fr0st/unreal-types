#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FString {
    pub data: usize, // *const u16 (wchar_t)
    pub count: i32,
    pub max: i32,
}

impl FString {
    pub fn is_valid(&self) -> bool {
        self.data != 0
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.data as *const u16
    }
}
