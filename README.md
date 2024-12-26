# Chatty

## Overview
Chatty is an interactive chatbot application.

### Features
- [x] Chat API service for Fictionx.ai
- [x] Provide chat API services
- [x] CLI chat mode
- [x] Attractive novel recommendations
- [x] Engage in conversations with characters

## Getting Started

To start the Chatty application, use the following command:
```
cargo run
```

## API Usage Example

Here's how to use the API:

1. **Send a message to a character:**

```
curl -X POST http://127.0.0.1:8080/api/chat_with_fictonx -H "Content-Type: application/json" -d '{
"character_id": "1c76e8ce-a6b4-4774-9edc-aa7a3b93fb1c",
"character_name": "Kitajima Kyouko",
"description": "Kitajima Kyouko is established as being male but turned into a ghoul by some mutated artifacts to seek help from for medical treatment.",
"message": "what are you doing?"
}'
```


2. **Expected response:**
```
{
"response": "Trying to get this damn curse lifted. It's a real pain in the ass.\n"
}
```


---

## CLI Mode
### Example Usage:

```
Running in CLI mode
Welcome to the FictionX chatbot! Type 'exit' to quit.
> hi, gm
========================== Response ============================
"Good morning! How can I help you today?\n"
================================================================

> recommend novels from fictionx
========================== Response ============================
> 1. Angel of Vengeance: Alita's High-Tech Battle for Justice
> 2. Metal and Mortal Destiny
> 3. Rebel Code: Rise of the Crypto Punks
> 4. Beyond the Barren Horizon
> 5. Shadows of the Digital Vodou
> 6. Mars' Last Stand: The AI War
> 7. Code of Deception: The Satoshi Conspiracy
> 8. Quest for Cryptic Dominion: Taming the Dragon's Fury with Bitcoin Sorcery
================================================================
```


## Tech Stack

- **Programming Language**: Rust
- **Web Framework**: Actix-web
- **Database**: PostgreSQL
- **Other Tools**: Rig / Gemini

---

## Note on `rig-core`
The support for Gemini in `rig-core v0.6.0` is incomplete; modifications to the `rig-core v0.6.0` code are necessary when defining tools to ensure proper functionality.    

### Here is the modified version [rig](https://github.com/zTgx/rig) for this project. 
