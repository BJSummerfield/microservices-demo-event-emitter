use rand::Rng;
use std::time::Duration;
use tokio::time::interval;

use crate::graphql::{Mutations, Queries};

pub struct Emitter {
    max_users: u32,
    request_rate: u32,
    running: bool,
    users: Vec<String>,
}

impl Emitter {
    pub fn new(max_users: u32, request_rate: u32) -> Self {
        Self {
            max_users,
            request_rate,
            running: false,
            users: Vec::new(),
        }
    }

    pub async fn start(&mut self) {
        if self.running {
            println!("Emitter is already running");
            return;
        }
        self.running = true;
        self.refresh_users().await.unwrap();
        self.emit().await;
    }

    pub fn stop(&mut self) {
        if !self.running {
            println!("Emitter is not running");
            return;
        }
        self.running = false;
    }

    async fn refresh_users(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = Queries::get_all_users().await?;
        if let Some(data) = response["data"]["getAllUsers"].as_array() {
            self.users = data
                .iter()
                .filter_map(|user| user["id"].as_str().map(|s| s.to_string()))
                .collect();
        }
        Ok(())
    }

    async fn emit(&mut self) {
        let mut interval = interval(Duration::from_millis(self.request_rate.into()));

        while self.running {
            println!("UserList: {:?}", self.users);
            interval.tick().await;

            match self.users.len() {
                0 => self.create_user_action().await,
                len if len >= self.max_users as usize => self.update_or_delete_user_action().await,
                _ => self.random_user_action().await,
            }
        }
    }

    async fn update_or_delete_user_action(&mut self) {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..100);

        match choice {
            // 0..=39 => self.update_name_action().await,
            // 40..=79 => self.update_birthday_action().await,
            _ => self.delete_user_action().await,
        }
    }

    async fn random_user_action(&mut self) {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..100);

        match choice {
            0..=24 => self.create_user_action().await,
            // 25..=49 => self.update_name_action().await,
            // 50..=74 => self.update_birthday_action().await,
            _ => self.delete_user_action().await,
        }
    }

    async fn create_user_action(&mut self) {
        match Mutations::create_user().await {
            Ok(response) => {
                if let Some(id) = response["data"]["createUser"]["id"].as_str() {
                    self.users.push(id.to_string());
                }
            }
            Err(e) => println!("Failed to create user: {:?}", e),
        }
    }

    async fn update_birthday_action(&self) {
        let user_id = self.choose_random_user();
        if let Err(e) = Mutations::update_birthday(user_id.clone()).await {
            println!("Failed to update user birthday: {:?}", e);
        }
    }

    async fn update_name_action(&self) {
        let user_id = self.choose_random_user();
        if let Err(e) = Mutations::update_name(user_id.clone()).await {
            println!("Failed to update user birthday: {:?}", e);
        }
    }

    async fn delete_user_action(&mut self) {
        let user_id = self.choose_random_user();
        match Mutations::delete_user(user_id.clone()).await {
            Ok(response) => {
                if let Some(id) = response["data"]["deleteUser"]["id"].as_str() {
                    self.users.retain(|u| u != id);
                }
            }
            Err(e) => println!("Failed to delete user: {:?}", e),
        }
    }
    fn choose_random_user(&self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.users.len());
        self.users[index].clone()
    }
}
