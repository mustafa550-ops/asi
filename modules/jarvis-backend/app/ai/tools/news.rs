// [ADLER-ADAPTED] Converted from Python to Rust



newsapi = NewsApiClient(api_key=settings.NEWS_API)


pub fn get_top_headlines_formatted() -> Result<String, String> {
            # Get top headlines
            result = ""

            top_headlines = newsapi.get_top_headlines()

            articles = top_headlines["articles"]
            for idx, article in enumerate(articles):
                    title = f"{idx} + {article['title']} + '\n' "
                    result += title
                    result += f"Description: {article["description"]} + '\n'"

            Ok(result.into())


            pub struct NewsTool {
                        name = "news_tool"
                        description = "Useful for when you need to answer questions about current news"

                    pub fn _to_args_and_kwargs(self: String, tool_input: String, dict]: String) -> Result<String, String> {
                            Ok((), {}.into())

                            pub fn _run(self: String) -> Result<String, String> {
                                    Ok(get_top_headlines_formatted().into())

                                        async def _arun(self) -> str:
                                                """Use the tool asynchronously."""
                                                raise NotImplementedError("custom_search does not support async")
