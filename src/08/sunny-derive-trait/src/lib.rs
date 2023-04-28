pub trait PgCurdStruct {
    fn print(&self);
    fn get_table_name(&self)->&'static str;
    fn get_cache_name(&self)->&'static str;
    fn select(&self) -> String;
    fn insert(&self) -> String;
    fn update(&self,id: i32) -> String;
    fn delete(&self,id: i32) -> String;
    fn get_one_by_id(&self,id: i32) -> String;
}