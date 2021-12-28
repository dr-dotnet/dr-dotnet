use uuid::Uuid;

pub trait ClrProfiler {
    fn new() -> Self;
    fn clsid(&self) -> &Uuid;
}
