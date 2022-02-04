mod value;
mod gc;

use value::Table;

pub struct Engine {
    environment: Table,
}
