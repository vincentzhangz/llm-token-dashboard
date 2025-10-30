use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_debounce_fn_with_arg;
use std::collections::HashMap;
use thaw::*;

use crate::models::ModelPricing;
use crate::utils::{estimate_tokens, fetch_model_prices, preload_tokenizer};

#[component]
pub fn HomePage() -> impl IntoView {
    let (model_prices, set_model_prices) = signal(HashMap::<String, ModelPricing>::new());
    let (selected_model, set_selected_model) = signal(String::from("claude-sonnet-4-5"));
    let (model_search, set_model_search) = signal(String::new());
    let (input_text, set_input_text) = signal(String::new());
    let (output_text, set_output_text) = signal(String::new());
    let (debounced_input_text, set_debounced_input_text) = signal(String::new());
    let (debounced_output_text, set_debounced_output_text) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    let debounce_input = use_debounce_fn_with_arg(
        move |text: String| {
            set_debounced_input_text.set(text);
        },
        300.0,
    );

    let debounce_output = use_debounce_fn_with_arg(
        move |text: String| {
            set_debounced_output_text.set(text);
        },
        300.0,
    );

    Effect::new(move |_| {
        let text = input_text.get();
        debounce_input(text);
    });

    Effect::new(move |_| {
        let text = output_text.get();
        debounce_output(text);
    });

    Effect::new(move |_| {
        spawn_local(async move {
            preload_tokenizer();
        });
    });

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

    let calculation = move || {
        let input = debounced_input_text.get();
        let output = debounced_output_text.get();
        let model_name = selected_model.get();

        let input_tokens = estimate_tokens(&input);
        let output_tokens = estimate_tokens(&output);
        let total_tokens = input_tokens + output_tokens;

        let prices = model_prices.get();
        let model_pricing = prices.get(&model_name);

        let (input_cost, output_cost, total_cost) = if let Some(pricing) = model_pricing {
            let input_cost = pricing.input_cost_per_token.unwrap_or(0.0) * input_tokens as f64;
            let output_cost = pricing.output_cost_per_token.unwrap_or(0.0) * output_tokens as f64;
            let total = input_cost + output_cost;
            (input_cost, output_cost, total)
        } else {
            (0.0, 0.0, 0.0)
        };

        (
            input_tokens,
            output_tokens,
            total_tokens,
            input_cost,
            output_cost,
            total_cost,
        )
    };

    let filtered_models = move || {
        let search = model_search.get().to_lowercase();
        let prices = model_prices.get();
        let mut model_list: Vec<String> = if search.is_empty() {
            prices.keys().cloned().collect()
        } else {
            prices
                .keys()
                .filter(|k| k.to_lowercase().contains(&search))
                .cloned()
                .collect()
        };
        model_list.sort();
        model_list
    };

    view! {
        <ConfigProvider>
            <div class="min-h-screen bg-gray-100 p-6">
                <div class="max-w-7xl mx-auto">
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-bold mb-2">
                            "LLM Cost Estimator"
                        </h1>
                        <p class="text-gray-600 text-lg">
                            "Calculate token usage and costs for various LLM models"
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
                                <div class="grid grid-cols-2 gap-4 auto-rows-fr">
                                    <div class="flex">
                                        <Card class="w-full flex flex-col h-full">
                                            <div class="p-5 flex-1 flex flex-col">
                                                <h3 class="text-xl font-semibold mb-4">
                                                    "Input Text"
                                                </h3>
                                                <div class="flex-1">
                                                    <Textarea
                                                        size=TextareaSize::Large
                                                        class="w-full h-full"
                                                        placeholder="Enter your input prompt here..."
                                                        on:input=move |ev| {
                                                            set_input_text.set(event_target_value(&ev));
                                                        }
                                                    />
                                                </div>
                                            </div>
                                        </Card>
                                    </div>

                                    <div class="flex">
                                        <Card class="w-full flex flex-col h-full">
                                            <div class="p-5 flex-1 flex flex-col">
                                                <h3 class="text-xl font-semibold mb-4">
                                                    "Select Model"
                                                </h3>
                                                <Space vertical=true>
                                                    <Input
                                                        class="w-full"
                                                        placeholder="Search models..."
                                                        on:input=move |ev| {
                                                            set_model_search.set(event_target_value(&ev));
                                                        }
                                                    />
                                                    <select
                                                        class="w-full h-80 p-2 border border-gray-300 rounded text-sm"
                                                        size="10"
                                                        on:change=move |ev| {
                                                            set_selected_model.set(event_target_value(&ev));
                                                        }
                                                    >
                                                        {move || {
                                                            filtered_models().into_iter().map(|model| {
                                                                let model_clone = model.clone();
                                                                let model_clone2 = model.clone();
                                                                view! {
                                                                    <option value=model selected=move || selected_model.get() == model_clone>
                                                                        {model_clone2}
                                                                    </option>
                                                                }
                                                            }).collect_view()
                                                        }}
                                                    </select>

                                                    {move || {
                                                        let model = selected_model.get();
                                                        let prices = model_prices.get();
                                                        if let Some(pricing) = prices.get(&model).cloned() {
                                                            view! {
                                                                <Card class="bg-gray-50">
                                                                    <div class="p-4">
                                                                        <h4 class="font-semibold mb-3">"Model Details"</h4>
                                                                        <Grid cols=2 x_gap=16 y_gap=8>
                                                                            <GridItem>
                                                                                <div class="text-sm">
                                                                                    <span class="text-gray-600">"Input Cost: "</span>
                                                                                    <strong>{format!("${:.6}", pricing.input_cost_per_token.unwrap_or(0.0))}</strong>
                                                                                </div>
                                                                            </GridItem>
                                                                            <GridItem>
                                                                                <div class="text-sm">
                                                                                    <span class="text-gray-600">"Output Cost: "</span>
                                                                                    <strong>{format!("${:.6}", pricing.output_cost_per_token.unwrap_or(0.0))}</strong>
                                                                                </div>
                                                                            </GridItem>
                                                                            {pricing.max_tokens.map(|max| view! {
                                                                                <GridItem>
                                                                                    <div class="text-sm">
                                                                                        <span class="text-gray-600">"Max Tokens: "</span>
                                                                                        <strong>{max.to_string()}</strong>
                                                                                    </div>
                                                                                </GridItem>
                                                                            })}
                                                                        </Grid>
                                                                    </div>
                                                                </Card>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    }}
                                                </Space>
                                            </div>
                                        </Card>
                                    </div>

                                    <div class="flex">
                                        <Card class="w-full flex flex-col h-full">
                                            <div class="p-5 flex-1 flex flex-col">
                                                <h3 class="text-xl font-semibold mb-4">
                                                    "Expected Output Text"
                                                </h3>
                                                <div class="flex-1">
                                                    <Textarea
                                                        size=TextareaSize::Large
                                                        class="w-full h-full"
                                                        placeholder="Enter expected output/completion here..."
                                                        on:input=move |ev| {
                                                            set_output_text.set(event_target_value(&ev));
                                                        }
                                                    />
                                                </div>
                                            </div>
                                        </Card>
                                    </div>

                                    <div class="flex">
                                        <Card class="w-full flex flex-col h-full">
                                            <div class="p-5 flex-1 flex flex-col">
                                                <h3 class="text-xl font-semibold mb-4">
                                                    "Cost Estimation"
                                                </h3>
                                                {move || {
                                                    let (input_tokens, output_tokens, total_tokens, input_cost, output_cost, total_cost) = calculation();

                                                    view! {
                                                        <Space vertical=true>
                                                            <Grid cols=3 x_gap=12>
                                                                <GridItem>
                                                                    <Card class="bg-gradient-to-br from-indigo-500 to-purple-600 text-white">
                                                                        <div class="p-4 text-center">
                                                                            <div class="text-xs opacity-90 mb-1">"Input"</div>
                                                                            <div class="text-2xl font-bold">{input_tokens}</div>
                                                                        </div>
                                                                    </Card>
                                                                </GridItem>
                                                                <GridItem>
                                                                    <Card class="bg-gradient-to-br from-pink-400 to-red-500 text-white">
                                                                        <div class="p-4 text-center">
                                                                            <div class="text-xs opacity-90 mb-1">"Output"</div>
                                                                            <div class="text-2xl font-bold">{output_tokens}</div>
                                                                        </div>
                                                                    </Card>
                                                                </GridItem>
                                                                <GridItem>
                                                                    <Card class="bg-gradient-to-br from-blue-400 to-cyan-400 text-white">
                                                                        <div class="p-4 text-center">
                                                                            <div class="text-xs opacity-90 mb-1">"Total"</div>
                                                                            <div class="text-2xl font-bold">{total_tokens}</div>
                                                                        </div>
                                                                    </Card>
                                                                </GridItem>
                                                            </Grid>

                                                            <Divider />

                                                            <div>
                                                                <h4 class="font-semibold mb-3">"Cost Breakdown"</h4>
                                                                <Space vertical=true>
                                                                    <div class="flex justify-between text-sm">
                                                                        <span class="text-gray-600">"Input Cost:"</span>
                                                                        <strong>{format!("${:.6}", input_cost)}</strong>
                                                                    </div>
                                                                    <div class="flex justify-between text-sm">
                                                                        <span class="text-gray-600">"Output Cost:"</span>
                                                                        <strong>{format!("${:.6}", output_cost)}</strong>
                                                                    </div>
                                                                    <Divider />
                                                                    <div class="flex justify-between">
                                                                        <span class="font-semibold">"Total Cost:"</span>
                                                                        <span class="text-xl font-bold text-blue-500">
                                                                            {format!("${:.6}", total_cost)}
                                                                        </span>
                                                                    </div>
                                                                </Space>
                                                            </div>

                                                            <Divider />

                                                            <div>
                                                                <h4 class="font-semibold mb-3">"Projections"</h4>
                                                                <Grid cols=2 x_gap=8 y_gap=8>
                                                                    <GridItem>
                                                                        <Card class="bg-gray-100">
                                                                            <div class="p-3 text-center">
                                                                                <div class="text-xs text-gray-600 mb-1">"100 requests"</div>
                                                                                <div class="font-semibold">{format!("${:.4}", total_cost * 100.0)}</div>
                                                                            </div>
                                                                        </Card>
                                                                    </GridItem>
                                                                    <GridItem>
                                                                        <Card class="bg-gray-100">
                                                                            <div class="p-3 text-center">
                                                                                <div class="text-xs text-gray-600 mb-1">"1,000 requests"</div>
                                                                                <div class="font-semibold">{format!("${:.2}", total_cost * 1000.0)}</div>
                                                                            </div>
                                                                        </Card>
                                                                    </GridItem>
                                                                    <GridItem>
                                                                        <Card class="bg-gray-100">
                                                                            <div class="p-3 text-center">
                                                                                <div class="text-xs text-gray-600 mb-1">"10,000 requests"</div>
                                                                                <div class="font-semibold">{format!("${:.2}", total_cost * 10000.0)}</div>
                                                                            </div>
                                                                        </Card>
                                                                    </GridItem>
                                                                    <GridItem>
                                                                        <Card class="bg-gray-100">
                                                                            <div class="p-3 text-center">
                                                                                <div class="text-xs text-gray-600 mb-1">"100,000 requests"</div>
                                                                                <div class="font-semibold">{format!("${:.2}", total_cost * 100000.0)}</div>
                                                                            </div>
                                                                        </Card>
                                                                    </GridItem>
                                                                </Grid>
                                                            </div>
                                                        </Space>
                                                    }
                                                }}
                                            </div>
                                        </Card>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </ConfigProvider>
    }
}
