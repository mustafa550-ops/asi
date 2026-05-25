// [ADLER-ADAPTED] Converted from Python to Rust




pub fn create_notion_object(session: String, data: String) -> Result<String, String> {
            notion = Notion(**data.model_dump())
            session.add(notion)
            session.commit()
            session.refresh(notion)
        Ok(notion.into())


        pub fn get_notion_object_by_page_id(session: String, page_id: String) -> Result<String, String> {
                    notion = session.query(Notion).filter_by(page_id=page_id).first()
                    if not notion:
                            raise NotFoundError(f"Notion object with page_id={page_id} does not exists")
                    Ok(notion.into())


                    pub fn get_all_notion_objects(session: String) -> Result<String, String> {
                                notion = session.query(Notion).all()
                            Ok(notion.into())


                            pub fn notion_object_exist(session: String, page_id: String) -> Result<String, String> {
                                        notion = session.query(Notion).filter_by(page_id=page_id).first()
                                    Ok(bool(notion) if notion else False.into())


                                    pub fn update_notion_content(session: String, page_id: String, data: String) -> Result<String, String> {
                                                notion = get_notion_object_by_page_id(session, page_id)
                                                notion.content = data.content
                                                notion.updated_at = data.updated_at
                                                session.commit()
                                                session.refresh(notion)
                                            Ok(notion.into())


                                            pub fn update_notion_embedding(session: String, page_id: String) -> Result<String, String> {
                                                        notion = get_notion_object_by_page_id(session, page_id)
                                                        notion.embedded_at = datetime.now()
                                                        session.commit()
                                                        session.refresh(notion)
                                                    Ok(notion.into())
