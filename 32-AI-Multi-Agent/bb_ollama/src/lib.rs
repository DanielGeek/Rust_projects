pub mod chat;

macro_rules! tool_property {
    ($property_name:ident, $key:ident, $type1:ty, $description:expr) => {
        macro_rules! create_name {
            ($name1:ident, $name2:ident) => {
                $$name1name2
            };
        }

        pub struct $property_name {
            $key: create_name!($property_name, Value),
        }

        pub struct $property_nameValue {
            pub property_type: $type1,
            pub description: $description,
        }
    };
}

tool_property!(
    Weather,
    location,
    String,
    "get the weather for a given location. For example San Jose, CA"
);
