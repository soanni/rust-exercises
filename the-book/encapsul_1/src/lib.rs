pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {
    pub fn add_item(&mut self, i: i32) {
        self.list.push(i);
        self.update_avg();
    }

    pub fn remove_item(&mut self) -> Option<i32> {
        let i = self.list.pop();
        match i {
            Some(i) => {
                self.update_avg();
                Some(i)
            }
            None => None,
        }
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.avg = total as f64 / self.list.len() as f64;
    }
}
