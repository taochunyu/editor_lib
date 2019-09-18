use crate::core::model::fragment;
use crate::core::model::fragment::Fragment;

pub struct Slice {
    pub content: Fragment,
    pub open_start: usize,
    pub open_end: usize,
}

impl Slice {
   fn size(&self) -> usize {
       self.content.size()
   }
}
