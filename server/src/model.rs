#[macro_use]

#[derive(Queryable)]
pub struct Location {
    pub id: u64,
    name: String,
}

#[derive(Insertable)]
#[table_name="location"]
pub struct NewLocation<'a> {
    pub name: &'a str,
}

