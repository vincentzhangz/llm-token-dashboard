use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashMap;
use thaw::*;

use crate::models::ModelPricing;
use crate::utils::fetch_model_prices;

#[component]
pub fn ModelPriceListPage() -> impl IntoView {
    let (model_prices, set_model_prices) = signal(HashMap::<String, ModelPricing>::new());
    let (search_query, set_search_query) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    Effect::new(move |_| {
        spawn_local(async move {
            match fetch_model_prices().await {
                Ok(prices) => {
                    set_model_prices.set(prices);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load model prices: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    let filtered_models = move || {
        let search = search_query.get().to_lowercase();
        let prices = model_prices.get();
        let mut model_list: Vec<(String, ModelPricing)> = if search.is_empty() {
            prices.into_iter().collect()
        } else {
            prices
                .into_iter()
                .filter(|(k, _)| k.to_lowercase().contains(&search))
                .collect()
        };
        model_list.sort_by(|a, b| a.0.cmp(&b.0));
        model_list
    };

    view! {
        <div class="min-h-screen bg-gray-100 p-6">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-4xl font-bold mb-2">
                        "Model Price List"
                    </h1>
                    <p class="text-gray-600 text-lg">
                        "Browse and compare pricing for various LLM models"
                    </p>
                    <div class="mt-4">
                        <a href="/" class="mr-4 text-blue-500 hover:text-blue-700">
                            "Cost Calculator"
                        </a>
                        <a href="/list-model-prices" class="mr-4 text-blue-500 hover:text-blue-700">
                            "Model Price List"
                        </a>
                        <a href="/info" class="text-blue-500 hover:text-blue-700">
                            "Info"
                        </a>
                    </div>
                </div>

                {move || {
                    if loading.get() {
                        view! {
                            <div class="text-center py-12">
                                <Spinner size=SpinnerSize::Huge />
                                <p class="mt-4 text-gray-600">"Loading model prices..."</p>
                            </div>
                        }.into_any()
                    } else if let Some(err) = error.get() {
                        view! {
                            <MessageBar intent=MessageBarIntent::Error>
                                <MessageBarBody>
                                    <div>
                                        <strong>"Error: "</strong>
                                        {err}
                                    </div>
                                </MessageBarBody>
                            </MessageBar>
                        }.into_any()
                    } else {
                        view! {
                            <Card class="bg-white shadow-md rounded-lg w-full">
                                <div class="p-6">
                                    <div class="mb-6">
                                        <Input
                                            class="w-full"
                                            placeholder="Search models by name..."
                                            on:input=move |ev| {
                                                set_search_query.set(event_target_value(&ev));
                                            }
                                        />
                                    </div>

                                    <div class="overflow-x-auto">
                                        <table class="w-full border-collapse">
                                            <thead>
                                                <tr class="bg-gray-200">
                                                    <th class="border border-gray-300 px-4 py-2 text-left font-semibold">"Model Name"</th>
                                                    <th class="border border-gray-300 px-4 py-2 text-right font-semibold">"Input Cost ($/token)"</th>
                                                    <th class="border border-gray-300 px-4 py-2 text-right font-semibold">"Output Cost ($/token)"</th>
                                                    <th class="border border-gray-300 px-4 py-2 text-right font-semibold">"Max Tokens"</th>
                                                    <th class="border border-gray-300 px-4 py-2 text-center font-semibold">"Provider"</th>
                                                    <th class="border border-gray-300 px-4 py-2 text-center font-semibold">"Mode"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {move || {
                                                    let models = filtered_models();
                                                    if models.is_empty() {
                                                        view! {
                                                            <tr>
                                                                <td colspan="6" class="border border-gray-300 px-4 py-8 text-center text-gray-500">
                                                                    "No models found"
                                                                </td>
                                                            </tr>
                                                        }.into_any()
                                                    } else {
                                                        models.into_iter().map(|(model_name, pricing)| {
                                                            view! {
                                                                <tr class="hover:bg-gray-50">
                                                                    <td class="border border-gray-300 px-4 py-2 font-mono text-sm">
                                                                        {model_name}
                                                                    </td>
                                                                    <td class="border border-gray-300 px-4 py-2 text-right">
                                                                        {format!("${:.8}", pricing.input_cost_per_token.unwrap_or(0.0))}
                                                                    </td>
                                                                    <td class="border border-gray-300 px-4 py-2 text-right">
                                                                        {format!("${:.8}", pricing.output_cost_per_token.unwrap_or(0.0))}
                                                                    </td>
                                                                    <td class="border border-gray-300 px-4 py-2 text-right">
                                                                        {pricing.max_tokens.map(|t| t.to_string()).unwrap_or_else(|| "N/A".to_string())}
                                                                    </td>
                                                                    <td class="border border-gray-300 px-4 py-2 text-center text-sm">
                                                                        {pricing.litellm_provider.unwrap_or_else(|| "N/A".to_string())}
                                                                    </td>
                                                                    <td class="border border-gray-300 px-4 py-2 text-center text-sm">
                                                                        {pricing.mode.unwrap_or_else(|| "N/A".to_string())}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view().into_any()
                                                    }
                                                }}
                                            </tbody>
                                        </table>
                                    </div>

                                    <div class="mt-4 text-sm text-gray-600 text-center">
                                        {move || format!("Showing {} models", filtered_models().len())}
                                    </div>
                                </div>
                            </Card>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
