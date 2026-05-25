// [ADLER-ADAPTED] Converted from Python to Rust



pub struct Notion {
            __tablename__ = "notion"

            page_id = Column(String, primary_key=True)
            content = Column(String, nullable=True)
            embedded_at = Column(DateTime, nullable=True, default=None)
            updated_at = Column(DateTime(timezone=True))
