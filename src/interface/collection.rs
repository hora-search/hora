pub struct OpenOptions {}
pub struct Document {}
pub struct QueryOptions {}

pub struct Collection {}
impl Collection {
    fn open(&mut self, options: OpenOptions) -> Result<(), &'static err_msg> { Err("not implement") }
    fn add(&mut self,  doc: Document) -> Result<(), &'static err_msg> { Err("not implement") }
    fn delete(&mut self,  doc: Document) -> Result<(), &'static err_msg> { Err("not implement") }
    fn compact(&mut self) -> Result<(), &'static err_msg> { Err("not implement") }
    fn query(&self, query: QueryOptions) -> Result<(), &'static err_msg> { Err("not implement") }
}