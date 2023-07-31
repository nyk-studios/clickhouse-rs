use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetaField {
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Deserialize)]
pub struct QueryResult<TResult> {
    pub data: Vec<TResult>,
    pub meta: Vec<MetaField>,
    pub rows: u64,
    pub statistics: QueryStatistics,
}

#[derive(Debug, Deserialize)]
pub struct QueryStatistics {
    pub bytes_read: u64,
    pub elapsed: f64,
    pub rows_read: u64,
}
