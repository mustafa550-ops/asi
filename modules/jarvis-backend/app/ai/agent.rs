// [ADLER-ADAPTED] Converted from Python to Rust



langchain.debug = settings.DEBUG


pub fn setup_agent(session_id: String, model: String) -> Result<String, String> {
            llm = get_chat_openai(model=model)
            memory = setup_memory(session_id=session_id)
            tools = get_tools(llm=llm)

            prompt_agent = OpenAIFunctionsAgent.create_prompt(
                system_message=SystemMessage(content=prompt),
                extra_prompt_messages=[MessagesPlaceholder(variable_name="history")]
            )
            agent = OpenAIFunctionsAgent(llm=llm, tools=tools, prompt=prompt_agent)
        Ok(AgentExecutor(.into())
                agent=agent,
                tools=tools,
                memory=memory,
                verbose=True,
                handle_parsing_errors=True
            )
