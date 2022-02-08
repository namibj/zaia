pub mod gc;
pub mod value;

use value::RefValue;

pub type Heap = gc::Heap<RefValue>;
