use std::io::Error;
use futures::TryStreamExt;

use crate::schemas::post_schema::Post;

use super::DB;

impl DB {

    pub async fn get_all_posts( &self) -> Result<Vec<Post>, Error> {
 
        //let col = &self.collections.posts;
        let col = DB::col_helper::<Post>(&self, "posts");
        let mut cursors = col.find(None, None).await.expect("Error getting list of owners");
        let mut posts: Vec<Post> = Vec::new();
        while let Some(post) = cursors.try_next().await.expect("Error mapping through cursor")
        {
            posts.push(post)
        }
        Ok(posts)
    }

}