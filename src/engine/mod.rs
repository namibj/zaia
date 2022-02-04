mod gc;
mod scope;
mod value;

use gc::Handle;
use value::Table;

pub struct Engine {
    environment: Handle<Table>,
}
