//extern crate
use async_graphql::{Context, FieldResult, Object};

//intern crate
use crate::repository::DB;
use crate::schemas::post_schema::{Post, FetchPost, CreatePost};

#[derive(Default)]
pub struct PostQuery;

//#[async_graphql::Object]
#[Object(extends)]
impl PostQuery {

    async fn get_all_posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
        let db = &ctx.data_unchecked::<DB>();
        let posts = db.get_all_posts().await.unwrap();
        Ok(posts)
    }

    async fn get_post(&self, ctx: &Context<'_>, input: FetchPost) -> FieldResult<Post> {
        let db = &ctx.data_unchecked::<DB>();
        let post = Post::get_single_post(db, &input._id).await.unwrap();
        Ok(post)
    }

}

pub struct PostMutation;

#[Object]
impl PostMutation {
    //post mutation

    async fn create_post(&self, ctx: &Context<'_>, input: CreatePost) -> FieldResult<Post> {
        let db = &ctx.data_unchecked::<DB>();
        let new_post = Post {
            _id: None,
            author_id: input.author_id,
            title: input.title,
            content: input.content,
            draft: input.draft,
        };
        let project = Post::create_post(db, new_post).await.unwrap();
        Ok(project)
    }
}
