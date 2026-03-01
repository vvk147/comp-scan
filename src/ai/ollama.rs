use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct OllamaClient {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<GenerateOptions>,
}

#[derive(Serialize)]
struct GenerateOptions {
    temperature: f32,
    num_predict: i32,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
    done: bool,
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

impl OllamaClient {
    pub fn new(endpoint: &str, model: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model: model.to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .is_ok()
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        let resp: ModelsResponse = self
            .client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .context("Failed to connect to Ollama")?
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(resp.models.into_iter().map(|m| m.name).collect())
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(GenerateOptions {
                temperature: 0.3,
                num_predict: 1024,
            }),
        };

        let resp: GenerateResponse = self
            .client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Ollama — is it running on localhost:11434?")?
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(resp.response)
    }

    pub async fn analyze_system_data(&self, context: &str) -> Result<String> {
        let prompt = format!(
            "You are CompScan, a local AI system analyst. Analyze the following system data and provide \
             actionable insights. Be concise, specific, and prioritize by impact.\n\n\
             System Data:\n{context}\n\n\
             Provide your analysis as a numbered list of insights, each with:\n\
             1. Category (Performance/Security/Productivity/Habits)\n\
             2. Severity (Critical/Warning/Suggestion/Info)\n\
             3. Finding\n\
             4. Recommended action"
        );

        self.generate(&prompt).await
    }
}
