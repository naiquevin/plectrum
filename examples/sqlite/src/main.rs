use std::collections::HashMap;

use plectrum::Plectrum;
use sqlx::SqlitePool;

#[derive(Debug, Plectrum)]
#[plectrum(rename_all = "snake_case")]
enum ItemState {
    Todo,
    InProgress,
    Completed,
    Parked,
    Archived,
}

#[derive(Debug, sqlx::FromRow)]
struct ItemStateRow {
    id: i32,
    label: String,
}

struct ItemStateModel<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ItemStateModel<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }
}

impl<'a> plectrum::DataSource for ItemStateModel<'a> {
    type Id = i32;

    async fn load(&self) -> Result<std::collections::HashMap<i32, String>, plectrum::Error> {
        let q = "SELECT * FROM item_states";
        let rows = sqlx::query_as::<_, ItemStateRow>(q)
            .fetch_all(self.pool)
            .await
            .map_err(|e| {
                plectrum::Error::DataSource(e.to_string())
            })?;
        let mut res = HashMap::new();
        for row in rows {
            res.insert(row.id, row.label);
        }
        Ok(res)
    }
}

#[tokio::main]
async fn main() {
    let dbpath = std::env::var("PLECTRUM_SQLITE_DB").unwrap_or(String::from("database/todos.db"));
    let conn_str = format!("sqlite:{dbpath}");
    let pool = SqlitePool::connect(&conn_str)
        .await
        .expect("Failed to connect to the db");
    let model = ItemStateModel::new(&pool);
    match plectrum::Mapping::<i32, ItemState>::load(&model).await {
        Ok(mapping) => {
            dbg!(mapping.by_id(1));
            dbg!(mapping.by_value("in_progress"));
            dbg!(mapping.get_id(&ItemState::Parked));
        }
        Err(e) => {
            panic!("Failed to initialize mapping: {e:?}");
        }
    }
}
