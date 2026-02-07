// ============================================================================
// Mureka API Client (optional feature)
// ============================================================================

#[cfg(feature = "mureka")]
#[derive(Debug, Clone)]
struct MurekaClient {
    api_key: String,
    client: reqwest::Client,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize)]
struct MurekaGenerateResponse {
    job_id: String,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize)]
struct MurekaJobStatus {
    status: String,  // "pending", "processing", "completed", "failed"
    #[serde(default)]
    songs: Vec<MurekaSong>,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize, Clone)]
struct MurekaSong {
    #[allow(dead_code)]
    id: String,
    #[serde(default)]
    audio_url: Option<String>,
    #[serde(default)]
    title: Option<String>,
}

#[cfg(feature = "mureka")]
impl MurekaClient {
    fn new(api_key: String) -> Self {
        MurekaClient {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn generate_music(&self, prompt: &str, instrumental: bool) -> Result<String, String> {
        let body = if instrumental {
            json!({
                "description": prompt,
                "instrumental": true
            })
        } else {
            json!({
                "lyrics": prompt
            })
        };

        let response = self.client
            .post(format!("{}/v1/song/generate", MUREKA_API_URL))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Mureka request failed: {}", e))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| format!("Failed to read Mureka response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Mureka API error ({}): {}", status, body));
        }

        let result: MurekaGenerateResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Mureka response: {} - Body: {}", e, body))?;

        Ok(result.job_id)
    }

    async fn poll_job(&self, job_id: &str) -> Result<MurekaJobStatus, String> {
        let response = self.client
            .get(format!("{}/v1/song/generate/jobs/{}", MUREKA_API_URL, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| format!("Mureka poll failed: {}", e))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| format!("Failed to read Mureka poll response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Mureka poll error ({}): {}", status, body));
        }

        let result: MurekaJobStatus = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Mureka poll response: {} - Body: {}", e, body))?;

        Ok(result)
    }

    /// Wait for music generation to complete (with timeout)
    async fn wait_for_completion(&self, job_id: &str, max_attempts: u32) -> Result<Vec<MurekaSong>, String> {
        for attempt in 0..max_attempts {
            let status = self.poll_job(job_id).await?;

            match status.status.as_str() {
                "completed" => {
                    return Ok(status.songs);
                }
                "failed" => {
                    return Err("Music generation failed".to_string());
                }
                _ => {
                    // Still processing, wait and retry
                    info!("Mureka job {} status: {} (attempt {}/{})", job_id, status.status, attempt + 1, max_attempts);
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }

        Err("Music generation timed out".to_string())
    }
}

