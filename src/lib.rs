use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub author: String,
    pub text: String,
    pub time: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BoardInfo {
    pub name: String,
    pub updated_at: u64,
    pub post_count: usize,
}

#[wasm_bindgen]
pub struct BoardEngine {
    posts_by_board: HashMap<String, Vec<Post>>,
    board_updated_at: HashMap<String, u64>,
}

#[wasm_bindgen]
impl BoardEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BoardEngine {
        BoardEngine {
            posts_by_board: HashMap::new(),
            board_updated_at: HashMap::new(),
        }
    }

    pub fn add_post(&mut self, board_name: String, id: String, author: String, text: String, time: u64) {
        let post = Post { id: id.clone(), author, text, time };
        let posts = self.posts_by_board.entry(board_name.clone()).or_insert_with(Vec::new);
        
        if !posts.iter().any(|p| p.id == id) {
            posts.push(post);
            posts.sort_by(|a, b| a.time.cmp(&b.time));
            
            let current_updated = self.board_updated_at.entry(board_name).or_insert(0);
            if time > *current_updated {
                *current_updated = time;
            }
        }
    }

    pub fn get_sorted_boards_json(&self) -> String {
        let mut boards: Vec<BoardInfo> = self.board_updated_at.iter().map(|(name, &updated_at)| {
            let count = self.posts_by_board.get(name).map(|v| v.len()).unwrap_or(0);
            BoardInfo {
                name: name.clone(),
                updated_at,
                post_count: count,
            }
        }).collect();

        boards.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        serde_json::to_string(&boards).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn get_posts_json(&self, board_name: &str) -> String {
        let empty = Vec::new();
        let posts = self.posts_by_board.get(board_name).unwrap_or(&empty);
        serde_json::to_string(posts).unwrap_or_else(|_| "[]".to_string())
    }
}
