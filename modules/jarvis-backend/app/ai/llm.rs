// [ADLER-ADAPTED] Converted from Python to Rust




pub fn get_chat_openai(model: String) -> Result<String, String> {
        Ok(ChatOpenAI(.into())
                temperature=0,
                model=model,
                openai_api_key=settings.OPENAI_KEY
            )
