use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Nightly,
    Beta,
    Stable,
}

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::EditType"]
pub enum EditType {
    Add,
    Edit,
    Delete,
}

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::SizeUnit"]
pub enum SizeUnit {
    Kb,
    Mb,
    Gb,
}
