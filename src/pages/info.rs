use leptos::prelude::*;
use thaw::*;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const LEPTOS_VERSION: &str = env!("LEPTOS_VERSION");
const LEPTOS_ROUTER_VERSION: &str = env!("LEPTOS_ROUTER_VERSION");

#[component]
pub fn InfoPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-4xl font-bold mb-2">
                        "Application Info"
                    </h1>
                    <p class="text-gray-600 text-lg">
                        "Version and dependency information"
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

                <div class="space-y-6">
                    <Card>
                        <div class="p-6">
                            <h2 class="text-2xl font-bold mb-4 text-blue-600">
                                "Application"
                            </h2>
                            <Grid cols=2 x_gap=16 y_gap=12>
                                <GridItem>
                                    <div class="text-sm text-gray-600">"Name"</div>
                                    <div class="text-lg font-semibold">{APP_NAME}</div>
                                </GridItem>
                                <GridItem>
                                    <div class="text-sm text-gray-600">"Version"</div>
                                    <div class="text-lg font-semibold">{APP_VERSION}</div>
                                </GridItem>
                            </Grid>
                        </div>
                    </Card>

                    <Card>
                        <div class="p-6">
                            <h2 class="text-2xl font-bold mb-4 text-purple-600">
                                "Core Framework"
                            </h2>
                            <Grid cols=2 x_gap=16 y_gap=12>
                                <GridItem>
                                    <div class="text-sm text-gray-600">"Leptos"</div>
                                    <div class="text-lg font-semibold">{LEPTOS_VERSION}</div>
                                    <div class="text-xs text-gray-500 mt-1">"Reactive Web Framework"</div>
                                </GridItem>
                                <GridItem>
                                    <div class="text-sm text-gray-600">"Leptos Router"</div>
                                    <div class="text-lg font-semibold">{LEPTOS_ROUTER_VERSION}</div>
                                    <div class="text-xs text-gray-500 mt-1">"Client-side Routing"</div>
                                </GridItem>
                            </Grid>
                        </div>
                    </Card>

                    <Card >
                        <div class="p-6">
                            <h2 class="text-2xl font-bold mb-4 text-green-600">
                                "Data Source Credits"
                            </h2>
                            <div class="space-y-3">
                                <p class="text-gray-700">
                                    "Model pricing data is sourced from the "
                                    <strong>"LiteLLM"</strong>
                                    " project, an open-source library that provides a unified interface to 100+ LLMs."
                                </p>
                                <div class="bg-white p-4 rounded border border-green-200">
                                    <div class="text-sm text-gray-600 mb-1">"Repository"</div>
                                    <a
                                        href="https://github.com/BerriAI/litellm"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="text-blue-600 hover:text-blue-800 font-mono text-sm"
                                    >
                                        "github.com/BerriAI/litellm"
                                    </a>
                                </div>
                                <div class="bg-white p-4 rounded border border-green-200">
                                    <div class="text-sm text-gray-600 mb-1">"Data Source"</div>
                                    <a
                                        href="https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="text-blue-600 hover:text-blue-800 font-mono text-xs break-all"
                                    >
                                        "model_prices_and_context_window.json"
                                    </a>
                                </div>
                                <p class="text-sm text-gray-600 italic">
                                    "Special thanks to the BerriAI team and contributors for maintaining this comprehensive pricing database."
                                </p>
                            </div>
                        </div>
                    </Card>

                    <Card class="bg-blue-50">
                        <div class="p-6">
                            <h2 class="text-xl font-bold mb-3">
                                "About This Application"
                            </h2>
                            <p class="text-gray-700 mb-2">
                                "This application helps estimate costs for various Large Language Model (LLM) APIs. It fetches real-time pricing data from the LiteLLM repository and provides an easy-to-use interface for cost calculations."
                            </p>
                            <p class="text-gray-700 mb-4">
                                "Built with Rust and Leptos for maximum performance and type safety, compiled to WebAssembly for seamless browser execution."
                            </p>
                            <div class="flex gap-3">
                                <a
                                    href="https://github.com/vincentzhangz/llm-token-dashboard/issues/new"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg shadow-md transition-colors duration-200"
                                >
                                    <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                                    </svg>
                                    "Report Issue"
                                </a>
                                <a
                                    href="https://github.com/vincentzhangz/llm-token-dashboard"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center px-4 py-2 bg-gray-700 hover:bg-gray-800 text-white font-semibold rounded-lg shadow-md transition-colors duration-200"
                                >
                                    <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                    </svg>
                                    "View on GitHub"
                                </a>
                            </div>
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
