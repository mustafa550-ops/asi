// [ADLER-ADAPTED] Converted from Python to Rust




# from langchain_core.messages import SystemMessage
# from langchain_core.prompts import MessagesPlaceholder


# from .prompt import prompt


pub fn setup_memory(session_id: String) -> Result<String, String> {
            chat_message_history = CustomSQLChatMessageHistory(session_id=session_id)

            # agent_kwargs = {
            #     # "system_message": SystemMessage(content=prompt),
            #     "extra_prompt_messages": [MessagesPlaceholder(variable_name="history")],
            # }
            memory = ConversationBufferMemory(
                memory_key="history",
                return_messages=True,
                chat_memory=chat_message_history,
            )

            # return agent_kwargs, memory
        Ok(memory.into())
