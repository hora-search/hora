pub struct OpenOptions {}
pub struct DumpOptions {}
pub struct UpdateOptions {}
pub struct QueryOptions {}

pub struct Database {}
impl Database {
    fn open(&mut self, options: OpenOptions) -> Result<(), &'static err_msg> { Err("not implement") }
    fn update(&mut self, options: UpdateOptions) -> Result<(), &'static err_msg> { Err("not implement") }
    fn dump(&mut self, options: DumpOptions) -> Result<(), &'static err_msg> { Err("not implement") }
    fn query(&mut self, options: QueryOptions) -> Result<(), &'static err_msg> { Err("not implement") }
    
    fn create_collection(&mut self, name: str) -> Result<(), &'static err_msg> { Err("not implement") }
    fn delete_collection(&mut self, name: str) -> Result<(), &'static err_msg> { Err("not implement") }
    fn list_collection(&self) -> Result<Vec<str>, &'static err_msg> { Err("not implement") }

    fn close(&mut self) -> Result<(), &'static err_msg> { Err("not implement") }
}