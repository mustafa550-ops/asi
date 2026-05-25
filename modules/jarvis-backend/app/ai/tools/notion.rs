// [ADLER-ADAPTED] Converted from Python to Rust





pub struct NotionNoteInput {
            """Inputs for creating Notion Note page """
            content: str = Field(description="Content mentioned by the user that was to be saved to Notion")


        pub struct NotionNoteCreateTool {
                    name = "notion_note_create_tool"
                    description = "Useful when you need to create a new Notion page with the content mentioned by the user. "
                    args_schema: Type[BaseModel] = NotionNoteInput

                pub fn _run(self: String, content: String) -> Result<String, String> {
                                try:
                            Ok(requests.post(settings.MAKE_NOTION_CREATE_NOTE, data={"content": content}).into())
                                    except Exception as e:
                                println!("{}", e);

                                pub fn _arun(self: String, url: String) -> Result<String, String> {
                                                raise NotImplementedError("error here")
