use sqlx::{query_as, Error, PgPool, };

use crate::serializers::Item;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn create_item(&self, name: &str, description: &str) -> Result<Item, Error> {
        let query = r#"
            INSERT INTO items (name, description)
            VALUES ($1, $2)
            RETURNING id, name, description
        "#;

        let row: (i32, String, String) = query_as(query)
            .bind(name)
            .bind(description)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(Item {
            id: row.0,
            name: row.1,
            description: row.2
        })
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, Error> {
        let query = r#"
            SELECT * FROM items
        "#;
        let result = query_as::<_, Item>(query).fetch_all(&self.db_pool).await?;
        Ok(result)
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        let query = r#"SELECT * FROM items WHERE id = $1"#;
        let result = query_as::<_, Item>(query)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(result)
    }

    pub async fn update_item(
        &self,
        id: i32,
        name: &str,
        description: &str
    ) -> Result<Option<Item>, Error> {
        let query = r#"
            UPDATE items
            SET name = $1, description = $2
            WHERE id =  $3
            RETURNING id, name, description
        "#;
        let result = query_as::<_, Item>(query)
           .bind(name)
           .bind(description)
           .bind(id)
           .fetch_optional(&self.db_pool).await?;

        Ok(result)
    }

    pub async fn delete_item(&self, id: i32) -> Result<bool, Error> {
        let query = r#"
            DELETE FROM items
            WHERE id = $1
            RETURNING id, name, description
        "#;
        let result = sqlx::query(query)
           .bind(id)
           .execute(&self.db_pool)
           .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_all_items(&self) -> Result<u64, Error> {
        let query = r#"DELETE FROM item"#;
        let result = sqlx::query(query).execute(&self.db_pool).await?;

        Ok(result.rows_affected())
    }



}