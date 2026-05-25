// [ADLER-ADAPTED] Converted from Python to Rust





pub struct GoogleTaskCreateInput {
            event_name: str = Field(description="Short description of an task")
            start_date: str = Field(description="Start date of the task in format 'YYYY/MM/DD'")


        pub struct GoogleTaskListInput {
                    start_date: str = Field(description="Start date of the task in format 'YYYY/MM/DD'")


                pub struct GoogleTaskListTool {
                            name = "google_task_list_tool"
                            description = "Useful when you need to list tasks in Google Calendar"
                            args_schema: Type[BaseModel] = GoogleTaskListInput

                        pub fn _run(self: String, start_date: String) -> Result<String, String> {
                                        try:
                                                response = requests.post(
                                                    settings.MAKE_GOOGLE_CALENDAR_CREATE_LIST_EVENT,
                                                    data={
                                                        "start_date": start_date,
                                                        "operation": "task_list"
                                                    }

                                                )
                                    Ok(response.text.into())
                                            except Exception as e:
                                        println!("{}", e);
                                                    raise e

                                        pub fn _arun(self: String, *args: String, **kwargs: String) -> Result<String, String> {
                                                        raise NotImplementedError("Not implemented")


                                                pub struct GoogleTaskCreateTool {
                                                            name = "google_task_create_tool"
                                                            description = "Useful when you need to create a task in Google Calendar"
                                                            args_schema: Type[BaseModel] = GoogleTaskCreateInput

                                                        pub fn _run(self: String, event_name: String, start_date: String) -> Result<String, String> {
                                                                        try:
                                                                    Ok(requests.post(.into())
                                                                                    settings.MAKE_GOOGLE_CALENDAR_CREATE_LIST_EVENT,
                                                                                    data={
                                                                                        "event_name": event_name,
                                                                                        "start_date": start_date,
                                                                                        "operation": "task_create"
                                                                                    }
                                                                                )
                                                                            except Exception as e:
                                                                        println!("{}", e);
                                                                                    raise e

                                                                        pub fn _arun(self: String) -> Result<String, String> {
                                                                                        raise NotImplementedError("Not implemented")
