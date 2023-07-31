use anyhow::Result;
use clickhouse_rs::Client;

async fn seed() -> Result<()> {
    let client = Client::new("http://localhost:8123");
    client.execute("CREATE TABLE IF NOT EXISTS test(name String, age Int32, PRIMARY KEY name) ENGINE = MergeTree;").await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use clickhouse_rs::Client;
    use serde::Deserialize;

    use crate::seed;

    #[tokio::test]
    async fn test_ping() -> Result<()> {
        let client = Client::new("http://localhost:8123");
        let is_ok = client.ping().await?;
        assert!(is_ok);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_table() -> Result<()> {
        let client = Client::new("http://localhost:8123");

        client
            .execute(
                "CREATE TABLE IF NOT EXISTS test(name String, age Int32, PRIMARY KEY name) ENGINE = MergeTree;",
            )
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        seed().await?;

        #[derive(Debug, Deserialize)]
        struct User {
            pub name: String,
            pub age: i32,
        }
        let client = Client::new("http://localhost:8123");

        client
            .execute("INSERT INTO test(name, age) VALUES ('John', 42) ;")
            .await?;
        client
            .execute("INSERT INTO test(name, age) VALUES ('Tommy', 34) ;")
            .await?;

        let result = client
            .query::<User>("SELECT * from test ORDER BY name ASC FORMAT JSON")
            .await?;

        assert_eq!(result.data.len(), 2);
        assert_eq!(result.data[0].name, "John");
        assert_eq!(result.data[0].age, 42);
        assert_eq!(result.data[1].name, "Tommy");
        assert_eq!(result.data[1].age, 34);
        assert_eq!(result.statistics.rows_read, 2);

        Ok(())
    }
}
