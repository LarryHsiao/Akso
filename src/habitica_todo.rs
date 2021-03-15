use crate::akso::{Todo, Todos};
use serde_json::{Value};

const HOST: &str = "https://habitica.com/api/v3";

pub struct HabiticaTodos {
    pub api_key: String,
    pub user_id: String,
}

impl Todos for HabiticaTodos {
    fn all(&self) -> Vec<Box<dyn Todo>> {
        let url = format!("{}/tasks/user", HOST);
        let body = reqwest::blocking::Client::new()
            .get(&url)
            .query(&[("type", "todos")])
            .header("x-api-key", self.api_key.as_str())
            .header("x-api-user", self.user_id.as_str())
            .send().unwrap()
            .text().unwrap();
        if body.is_empty() {
            panic!("Fetch todo list failed.");
        }
        let res: Value = serde_json::from_str(body.as_str()).unwrap();
        let obj = res.as_object().unwrap();
        let task_arr = obj.get("data").unwrap().as_array().unwrap();
        let mut result: Vec<Box<dyn Todo>> = Vec::new();
        for i in 0..task_arr.len() {
            let obj = task_arr.get(i).unwrap();
            result.push(
                Box::new(
                    HabiticaTodo {
                        id: obj.get("id").unwrap().as_str().unwrap().to_string(),
                        title: obj.get("text").unwrap().as_str().unwrap().to_string(),
                        completed: obj.get("completed").unwrap().as_bool().unwrap(),
                    }
                )
            )
        }
        return result;
    }

    fn create(&self) {
        unimplemented!()
    }

    fn finish(&self, id: String) {
        let url = format!("{}/tasks/{}/score/up", HOST, id);
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("x-api-key", self.api_key.as_str())
            .header("x-api-user", self.user_id.as_str())
            .header("Content-Length", 0)
            .send()
            .unwrap();
        if res.status() != 200 {
            panic!("Finish task failed: {}", res.text().unwrap());
        }
    }

    fn delete(&self, id: String) {
        unimplemented!()
    }
}

pub struct HabiticaTodo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Todo for HabiticaTodo {
    fn id(&self) -> String {
        return self.id.to_string();
    }

    fn title(&self) -> String {
        return self.title.to_string();
    }

    fn completed(&self) -> bool {
        return self.completed;
    }
}
