#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Params {
    replicas: i32,
    table_name: Option<String>,
}

fn main() {
    let pool = my::Pool::new("mysql://root:wanglei123@localhost:3306/mysql").unwrap();
    pool.prep_exec("show tables", ()).unwrap();
}
