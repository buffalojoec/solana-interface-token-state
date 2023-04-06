use syn::{Field, Fields};

pub fn check_fields(defined_fields: &Fields, required_fields: Vec<Field>) -> Result<(), String> {
    let mut defined_field_map = std::collections::HashMap::new();

    match defined_fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                if let Some(ident) = &field.ident {
                    defined_field_map.insert(ident.to_string(), &field.ty);
                }
            }
        }
        _ => return Err("The struct must have named fields".to_string()),
    }

    for required_field in required_fields {
        if let Some(ident) = &required_field.ident {
            let field_name = ident.to_string();
            match defined_field_map.get(&field_name) {
                Some(defined_field_type) => {
                    if **defined_field_type != required_field.ty {
                        return Err(format!(
                            "Field '{}' has an incorrect type. Expected {:?}, found {:?}",
                            field_name, required_field.ty, defined_field_type
                        ));
                    }
                }
                None => {
                    return Err(format!("Field '{}' is missing", field_name));
                }
            }
        }
    }

    Ok(())
}
