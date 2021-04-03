pub trait Todo {
    fn id(&self) -> String;
    fn title(&self) -> String;
    fn completed(&self) -> bool;
}

pub trait Todos {
    fn all(&self) -> Vec<Box<dyn Todo>>;
    fn create(&self, title: String);
    fn finish(&self, id: String);
    fn delete(&self, id: String);
    fn do_first(&self, id: String);
    fn do_last(&self, id:String);
}
