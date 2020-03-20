#[macro_use]
use super::schema::location;

#[derive(Eq, PartialEq, Debug, Queryable)]
pub struct Location {
    pub id: u32,
    name: String,
}

#[derive(Insertable)]
#[table_name="location"]
pub struct NewLocation<'a> {
    pub name: &'a str,
}

