use async_graphql::{InputObject, SimpleObject};
use db::schema::content;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Insertable)]
#[table_name = "content"]
pub struct CreateContent {
  pub title: String,
}

derive_id! {
  #[derive(Identifiable)]
  #[table_name = "content"]
  #[graphql(name = "ContentID")]
  pub struct ContentId(#[column_name = "id"] i32);
}

#[derive(SimpleObject, Queryable, Clone, Debug, Serialize, Deserialize)]
pub struct Content {
  pub id: ContentId,
  pub title: String,
}
