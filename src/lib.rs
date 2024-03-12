use substreams_database_change::pb::database::{table_change::{Operation, PrimaryKey}, DatabaseChanges};
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::{entity_change::{Operation as EntityOperation}, EntityChanges};
use std::{collections::HashMap};



#[substreams::handlers::map]
fn dbout_to_graphout(database_changes: DatabaseChanges) -> Result<EntityChanges, substreams::errors::Error> {
    let table_changes = database_changes.table_changes;
    let mut entity_changes = EntityChanges::default();


    for table_change in table_changes {
        let table_name = table_change.table;
        let operation = table_change.operation;
        let mut fields = table_change.fields;
        let ordinal = table_change.ordinal;

        let primary_key = table_change.primary_key.unwrap();

        let mut primary_key_string = String::new();

        match primary_key {
            PrimaryKey::Pk(s) => {
                primary_key_string = s;
            },
            PrimaryKey::CompositePk(composite_keys) => {
                let mut composite_keys_array = Vec::new(); 
                for (key, value) in composite_keys.keys.iter() {
                    composite_keys_array.push(value);

                    let composite_key_as_field = substreams_database_change::pb::database::Field { 
                        name: key.to_string(), 
                        new_value: value.to_string(),
                        old_value: "".to_string(),
                    };

                    fields.push(composite_key_as_field);
                }
                composite_keys_array.sort();

                for composite_key in composite_keys_array {
                    primary_key_string.push_str(&format!("{}-", composite_key));
                }                
                primary_key_string.pop();

             }
        }

        match operation {
            0 => {
                let entity_change = entity_changes.push_change(table_name, primary_key_string, ordinal, EntityOperation::Unspecified);
                for field in fields {
                    let name = field.name;
                    let new_value = field.new_value;
                    let old_value = field.old_value;
                    entity_change.change(name, (old_value, new_value));
                }
            }
            1 => {
                let entity_change = entity_changes.push_change(table_name, primary_key_string, ordinal, EntityOperation::Create );
                for field in fields {
                    let name = field.name;
                    let new_value = field.new_value;
                    let old_value = field.old_value;
                    entity_change.change(name, (old_value, new_value));
                }
            }
            2 => {
                let entity_change = entity_changes.push_change(table_name, primary_key_string, ordinal, EntityOperation::Update);
                for field in fields {
                    let name = field.name;
                    let new_value = field.new_value;
                    let old_value = field.old_value;
                    entity_change.change(name, (old_value, new_value));
                }
            }
            3 => {
                let entity_change = entity_changes.push_change(table_name, primary_key_string, ordinal, EntityOperation::Delete);
                for field in fields {
                    let name = field.name;
                    let new_value = field.new_value;
                    let old_value = field.old_value;
                    entity_change.change(name, (old_value, new_value));
                }
            }
            _ => {}
        }

    }

    Ok(entity_changes)
}

#[substreams::handlers::map]
fn example_dbout(clock: Clock) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    let id_string: String = clock.id.to_string();
    let timestamp = clock.timestamp.unwrap().to_string();

    let mut composite_keys = HashMap::new();
    composite_keys.insert("id".to_string(), id_string);
    composite_keys.insert("test".to_string(), "test".to_string());

    database_changes.push_change_composite("clock", composite_keys, 0, Operation::Create)
        .change("timestamp", (None, timestamp));

    Ok(database_changes)
}