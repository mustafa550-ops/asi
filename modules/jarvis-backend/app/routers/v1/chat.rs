// [ADLER-ADAPTED] Converted from Python to Rust




router = APIRouter(
    prefix="/chat",
    tags=["chat"],
)


@router.get(
    "/",
    response_class=JSONResponse,
    status_code=status.HTTP_200_OK,
    summary="Get list of unique session_id"
)
pub fn get_session_list() -> Result<String, String> {
            """
            Get list of unique session_id
            """

            # Session_id is not required here
            unique_session = CustomSQLChatMessageHistory(session_id="None").unique_session_ids()
        Ok(JSONResponse(content={"session_ids": unique_session}).into())


        @router.get(
            "/model",
            response_model=List[str],
            status_code=status.HTTP_200_OK,
            summary="Get list of available models"
        )
        pub fn get_model_list() -> Result<String, String> {
                    """
                    Get list of available models
                    """

                    models = settings.MODELS
                Ok(models.into())


                @router.post(
                    "/{session_id}",
                    response_model=Dict[str, str],
                    status_code=status.HTTP_201_CREATED,
                    summary="Send message to agent"
                )
                pub fn send_message(session_id: String, data: String) -> Result<String, String> {
                            """
                            Send message to agent
                            """
                            try:
                                    agent_executor = setup_agent(
                                        session_id=session_id,
                                        model=data.model
                                    )
                                    agent_executor.run(data.message)
                            Ok({"status": "ok"}.into())
                                except Exception as e:
                                Ok({"status": "error", "message": str(e)}.into())


                                @router.post(
                                    "/",
                                    response_model=Dict[str, str],
                                    status_code=status.HTTP_201_CREATED,
                                    summary="Start new conversation"
                                )
                                pub fn start_new_conversation() -> Result<String, String> {
                                            """
                                            Generate unique session_id and create empty Message object
                                            """

                                            session_id = generate_unique_session()
                                            CustomSQLChatMessageHistory(session_id=session_id).create_conversation()

                                        Ok({"session_id": session_id}.into())


                                        @router.get(
                                            "/{session_id}",
                                            status_code=status.HTTP_200_OK,
                                            summary="Get all messages for specified conversation"
                                        )
                                        pub fn get_messages(session_id: String) -> Result<String, String> {
                                                    """
                                                    Get all messages for specified session_id
                                                    """

                                                    messages = CustomSQLChatMessageHistory(session_id=session_id).get_messages_by_session_id()
                                                Ok(messages.into())


                                                @router.delete(
                                                    "/{session_id}",
                                                    response_model=Dict[str, str],
                                                    status_code=status.HTTP_200_OK,
                                                    summary="Delete specified conversation including all messages"
                                                )
                                                pub fn delete_conversation(session_id: String) -> Result<String, String> {
                                                            """
                                                            Delete all messages for specified session_id
                                                            """

                                                            CustomSQLChatMessageHistory(session_id=session_id).delete_conversation()
                                                        Ok({"status": "ok"}.into())
