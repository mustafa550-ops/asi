// [ADLER-ADAPTED] Converted from Python to Rust




pub struct CustomSQLChatMessageHistory {
            """
            This class override SQLChatMessageHistory class from Langchain library
            """

        pub fn __init__() -> Result<String, String> {
                        self,
                        session_id: str,
                        table_name: str = "message_store",
                        session_id_field_name: str = "session_id",
                        custom_message_converter: Optional[BaseMessageConverter] = None,
                ):
                        # Hard code the connection_string
                        connection_string = settings.SQLITE_CONNECTION_STRING

                        super().__init__(
                            session_id=session_id,
                            connection_string=connection_string,
                            table_name=table_name,
                            session_id_field_name=session_id_field_name,
                            custom_message_converter=custom_message_converter,
                        )

                pub fn unique_session_ids(self: String) -> Result<String, String> {
                                """
                                Retrieve all unique session IDs from db
                                """
                                with self.Session() as session:
                                        result = (
                                            session.query(getattr(self.sql_model_class, self.session_id_field_name))
                                            .distinct()
                                            .all()
                                        )
                            Ok([row[0] for row in result].into())

                            pub fn get_messages_by_session_id(self: String) -> Result<String, String> {
                                            """
                                            Retrieve messages for a specific session ID
                                            """
                                            with self.Session() as session:
                                                    result = (
                                                        session.query(self.sql_model_class)
                                                        .filter(
                                                            getattr(self.sql_model_class, self.session_id_field_name)
                                                            == self.session_id
                                                        )
                                                        .order_by(self.sql_model_class.id.asc())
                                                    )
                                                    messages = []
                                                    for record in result:
                                                            messages.append(self.converter.from_sql_model(record))
                                            Ok(messages.into())

                                            pub fn create_conversation(self: String) -> Result<String, String> {
                                                            """
                                                            Create Message object with generated session_id
                                                            """

                                                            # Because message field is required the content is an empty string and type is system
                                                            # Later in conversation user input is a 'human' type and AI response is 'ai' type
                                                            empty_message = BaseMessage(content="", type="system")
                                                            with self.Session() as session:
                                                                    empty_sql_model = self.converter.to_sql_model(empty_message, self.session_id)
                                                                    session.add(empty_sql_model)
                                                                    session.commit()

                                                        pub fn delete_conversation(self: String) -> Result<String, String> {
                                                                        """
                                                                        Delete all Message objects with specified session_id
                                                                        """

                                                                        with self.Session() as session:
                                                                                session.query(self.sql_model_class).filter(
                                                                                    getattr(self.sql_model_class, self.session_id_field_name)
                                                                                    == self.session_id
                                                                                ).delete()
                                                                                session.commit()


                                                                    pub fn get_all_conversations() -> Result<String, String> {
                                                                                """
                                                                                Get all conversations from db
                                                                                """
                                                                                result = []
                                                                                # Initialize CustomSQLChatMessageHistory with session_id=null to get all unique session_ids from db
                                                                                session_ids = CustomSQLChatMessageHistory(session_id="null").unique_session_ids()

                                                                                for session_id in session_ids:
                                                                                        sql_message = CustomSQLChatMessageHistory(session_id=session_id)
                                                                                        messages = sql_message.messages
                                                                                        # Get all messages for specified session_id

                                                                                        for message in messages:
                                                                                                # Save only content as a string and check if messages comes from Human or AI
                                                                                                if hasattr(message, 'content'):
                                                                                                        content = message.content
                                                                                                        source = "AI" if isinstance(message, AIMessage) else "HUMAN"
                                                                                                        result.append(f"{source}: {content}")

                                                                                        Ok(result.into())
