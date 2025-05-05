use super::EPropertyType;
use crate::objects::{UEnum, UStruct};
use crate::{
    mappings::TypeMappings,
    readers::{FUsmapReader, Reader},
};
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Struct {
    pub context: Option<Rc<RefCell<TypeMappings>>>,
    pub name: String,
    pub super_type: Option<String>,
    pub properties: HashMap<i32, PropertyInfo>,
    pub property_count: i32,
    pub super_struct: OnceCell<Option<Box<Struct>>>,
}

impl Struct {
    pub fn new(
        context: Option<Rc<RefCell<TypeMappings>>>,
        name: String,
        property_count: i32,
    ) -> Self {
        Struct {
            context,
            name,
            super_type: None,
            properties: HashMap::new(),
            property_count,
            super_struct: OnceCell::new(),
        }
    }
    pub fn new_with_super(
        context: Option<Rc<RefCell<TypeMappings>>>,
        name: String,
        super_type: Option<String>,
        properties: HashMap<i32, PropertyInfo>,
        property_count: i32,
    ) -> Self {
        Struct {
            context,
            name,
            super_type,
            properties,
            property_count,
            super_struct: OnceCell::new(),
        }
    }

    pub fn init_super(&mut self) {
        self.super_struct
            .get_or_init(|| match (&self.context, &self.super_type) {
                (Some(ctx), Some(super_name)) => ctx
                    .borrow_mut()
                    .types
                    .borrow_mut()
                    .get(super_name)
                    .map(|s| s.clone()),
                _ => None,
            });
        self.context = None; // To prevent infinity loop
    }

    pub fn parse(
        context: Option<Rc<RefCell<TypeMappings>>>,
        reader: &mut FUsmapReader,
        name_lut: &Vec<String>,
    ) -> Self {
        let name = reader.read_name(name_lut);
        let super_type = Some(reader.read_name(name_lut));

        let property_count: u16 = reader.read_u16().unwrap();
        let serializable_property_count: u16 = reader.read_u16().unwrap();

        let mut properties = HashMap::new();
        for _ in 0..serializable_property_count {
            let prop_info: PropertyInfo = PropertyInfo::parse(reader, &name_lut);
            for i in 0..prop_info.array_size.unwrap_or(0) {
                let mut clone: PropertyInfo = prop_info.clone();
                clone.index = i as i32;
                properties.insert(prop_info.index + i as i32, clone);
            }
        }
        let mut new_super =
            Struct::new_with_super(context, name, super_type, properties, property_count as i32);
        new_super.init_super();
        new_super
    }
}

#[derive(Debug, Clone)]
pub struct PropertyInfo {
    pub index: i32,
    pub name: String,
    pub array_size: Option<u8>,
    pub mapping_type: PropertyType,
}

impl PropertyInfo {
    pub fn new(
        index: i32,
        name: String,
        mapping_type: PropertyType,
        array_size: Option<u8>,
    ) -> Self {
        Self {
            index,
            name,
            array_size,
            mapping_type,
        }
    }
    pub fn parse(reader: &mut FUsmapReader, name_lut: &Vec<String>) -> Self {
        let index: u16 = reader.read_u16().unwrap();
        let arraydim: u8 = reader.read_u8().unwrap();
        let name: String = reader.read_name(&name_lut);
        let p_type: PropertyType = PropertyType::parse(reader, &name_lut);
        PropertyInfo::new(index as i32, name, p_type, Some(arraydim))
    }
}

#[derive(Debug, Clone)]
pub struct PropertyType {
    pub f_type: String,
    pub struct_type: Option<String>,
    pub inner_type: Option<Box<PropertyType>>,
    pub value_type: Option<Box<PropertyType>>,
    pub enum_name: Option<String>,
    pub is_enum_as_byte: Option<bool>,
    pub f_bool: Option<bool>,
    pub f_struct: Option<UStruct>,
    pub f_enum: Option<UEnum>,
}

impl PropertyType {
    pub fn new(
        f_type: String,
        struct_type: Option<String>,
        inner_type: Option<Box<PropertyType>>,
        value_type: Option<Box<PropertyType>>,
        enum_name: Option<String>,
        is_enum_as_byte: Option<bool>,
        f_bool: Option<bool>,
    ) -> Self {
        Self {
            f_type,
            struct_type,
            inner_type,
            value_type,
            enum_name,
            is_enum_as_byte,
            f_bool,
            f_struct: None,
            f_enum: None,
        }
    }
    pub fn parse(reader: &mut FUsmapReader, name_lut: &Vec<String>) -> Self {
        let type_enum = EPropertyType::from_u8(reader.read_u8().unwrap());
        let f_type: String = format!("{:?}", type_enum);
        let mut struct_type: Option<String> = None;
        let mut inner_type: Option<Box<PropertyType>> = None;
        let mut value_type: Option<Box<PropertyType>> = None;
        let mut enum_name: Option<String> = None;
        let is_enum_as_byte: Option<bool> = None;

        match type_enum {
            EPropertyType::EnumProperty => {
                inner_type = Some(Box::new(PropertyType::parse(reader, &name_lut)));
                enum_name = Some(reader.read_name(&name_lut));
            }
            EPropertyType::StructProperty => {
                struct_type = Some(reader.read_name(&name_lut));
            }
            EPropertyType::SetProperty
            | EPropertyType::ArrayProperty
            | EPropertyType::OptionalProperty => {
                inner_type = Some(Box::new(PropertyType::parse(reader, &name_lut)));
            }
            EPropertyType::MapProperty => {
                inner_type = Some(Box::new(PropertyType::parse(reader, &name_lut)));
                value_type = Some(Box::new(PropertyType::parse(reader, &name_lut)));
            }
            _ => {}
        }
        PropertyType::new(
            f_type,
            struct_type,
            inner_type,
            value_type,
            enum_name,
            is_enum_as_byte,
            None,
        )
    }
}
