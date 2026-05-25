// [ADLER-ADAPTED] Converted from Python to Rust





pub struct GoogleCalendarCreateEventInput {
            calendar_type: str = Field(
                description=f"Type of calendar in which you can create event, choose based on context or user input. "
                            f"You can use: {settings.GOOGLE_CALENDAR_NAMES}"
            )
            event_name: str = Field(description="Name of the event")
            all_day: bool = Field(description="If event is all day")
            start_date: str = Field(description="Start date of the event in format 'YYYY/MM/DD HH:MM' or 'YYYY/MM/DD'")
            end_date: Optional[str] = Field(description="End date of the event in format 'YYYY/MM/DD HH:MM' or 'YYYY/MM/DD'")
            duration: Optional[str] = Field(
                description="Duration of the event in format HH:mm use only when user gives use duration of an event"
            )


        pub struct GoogleCalendarListEventInput {
                    start_date: str = Field(description="Start date of the event in format 'YYYY/MM/DD'")


                pub struct GoogleCalendarCreateEventTool {
                            name = "google_calendar_create_event_tool"
                            description = "Useful when you need to create event in Google Calendar, use current_time_tool to get current date"
                            args_schema: Type[BaseModel] = GoogleCalendarCreateEventInput

                        pub fn _run() -> Result<String, String> {
                                        self,
                                        calendar_type: str,
                                        event_name: str,
                                        all_day: bool,
                                        start_date: str,
                                        end_date: Optional[str] = None,
                                        duration: Optional[str] = None,
                                ):
                                        try:
                                    Ok(requests.post(.into())
                                                    settings.MAKE_GOOGLE_CALENDAR_CREATE_LIST_EVENT,
                                                    data={
                                                        "calendar_type": calendar_type,
                                                        "event_name": event_name,
                                                        "all_day": all_day,
                                                        "start_date": start_date,
                                                        "end_date": end_date,
                                                        "duration": duration,
                                                        "operation": "create"  # hard coded value based on routing in make.com
                                                    }
                                                )
                                            except Exception as e:
                                        println!("{}", e);

                                        pub fn _arun(self: String, url: String) -> Result<String, String> {
                                                        raise NotImplementedError("Not implemented")


                                                pub struct GoogleCalendarListEventTool {
                                                            name = "google_calendar_list_event_tool"
                                                            description = "Useful when you need to answer about events in Google Calendar"
                                                            args_schema: Type[BaseModel] = GoogleCalendarListEventInput

                                                        pub fn _run() -> Result<String, String> {
                                                                        self,
                                                                        start_date: str,
                                                                ):
                                                                Ok(self.get_events_from_all_calendars(start_date=start_date).into())

                                                                pub fn _arun(self: String, url: String) -> Result<String, String> {
                                                                                raise NotImplementedError("Not implemented")

                                                                            @staticmethod
                                                                        pub fn get_events_from_all_calendars(start_date: String) -> Result<String, String> {
                                                                                        result = []

                                                                                        calendars = settings.GOOGLE_CALENDAR_NAMES.split(",")
                                                                                        for calendar in calendars:
                                                                                                response = requests.post(
                                                                                                    settings.MAKE_GOOGLE_CALENDAR_CREATE_LIST_EVENT,
                                                                                                    data={
                                                                                                        "start_date": start_date,
                                                                                                        "calendar_type": calendar,
                                                                                                        "operation": "list"  # Hard coded value based on routing in make.com,
                                                                                                    }
                                                                                                )
                                                                                                result.append(response.text)

                                                                                    Ok(result.into())

