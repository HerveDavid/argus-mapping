use serde::{de::Error, Deserialize};
use serde_json::Value;

pub fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: crate::extensions::JsonSchema + for<'de> Deserialize<'de> + schemars::JsonSchema,
{
    // Parse as Value for initial validation
    let value: Value = serde_json::from_str(json)?;

    // Make sure it's an object
    let obj = value
        .as_object()
        .ok_or_else(|| serde_json::Error::custom("JSON input must be an object"))?;

    // Check for unexpected fields
    let schema_fields = T::fields_json();
    for field in obj.keys() {
        if !schema_fields.contains(&field.to_string()) {
            return Err(serde_json::Error::custom(format!(
                "Unexpected field: {}",
                field
            )));
        }
    }

    // Get the JSON schema for T
    let schema = schemars::schema_for!(T);
    let schema_value = serde_json::to_value(&schema).map_err(|e| {
        serde_json::Error::custom(format!("Failed to convert schema to value: {}", e))
    })?;

    // Utilisation correcte de jsonschema
    let instance = &value;
    let result = jsonschema::validate(&schema_value, instance);

    if let Err(errors) = result {
        return Err(serde_json::Error::custom(format!(
            "Schema validation failed: {}",
            errors
        )));
    }

    // If validation passes, deserialize the input
    serde_json::from_value(value)
}
