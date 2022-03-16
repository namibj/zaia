use super::super::value::Table;
use super::super::gc::Heap;

pub struct Ctx<'a> {
    global: &'a mut Table,
    scope: Vec<Table>,
    heap: &'a Heap,
}


impl<'a> Ctx<'a> {
    pub fn new(global: &'a mut Table, heap: &'a Heap) -> Self {
        Ctx {
            global,
            scope: vec![Table::new(heap.clone())],
            heap
        }
    }
}
