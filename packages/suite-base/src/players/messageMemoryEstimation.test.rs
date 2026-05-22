```rust
use std::collections::HashMap;

const OBJECT_BASE_SIZE: usize = 12; // Base size of an object in bytes

// Function to estimate the size of a message object based on a schema and data
fn estimate_message_object_size(datatypes: &HashMap<String, HashMap<String, serde_json::Value>>, object: &serde_json::Value) -> usize {
    let mut total_size = OBJECT_BASE_SIZE;

    match object {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if let Some(definition) = datatypes.get(&key).and_then(|def| def.get("type")) {
                    match definition.as_str() {
                        "int32" => total_size += 4,
                        "bool" => total_size += 1,
                        "float64" => total_size += 8,
                        _ => {
                            panic!("Unknown type '{}'", key);
                        }
                    }
                } else if let serde_json::Value::Array(array) = value {
                    let array_length = array.len();
                    total_size += OBJECT_BASE_SIZE + array_length * 4; // Base size plus each element pointer
                }
            }
        },
        _ => panic!("Unknown type in object"),
    }

    total_size
}

// Function to estimate the size of a message field based on a schema and data
fn estimate_message_field_sizes(datatypes: &HashMap<String, HashMap<String, serde_json::Value>>, field_name: &str, object: &serde_json::Value) -> usize {
    let definition = datatypes.get(field_name).and_then(|def| def.get("type")).unwrap();
    match definition.as_str() {
        "int32" => 4,
        "bool" => 1,
        "float64" => 8,
        _ => {
            panic!("Unknown type '{}'", field_name);
        }
    }
}

// Test cases for memory estimation by schema
describe!("memoryEstimationBySchema", () => {
    it("size for empty schema is greater than 0", () => {
        let datatypes = HashMap::new();
        let object = serde_json::Value::Object(HashMap::new());
        let expected_size = 12; // Base size of an object
        expect(estimate_message_object_size(&datatypes, &object)).toBeGreaterThan(expected_size);
    });

    it("throws an error for an unknown type", () => {
        let datatypes = HashMap::new();
        let object = serde_json::Value::Object(HashMap::from([["foo", { definitions: [] }]]));
        expect(() => estimate_message_object_size(&datatypes, &object)).toThrow("Type 'UnknownType' not found in definitions");
    });

    it("handles complex types with arrays of unknown primitive type", () => {
        let datatypes = HashMap::new();
        let object = serde_json::Value::Object(HashMap::from([["ComplexType", { definitions: [{ type: "unknownType", name: "field1", isArray: true, arrayLength: 2 }] }]]));
        expect(() => estimate_message_object_size(&datatypes, &object)).toThrow("Unknown primitive type unknownType");
    });

    it("size for a complex schema is calculated correctly", () => {
        let datatypes = HashMap::new();
        let object = serde_json::Value::Object(HashMap::from([
            ["ComplexType", { definitions: [
                { type: "int32", name: "field1" },
                { type: "bool", name: "field2" },
              ] },
        ])));
        let expected_size = 20; // 3 * pointers + 1 x 4 byte smi + 1 x pointer to boolean
        expect(estimate_message_object_size(&datatypes, &object)).toBeGreaterThan(expected_size);
    });

    it("handles complex types with arrays", () => {
        let datatypes = HashMap::new();
        let object = serde_json::Value::Object(HashMap::from([
            ["ComplexType", { definitions: [
                { type: "int32", name: "field1" },
                { type: "float64", name: "field2" },
                { type: "bool", name: "field3", isArray: true, arrayLength: 10 },
              ] },
        ])));
        let expected_size = 90; // 52 + 1 x 8 byte smi + 4 x boolean pointers
        expect(estimate_message_object_size(&datatypes, &object)).toBeGreaterThan(expected_size);
    });

    it.each([
        { num_floats: 2, num_ints: 2, measured_size: 52, tolerance_percent: 5 },
        { num_floats: 10, num_ints: 10, measured_size: 212, tolerance_percent: 5 },
        { num_floats: 10, num_ints: 10, measured_size: 212, tolerance_percent: 5 },
        { num_floats: 50, num_ints: 50, measured_size: 1012, tolerance_percent: 5 },
        { num_floats: 100, num_ints: 100, measured_size: 2012, tolerance_percent: 5 },
        { num_floats: 200, num_ints: 200, measured_size: 4028, tolerance_percent: 5 },
        { num_floats: 400, num_ints: 400, measured_size: 8024, tolerance_percent: 5 },
        { num_floats: 550, num_ints: 550, measured_size: 31220, tolerance_percent: 5 },
        { num_floats: 1000, num_ints: 1000, measured_size: 61196, tolerance_percent: 5 },
        { num_floats: 2000, num_ints: 2000, measured_size: 122348, tolerance_percent: 5 },
    ])(
        "matches the size of objects with int + double fields measured with chrome devtools",
        ({ num_floats, num_ints, measured_size, tolerance_percent }) => {
            let datatypes = HashMap::new();
            let mut large_object: serde_json::Value = serde_json::from_str(r#"{"field1":[1,2,3,4,5,6],"field2":true,"field3":1.23}"#).unwrap();
            let expected_size = 12 + 52 + 4 * (num_floats as usize) + 4 * num_ints;
            expect(estimate_message_object_size(&datatypes, &large_object)).toBeGreaterThan(expected_size - measured_size * tolerance_percent / 100);
        },
    );

    it("sum of field sizes matches total object size", () => {
        let datatypes = HashMap::new();
        let large_object: serde_json::Value = serde_json::from_str(r#"{"field1":[1,2,3,4,5,6],"field2":true,"field3":1.23}"#).unwrap();
        let field_sizes = estimate_message_field_sizes(&datatypes, "field1", &large_object);
        let msg_size_in_bytes = estimate_message_object_size(&datatypes, &large_object);
        let expected_size = field_sizes + msg_size_in_bytes - large_object.get("obj").map_or(0, |obj| estimate_message_object_size(&datatypes, obj));
        expect(field_sizes).toEqual(expected_size);
    });
});

// Test cases for memory estimation by object
describe!("memoryEstimationByObject", () => {
    it("estimates the size of an empty object to be greater than 0", () => {
        let expected_size = 12; // Base size of an object
        expect(estimate_object_size(&HashMap::new())).toBeGreaterThan(expected_size);
    });

    it("estimates size of null object to be 8 bytes", () => {
        let expected_size = 8; // Base size plus pointer to null
        expect(estimate_object_size(&serde_json::Value::Null)).toBeGreaterThan(expected_size);
    });

    it("estimates size of true object to be 1 byte", () => {
        let expected_size = 1; // Base size plus pointer to boolean true
        expect(estimate_object_size(&serde_json::Value::Bool(true))).toBeGreaterThan(expected_size);
    });

    it("estimates size of false object to be 1 byte", () => {
        let expected_size = 1; // Base size plus pointer to boolean false
        expect(estimate_object_size(&serde_json::Value::Bool(false))).toBeGreaterThan(expected_size);
    });

    it("estimates size of number object to be 8 bytes", () => {
        let expected_size = 8; // Base size plus pointer to number
        expect(estimate_object_size(&serde_json::Value::Number(12345.6789))).toBeGreaterThan(expected_size);
    });

    it("estimates size of string object to be 20 bytes", () => {
        let expected_size = 20; // Base size plus pointer to string
        expect(estimate_object_size(&serde_json::Value::String(String::from("abcdef")))).toBeGreaterThan(expected_size);
    });

    it("estimates size of array object to be 48 bytes", () => {
        let expected_size = 48; // Base size plus pointer to array
        let array: serde_json::Value = serde_json::from_str(r#"["1","2","3","4","5","6"]"#).unwrap();
        expect(estimate_object_size(&array)).toBeGreaterThan(expected_size);
    });

    it("estimates size of array object with nested objects to be 96 bytes", () => {
        let expected_size = 96; // Base size plus pointer to array with nested objects
        let obj: serde_json::Value = serde_json::from_str(r#"{"n":[1,2,3,4,5,6],"str":"abcdef"}"#).unwrap();
        expect(estimate_object_size(&obj)).toBeGreaterThan(expected_size);
    });

    it("estimates size of object with nested array to be 120 bytes", () => {
        let expected_size = 120; // Base size plus pointer to object with nested array
        let obj: serde_json::Value = serde_json::from_str(r#"{"obj":{"n":[1,2,3,4,5,6],"str":"abcdef"}}"#).unwrap();
        expect(estimate_object_size(&obj)).toBeGreaterThan(expected_size);
    });
});
```