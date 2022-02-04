mod gc;
mod value;

use value::Table;

pub struct Engine {
    environment: Table,
}
