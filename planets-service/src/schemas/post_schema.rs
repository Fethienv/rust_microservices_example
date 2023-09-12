//intern crate
use crate::repository::DB;

//extern crate
use std::io::Error;
//use futures::TryStreamExt;
use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
//use async_trait::async_trait;

//Author schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
}


#[derive(InputObject)]
pub struct CreateAuthor {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct FetchAuthor {
    pub _id: String,
}


//Post schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Post{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub draft: bool,
}

#[derive(InputObject)]
pub struct CreatePost {
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub draft: bool,
}

#[derive(InputObject)]
pub struct FetchPost{
    pub _id: String,
}

// #[async_trait]
// pub trait PostDB  {
//     async fn get_posts(&self, ctx: &DB) -> Result<Vec<Post>, Error> {
 
//         let col = DB::col_helper::<Post>(ctx, "posts");
//         let mut cursors = col.find(None, None).await.expect("Error getting list of owners");
//         let mut posts: Vec<Post> = Vec::new();
//         while let Some(post) = cursors.try_next().await.expect("Error mapping through cursor")
//         {
//             posts.push(post)
//         }
//         Ok(posts)
//     } 
// }

impl Post { 
    // pub async fn get_posts( ctx: &DB) -> Result<Vec<Post>, Error> {
 
    //     let col = &ctx.collections.posts;
    //     let mut cursors = col.find(None, None).await.expect("Error getting list of owners");
    //     let mut posts: Vec<Post> = Vec::new();
    //     while let Some(post) = cursors.try_next().await.expect("Error mapping through cursor")
    //     {
    //         posts.push(post)
    //     }
    //     Ok(posts)
    // }

    pub async fn create_post(ctx: &DB, new_post: Post) -> Result<Post, Error> {
        let new_doc = Post {
            _id: None,
            author_id: new_post.author_id.clone(),
            title: new_post.title.clone(),
            content: new_post.content.clone(),
            draft: new_post.draft.clone(),
        };
        let col = DB::col_helper::<Post>(ctx, "posts");
        let data = col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating project");
        let new_post = Post {
            _id: data.inserted_id.as_object_id(),
            author_id: new_post.author_id.clone(),
            title: new_post.title.clone(),
            content: new_post.content.clone(),
            draft: new_post.draft.clone(),
        };
        Ok(new_post)
    }

    pub async fn get_single_post(ctx: &DB, id: &String) -> Result<Post, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let col = DB::col_helper::<Post>(ctx, "posts");
        let post_detail = col
            .find_one(filter, None)
            .await
            .expect("Error getting post's detail");
        Ok(post_detail.unwrap())
    }
}

