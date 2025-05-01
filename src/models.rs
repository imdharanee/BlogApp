use async_graphql::SimpleObject;
use serde::{Serialize,Deserialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize,SimpleObject,FromRow)]
pub struct User {
   pub id:i32,
   pub name:String,
}

#[derive(Debug,Serialize,Deserialize,SimpleObject,FromRow)]
pub struct Post {

    pub id:i32,
    pub title:String,
    pub content:String,
    pub user_id:i32,
}
#[derive(Debug, Serialize, Deserialize, SimpleObject,FromRow)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    
    

}
#[derive(Debug,Serialize,Deserialize,SimpleObject,FromRow)]
pub struct PostWithComments{
      pub post:Post,
      pub comments:Vec<Comment>,
}
