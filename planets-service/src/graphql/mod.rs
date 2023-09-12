//extern crate
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema, MergedObject};

//intern crate
use crate::repository::DB;
use crate::schemas::post_schema::{Post, FetchPost, CreatePost};

mod post_ql;

use crate::graphql::post_ql::PostQuery;

#[derive(MergedObject, Default)]
pub struct RootQuery(PostQuery);

//#[async_graphql::Object]
// #[Object(extends)]
// impl RootQuery {

//     async fn get_post_by_id(&self, ctx: &Context<'_>, input: FetchPost) -> FieldResult<Post> {
//         let db = &ctx.data_unchecked::<DB>();
//         let post = Post::get_single_post(db, &input._id).await.unwrap();
//         Ok(post)
//     }

//     //async fn posts(&self) -> PostQuery { PostQuery }

// }

pub struct RootMutation;

#[Object]
impl RootMutation {
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

pub type AppSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub async fn create_schema() -> Schema<RootQuery, RootMutation, EmptySubscription> {
    let db = DB::new("rustDB").await;
    let schema_data = Schema::build(RootQuery::default(), RootMutation, EmptySubscription)
        .data(db)
        .finish();
        schema_data
}






// pub fn create_schema_with_context(pool: DB) -> Schema<RootQuery, EmptyMutation, EmptySubscription> {
//      let arc_pool = std::sync::Arc::new(pool);
// //     let cloned_pool = Arc::clone(&arc_pool);
// //     let details_data_loader = DataLoader::new(DetailsLoader { pool: cloned_pool }, actix_rt::spawn).max_batch_size(10);

// //     let kafka_consumer_counter = Mutex::new(0);

//     Schema::build(RootQuery, EmptyMutation, EmptySubscription)
// //         // limits are commented out, because otherwise introspection query won't work
// //         // .limit_depth(3)
// //         // .limit_complexity(15)
//          .data(arc_pool)
// //         .data(details_data_loader)
// //         .data(kafka::create_producer())
// //         .data(kafka_consumer_counter)
// //         .enable_subscription_in_federation()
//          .finish()
// }