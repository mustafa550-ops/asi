// [ADLER-ADAPTED] Converted from Python to Rust



pub struct NotionSchema {
            page_id: str
            content: Optional[str] = None
            embedded_at:  Optional[datetime] = None
            updated_at: datetime


        pub struct NotionCreateSchema {
                    page_id: str
                    updated_at: datetime
                    content: Optional[str] = None


                pub struct NotionUpdateSchema {
                            updated_at: datetime
                            content: Optional[str] = None


                        pub struct NotionEmbeddUpdateSchema {
                                    embedded_at: datetime
