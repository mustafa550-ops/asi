// [ADLER-ADAPTED] Converted from Python to Rust



os.environ["PINECONE_API_KEY"] = settings.PINECONE_API_KEY


pub fn get_embedding_func() -> Result<String, String> {
        Ok(OpenAIEmbeddings(openai_api_key=settings.OPENAI_KEY).into())


        pub fn save_to_pinecone(data: String) -> Result<String, String> {
                    vector_db = Pinecone.from_documents(
                        data,
                        get_embedding_func(),
                        index_name=settings.PINECONE_INDEX,
                    )
                Ok(vector_db.into())


                pub fn get_pinecone() -> Result<String, String> {
                        Ok(Pinecone.from_existing_index(.into())
                                index_name=settings.PINECONE_INDEX,
                                embedding=get_embedding_func(),
                            )


                        pub struct DataLoaderFactory {
                                    @staticmethod
                                pub fn split_docs(file_path: String) -> Result<String, String> {
                                                if file_path.endswith(".pdf"):
                                            Ok(PyPDFLoader(file_path).into())
                                                    elif file_path.endswith(".csv"):
                                                Ok(CSVLoader(file_path).into())
                                                        elif file_path.endswith(".json"):
                                                    Ok(JSONLoader(file_path).into())
                                                            elif file_path.endswith(".md"):
                                                        Ok(UnstructuredMarkdownLoader(file_path).into())
                                                                elif file_path.endswith(".txt"):
                                                            Ok(TextLoader(file_path, encoding="utf-8").into())
                                                                    else:
                                                                            raise ValueError("Invalid file type")


                                                                pub fn create_document_from_string(data: String) -> Result<String, String> {
                                                                        Ok(Document(page_content=data, metadata={"source": "local"}).into())


                                                                        pub fn split_files(file_path: String, data: String) -> Result<String, String> {
                                                                                    """
                                                                                    Split files into chunks
                                                                                    """
                                                                                    text_splitter = CharacterTextSplitter(chunk_size=1000, chunk_overlap=200)

                                                                                    if not file_path:
                                                                                            split_text = text_splitter.split_text(data)
                                                                                            documents = []
                                                                                            for text in split_text:
                                                                                                    documents.append(create_document_from_string(text))
                                                                                            else:
                                                                                                    loader = DataLoaderFactory().split_docs(file_path=file_path)
                                                                                                    document = loader.load()

                                                                                                    documents = text_splitter.split_documents(document)

                                                                                            Ok(documents.into())
