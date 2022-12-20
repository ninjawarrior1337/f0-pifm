use alloc::{ffi::CString, string::String};

pub struct CStringWrapper {
    inner: CString,
}

impl From<String> for CStringWrapper {
    fn from(s: String) -> Self {
        CStringWrapper {
            inner: CString::new(s.as_bytes()).unwrap(),
        }
    }
}

impl CStringWrapper {
    pub fn as_ptr(&self) -> *const i8 {
        self.inner.as_ptr()
    }
}

