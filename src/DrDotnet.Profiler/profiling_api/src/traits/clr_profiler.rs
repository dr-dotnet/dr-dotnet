use uuid::Uuid;

pub trait ClrProfiler {
    fn new() -> Self;
    fn get_guid() -> Uuid;
    fn get_name() -> String;
    fn get_description() -> String;
}
