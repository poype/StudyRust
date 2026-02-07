use crate::study_trait::runner::Runner;

pub(crate) struct Cat {
    name: String,
    color: String,
}

impl Runner for Cat {
    fn run(&self) {
        println!(
            "A cat whose name is {} color is {} is running!!!",
            self.name, self.color
        );
    }
}

impl Cat {
    pub fn new(name: String, color: String) -> Cat {
        Cat { name, color }
    }
}

pub(crate) struct Dog {
    name: String,
    kind: String,
}

impl Runner for Dog {
    fn run(&self) {
        println!(
            "A dog of {} whose name is {} is quickly running!!!",
            self.kind, self.name
        );
    }
}

impl Dog {
    pub fn new(name: String, kind: String) -> Dog {
        Dog { name, kind }
    }
}
