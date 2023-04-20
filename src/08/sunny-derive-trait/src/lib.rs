pub trait PgCurdStruct {
    fn print(&self);
    fn get_table_name(&self)->&'static str;
    fn select(&self) -> String;
    fn insert(&self) -> String;
    fn update(&self,id: i32) -> String;
    fn delete(&self,id: i32) -> String;
    //fn get_one_by_id<'a,'b>(&self,state: &'a DbState,id:i32) -> Result<T> ;
}