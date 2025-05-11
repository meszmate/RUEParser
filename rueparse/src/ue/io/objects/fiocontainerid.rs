#[derive(Debug)]
pub struct FIoContainerId {
    id: u64,
}
impl FIoContainerId {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
    pub fn to_string(&self) -> String {
        self.id.to_string()
    }
}
