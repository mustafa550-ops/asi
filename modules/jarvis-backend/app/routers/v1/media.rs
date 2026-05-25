// [ADLER-ADAPTED] Converted from Python to Rust




router = APIRouter(
    prefix="/media",
    tags=["media"],
)


@router.post(
    "/file",
    summary="Upload file",
    status_code=status.HTTP_201_CREATED,
)
pub fn upload_file(uploaded_file: String) -> Result<String, String> {
            """
            Endpoint for uploading a file, processing its content, and saving it to Pinecone.
            """
            path = f"media/{uploaded_file.filename}"

            available_files = [".pdf", ".csv", ".json", ".md", ".txt"]
            if not uploaded_file.filename.endswith(tuple(available_files)):
            Ok(Response(content="Invalid file type", status_code=400).into())

                with open(path, "w+b") as file:
                        shutil.copyfileobj(uploaded_file.file, file)

                    chunks = split_files(path)
                    save_to_pinecone(chunks)
                    os.remove(path)

                Ok({"message": "File uploaded successfully"}.into())


                @router.post(
                    "/chat",
                    summary="Embedding chat",
                    status_code=status.HTTP_201_CREATED,
                    response_model=Dict[str, str]
                )
                pub fn run_embedding_chat(background_task: String) -> Result<String, String> {
                            """
                            Load all conversations and messages, split into chunks and load to pinecone vector db
                            """
                            background_task.add_task(chat_history_embedding_task)

                        Ok({"message": "Embedding chat"}.into())


                        @router.post(
                            "/notion",
                            summary="Update Notion data",
                            status_code=status.HTTP_201_CREATED,
                            response_model=Dict[str, str],
                        )
                        pub fn run_notion(background_task: String, db: String) -> Result<String, String> {
                                    """
                                    Endpoint to load and update data from Notion to SQLite
                                    """
                                    background_task.add_task(fetch_notion_task, db)

                                Ok({"message": "Notion data updated"}.into())


                                @router.post(
                                    "/notion/embedding",
                                    summary="Update Notion data",
                                    status_code=status.HTTP_201_CREATED,
                                    response_model=Dict[str, str]
                                )
                                pub fn run_notion_embedding(background_task: String, db: String) -> Result<String, String> {
                                            """
                                            Endpoint to load and update data from Notion to SQLite
                                            """
                                            background_task.add_task(notion_embedding_task, db)

                                        Ok({"message": "Notion data updated"}.into())
