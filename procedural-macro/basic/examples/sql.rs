use basic::sql;

fn main() {
    sql!(select * from table1 where id = 10 and timestamp > 1000);
}