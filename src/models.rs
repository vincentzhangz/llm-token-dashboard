use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchContextCost {
    #[serde(default)]
    pub search_context_size_high: Option<f64>,
    #[serde(default)]
    pub search_context_size_low: Option<f64>,
    #[serde(default)]
    pub search_context_size_medium: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    #[serde(default)]
    pub input_cost_per_token: Option<f64>,
    #[serde(default)]
    pub output_cost_per_token: Option<f64>,
    #[serde(default)]
    pub input_cost_per_audio_token: Option<f64>,
    #[serde(default)]
    pub output_cost_per_reasoning_token: Option<f64>,
    #[serde(default)]
    pub cache_creation_input_token_cost: Option<f64>,
    #[serde(default)]
    pub cache_read_input_token_cost: Option<f64>,
    #[serde(default)]
    pub input_cost_per_pixel: Option<f64>,
    #[serde(default)]
    pub output_cost_per_pixel: Option<f64>,
    #[serde(default)]
    pub output_cost_per_image: Option<f64>,
    #[serde(default)]
    pub code_interpreter_cost_per_session: Option<f64>,
    #[serde(default)]
    pub computer_use_input_cost_per_1k_tokens: Option<f64>,
    #[serde(default)]
    pub computer_use_output_cost_per_1k_tokens: Option<f64>,
    #[serde(default)]
    pub file_search_cost_per_1k_calls: Option<f64>,
    #[serde(default)]
    pub file_search_cost_per_gb_per_day: Option<f64>,
    #[serde(default)]
    pub vector_store_cost_per_gb_per_day: Option<f64>,
    #[serde(default)]
    pub search_context_cost_per_query: Option<SearchContextCost>,
    #[serde(default)]
    pub max_tokens: Option<i64>,
    #[serde(default)]
    pub max_input_tokens: Option<i64>,
    #[serde(default)]
    pub max_output_tokens: Option<i64>,
    #[serde(default)]
    pub litellm_provider: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub deprecation_date: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub supports_function_calling: Option<bool>,
    #[serde(default)]
    pub supports_parallel_function_calling: Option<bool>,
    #[serde(default)]
    pub supports_vision: Option<bool>,
    #[serde(default)]
    pub supports_audio_input: Option<bool>,
    #[serde(default)]
    pub supports_audio_output: Option<bool>,
    #[serde(default)]
    pub supports_prompt_caching: Option<bool>,
    #[serde(default)]
    pub supports_reasoning: Option<bool>,
    #[serde(default)]
    pub supports_response_schema: Option<bool>,
    #[serde(default)]
    pub supports_system_messages: Option<bool>,
    #[serde(default)]
    pub supports_web_search: Option<bool>,
    #[serde(default)]
    pub supports_assistant_prefill: Option<bool>,
    #[serde(default)]
    pub supports_pdf_input: Option<bool>,
    #[serde(default)]
    pub supports_tool_choice: Option<bool>,
    #[serde(default)]
    pub supported_regions: Option<Vec<String>>,
}
