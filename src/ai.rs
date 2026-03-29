use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "systemInstruction")]
    system_instruction: GeminiSystemInstruction,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct Candidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

pub struct AiClient {
    client: Client,
    api_key: String,
    pub history: Vec<Message>,
}

impl AiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            history: Vec::new(),
        }
    }

    fn system_prompt(&self) -> String {
        let os = std::env::consts::OS;
        format!(
            "You are an AI terminal assistant on {}. \
             Convert natural language into shell commands. \
             Return ONLY the raw command — no explanation, no backticks, no markdown. \
             Use cmd.exe syntax on Windows, bash on macOS/Linux. \
             If the input is already a valid command, return it unchanged. \
             Use conversation history for context.",
            os
        )
    }

    pub async fn translate(
        &mut self,
        input: &str,
        on_token: impl Fn(String),
    ) -> Result<String, String> {
        self.history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // Convert history to Gemini format (roles must alternate user/model)
        let contents: Vec<GeminiContent> = self.history.iter().map(|m| GeminiContent {
            role: if m.role == "assistant" { "model".to_string() } else { "user".to_string() },
            parts: vec![GeminiPart { text: m.content.clone() }],
        }).collect();

        let body = GeminiRequest {
            contents,
            system_instruction: GeminiSystemInstruction {
                parts: vec![GeminiPart { text: self.system_prompt() }],
            },
            generation_config: GenerationConfig {
                max_output_tokens: 256,
                temperature: 0.0,
            },
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );

        let res = self.client
            .post(&url)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let response: GeminiResponse = res.json().await.map_err(|e| e.to_string())?;

        // Check for API errors
        if let Some(err) = response.error {
            return Err(format!("API error {}: {}", status, err.message));
        }

        let cmd = response
            .candidates
            .and_then(|c| c.into_iter().next())
            .and_then(|c| c.content.parts.into_iter().next())
            .map(|p| p.text.trim().to_string())
            .unwrap_or_default();

        // Stream the full command as a single token to the frontend
        on_token(cmd.clone());

        self.history.push(Message {
            role: "assistant".to_string(),
            content: cmd.clone(),
        });

        if self.history.len() > 20 {
            self.history.drain(0..2);
        }

        Ok(cmd)
    }

    pub async fn fix_command(&mut self, failed: &str, error: &str, on_token: impl Fn(String)) -> Result<String, String> {
        let prompt = format!(
            "Command `{}` failed with:\n{}\nGive me a corrected command only.",
            failed, error
        );
        self.translate(&prompt, on_token).await
    }
}