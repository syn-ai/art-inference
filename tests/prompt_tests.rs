use comfyui_api_proxy::prompt::constructor::PromptConstructor;
use serde_json::json;

#[test]
fn test_construct_prompt() {
    let mut constructor = PromptConstructor::new();
    let template = json!({
        "node1": {
            "inputs": {
                "text": "{{text}}",
                "value": "{{value}}"
            }
        }
    });
    let inputs = json!({
        "text": "Hello, world!",
        "value": 42
    });

    let result = constructor.construct_prompt(&template, &inputs);
    assert!(result.is_ok());
    let constructed_prompt = result.unwrap();
    assert_eq!(
        constructed_prompt,
        json!({
            "node1": {
                "inputs": {
                    "text": "Hello, world!",
                    "value": 42
                }
            }
        })
    );
}