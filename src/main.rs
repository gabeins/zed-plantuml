use serde_json::json;
use std::io::{self};

/// Struct for handling PlantUML-related commands and actions.
struct PlantUMLHandler;

impl PlantUMLHandler {
    /// This Registers the command for the Zed extension and starts the main loop.
    pub fn start() {
        // Register the command for previewing PlantUML.
        let command = json!({
            "command": "extension.previewPlantUML",
            "handler": "preview_plantuml"
        });
        println!("{}", command);

        // Command handler loop to process incoming requests.
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input");
                continue;
            }

            if let Ok(request) = serde_json::from_str::<serde_json::Value>(&input) {
                // Check if the received command matches "preview_plantuml".
                if request["command"] == "preview_plantuml" {
                    if let Some(editor_content) = request["content"].as_str() {
                        Self::handle_preview(editor_content);
                    } else {
                        Self::respond_with_alert("No active editor found. Please open a file.");
                    }
                }
            }
        }
    }

    /// Handles the previewing of PlantUML diagrams.
    fn handle_preview(content: &str) {
        if content.trim().is_empty() {
            Self::respond_with_alert("Editor content is empty. Nothing to preview.");
            return;
        }

        // Encode the content for URL compatibility and create the PlantUML diagram URL.
        // Note: This example uses a public PlantUML server. For local or private servers...
        let encoded_content = urlencoding::encode(content);
        let plantuml_server = "https://www.plantuml.com/plantuml/svg/";
        let diagram_url = format!("{}{}", plantuml_server, encoded_content);

        // Send a response to Zed to open the diagram in a panel for preview.
        let response = json!({
            "type": "panel",
            "title": "PlantUML Preview",
            "url": diagram_url
        });

        println!("{}", response);
    }

    /// Utility function to send alert messages to Zed, Duh
    fn respond_with_alert(message: &str) {
        let response = json!({
            "type": "alert",
            "message": message
        });

        println!("{}", response);
    }
}

fn main() {
    println!(
        "{}",
        json!({
            "command": "extension.previewPlantUML",
            "handler": "preview_plantuml"
        })
    );

    PlantUMLHandler::start();
}
