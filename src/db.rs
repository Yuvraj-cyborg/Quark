#[derive(Debug,Clone)]
pub struct QuarkDB {
   pages: Vec<Page>,
}

#[derive(Debug,Clone)]
struct Page {
    data: [u8; 4096]
}

impl Page {
    pub fn insert()
    pub fn search()
    pub fn free()
    pub fn is_full() -> bool {}
}
