use crate::models::ModelPricing;
use std::collections::HashMap;

pub async fn fetch_model_prices(
) -> Result<HashMap<String, ModelPricing>, Box<dyn std::error::Error>> {
    let url: &str = "https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json";

    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    let all_prices: HashMap<String, serde_json::Value> = serde_json::from_str(&text)?;

    let mut filtered_prices: HashMap<String, ModelPricing> = HashMap::new();

    for (key, value) in all_prices {
        if key == "sample_spec" {
            continue;
        }

        if let Ok(pricing) = serde_json::from_value::<ModelPricing>(value) {
            if pricing.input_cost_per_token.is_some()
                && pricing.output_cost_per_token.is_some()
                && (pricing
                    .mode
                    .as_ref()
                    .map_or(true, |m| m == "chat" || m == "completion"))
            {
                filtered_prices.insert(key, pricing);
            }
        }
    }

    Ok(filtered_prices)
}
