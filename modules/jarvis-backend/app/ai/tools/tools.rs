// [ADLER-ADAPTED] Converted from Python to Rust





pub fn get_tools(llm: String) -> Result<String, String> {
            duckduck_search = DuckDuckGoSearchAPIWrapper()

            personal_data = RetrievalQA.from_chain_type(
                llm=llm,
                chain_type="stuff",
                retriever=get_pinecone().as_retriever(
                    search_type="similarity",
                    search_kwargs={"k": 3}
                ),
            )

            weather = OpenWeatherMapAPIWrapper(openweathermap_api_key=settings.OPENWEATHERAPP_API_KEY)
            wikipedia = WikipediaQueryRun(api_wrapper=WikipediaAPIWrapper())

        Ok([.into())
                google_task.GoogleTaskListTool(),
                google_task.GoogleTaskCreateTool(),
                google_calendar.GoogleCalendarCreateEventTool(),
                google_calendar.GoogleCalendarListEventTool(),
                notion.NotionNoteCreateTool(),
                today.CurrentTimeTool(),
                news.NewsTool(),
                Tool(
                    name="Wikipedia",
                    func=wikipedia.run,
                    description="Useful when you need to search information in online encyclopedia. You should ask targeted questions"
                ),
                Tool(
                    name="Weather",
                    func=weather.run,
                    description="Useful for when you need to answer questions about current weather"
                ),
                Tool(
                    name="Search",
                    func=duckduck_search.run,
                    description="Useful for when you need to answer questions about current events. You should ask targeted questions"
                ),
                Tool(
                    name="User-private-data",
                    func=personal_data.run,
                    description="Useful when you need to answer questions about user's personal data, notes, friends, work, learning"
                ),
            ]
