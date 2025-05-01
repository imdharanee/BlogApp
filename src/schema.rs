
use async_graphql::{Context, Object};
use sqlx::PgPool;
use crate::models::{Comment, Post, PostWithComments, User};

pub struct QueryRoot;

#[Object]
impl QueryRoot {

        async fn get_posts(&self,ctx:&Context<'_>)->async_graphql::Result<Vec<Post>> {

            let pool=ctx.data::<PgPool>()?;

            let posts=sqlx::query_as(
                "SELECT * FROM POSTS"
            ).fetch_all(pool).await?;

            Ok(posts)
        }
        async fn get_user_posts(&self,ctx:&Context<'_>,user_id:i32)->async_graphql::Result<Vec<Post>> {
              let pool=ctx.data::<PgPool>()?;

              let user_posts=sqlx::query_as("SELECT * FROM posts WHERE user_id = $1").bind(user_id).fetch_all(pool).await?;

              Ok(user_posts)

        }
        async fn commented_posts(&self,ctx:&Context<'_>,user_id:i32)->async_graphql::Result<Vec<Post>> {

               let pool=ctx.data::<PgPool>()?;

               let commend_posts=sqlx::query_as::<_,Post>("SELECT * FROM posts p JOIN comments c ON p.id=c.post_id where c.user_id=$1").bind(user_id).fetch_all(pool).await?;

               Ok(commend_posts)



        }
        async fn get_post_with_comments(&self, ctx: &Context<'_>, post_id: i32) -> async_graphql::Result<PostWithComments> {
            let pool = ctx.data::<PgPool>()?;
            let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
                .bind(post_id)
                .fetch_one(pool)
                .await?;
           
            let comments=sqlx::query_as::<_,Comment>("SELECT * FROM comments WHERE post_id=$1").bind(post_id).fetch_all(pool).await?;
            
        
            Ok(PostWithComments{post,comments})
        }
        async fn get_users_commented(&self,ctx:&Context<'_>,post_id: i32)->async_graphql::Result<Vec<User>> {

              let pool=ctx.data::<PgPool>()?;

              let users=sqlx::query_as::<_,User>("SELECT DISTINCT u.* FROM users u JOIN comments c ON u.id = c.user_id WHERE c.post_id=$1").bind(post_id).fetch_all(pool).await?;

              Ok(users)
        }
        

}
pub struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn create_post(&self, ctx: &Context<'_>, title: String, content: String, user_id: i32) -> async_graphql::Result<Post> {
        let pool = ctx.data::<PgPool>()?;
        let post = sqlx::query_as::<_, Post>(
            "INSERT INTO posts (title, content, user_id) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(title)
        .bind(content)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(post)
    }
    async fn create_comment(&self,ctx:&Context<'_>,post_id:i32,user_id:i32,content:String)->async_graphql::Result<Comment> {
           let pool=ctx.data::<PgPool>()?;
           let comment=sqlx::query_as::<_,Comment>("INSERT INTO comments (content,post_id,user_id) VALUES ($1,$2,$3) RETURNING *").bind(content).bind(post_id).bind(user_id).fetch_one(pool).await?;

           Ok(comment) 
    }


}