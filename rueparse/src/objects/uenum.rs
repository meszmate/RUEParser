use super::FName;

#[derive(Debug, Clone)]
pub enum ECppForm {
    Regular,
    Namespaced,
    EnumClass,
}

#[derive(Debug, Clone)]
pub struct UEnum {
    pub names: Vec<(FName, i64)>,
    pub cpp_form: ECppForm,
}
