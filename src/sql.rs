use log::*;
use sqlx::{postgres::PgPool };
use crate::models::Human;

pub async fn get_human_data(
    connection: &PgPool,
    name:String
) -> anyhow::Result<Human> {
    info!("query human data ");
    let rec = sqlx::query!(
        r#"
    SELECT id, name
    FROM human
    WHERE name=$1
    "#,
    name
    )
        .fetch_one(&*connection)
        .await?;
    Ok(Human{ id: rec.id, name:rec.name })
}


pub async fn create_human_data(
    connection: &PgPool,
    name:String
) -> anyhow::Result<Human> {
    info!("create human data ");
    let rec = sqlx::query!(
        r#"
    INSERT INTO human (name) VALUES($1) RETURNING id,name
    "#,name
    )
        .fetch_one(connection)
        .await?;
    Ok(Human{ id: rec.id, name:rec.name })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[actix_rt::test]
    async fn test_create_human_data() {

        let url =  "postgresql://pdemo:pdemo@127.0.0.1:5431/pdemo";
        std::env::set_var("DATABASE_URL", url);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let  pool = PgPool::connect(&database_url).await.unwrap();

        let x = create_human_data(&pool, "keng".to_string()).await.unwrap();
        let q = get_human_data(&pool, "keng".to_string() ).await.unwrap();
        assert_eq!(x.name, q.name)
    }
}
