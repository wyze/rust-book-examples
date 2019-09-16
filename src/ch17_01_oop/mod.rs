pub struct AveragedCollection {
    average: f64,
    list: Vec<i32>,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        if let Some(value) = result {
            self.update_average();
        }

        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();

        self.average = f64::from(total) / self.list.len() as f64;
    }
}

pub fn run() {}
