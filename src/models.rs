use chrono::NaiveDate;

#[derive(Debug, Clone, Queryable)]
pub(crate) struct Issue {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created: NaiveDate,
}