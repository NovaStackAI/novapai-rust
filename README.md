# NovaPAI Rust SDK Example 🦀

> Use NovaPAI in Rust — OpenAI-compatible API gateway for DeepSeek V4 Pro & 50+ LLMs. Change one line of code, switch to any model.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![API](https://img.shields.io/badge/API-NovaPAI-6C47FF)](https://novapai.ai)
[![Rust](https://img.shields.io/badge/Rust-SDK-brightgreen)](#)

## What is NovaPAI?

**[NovaPAI](https://novapai.ai)** is an **OpenAI-compatible API gateway** that gives you access to top-tier LLMs like **DeepSeek V4 Pro**, GPT-4, Claude, Gemini (50+ models) through the standard OpenAI SDK. If you know how to use the OpenAI API, you already know how to use NovaPAI — just change the `base_url`.

## Quick Start

### 1. Get an API Key

Sign up at **[novapai.ai](https://novapai.ai)** to get your free API key.

### 2. Install the SDK

```bash
cargo add async-openai
```

### 3. Run the Example

```bash
git clone https://github.com/NovaStackAI/novapai-rust.git
cd novapai-rust
export NOVAPAI_API_KEY="your-api-key"
cargo run
```

## Examples Included

All examples are in `main.rs`:

| # | Example | Description |
|---|---------|------------|
| 1 | **Basic Chat** | Single-turn chat completion |
| 2 | **Streaming** | Real-time token-by-token output |
| 3 | **Multi-turn Conversation** | Context-aware multi-message dialogue |
| 4 | **Function Calling** | Define tools, let the model decide which to call |
| 5 | **JSON Mode** | Structured JSON output for data extraction |
| 6 | **List Models** | Fetch all available models |

## Core Concept

```python
# Standard OpenAI SDK — just change base_url!
from openai import OpenAI

client = OpenAI(
    api_key="your-novapai-key",
    base_url="https://api.novapai.ai/router/v1"  # ← Only this line changes!
)

response = client.chat.completions.create(
    model="deepseek-v4-pro",
    messages=[{"role": "user", "content": "Hello!"}]
)
print(response.choices[0].message.content)
```

**That's it.** The OpenAI SDK you already know works directly with NovaPAI. No new SDK to learn, no new API surface.

## Why NovaPAI?

- ✅ **OpenAI-Compatible** — Use any OpenAI SDK in any language
- ✅ **50+ Models** — DeepSeek V4 Pro, GPT-4, Claude, Gemini and more
- ✅ **One API Key** — All models, one account
- ✅ **No Vendor Lock-in** — Switch models with one line change
- ✅ **Free Tier Available** — Start building with free credits

## All SDK Examples

Browse all our SDK examples:

- [novapai-python](https://github.com/NovaStackAI/novapai-python)
- [novapai-javascript](https://github.com/NovaStackAI/novapai-javascript)
- [novapai-typescript](https://github.com/NovaStackAI/novapai-typescript)
- [novapai-go](https://github.com/NovaStackAI/novapai-go)
- [novapai-java](https://github.com/NovaStackAI/novapai-java)
- [novapai-csharp](https://github.com/NovaStackAI/novapai-csharp)
- [novapai-php](https://github.com/NovaStackAI/novapai-php)
- [novapai-ruby](https://github.com/NovaStackAI/novapai-ruby)
- [novapai-curl](https://github.com/NovaStackAI/novapai-curl)

## Links

- 🌐 [NovaPAI Website](https://novapai.ai)
- 📖 [API Reference](https://novapai.ai/docs)
- 🏠 [NovaStackAI GitHub](https://github.com/NovaStackAI)

---

Made with ❤️ by [NovaStackAI](https://github.com/NovaStackAI)
