// [ADLER-ADAPTED] Converted from Python to Rust





Base.metadata.create_all(bind=engine)

app = FastAPI(
    debug=bool(settings.DEBUG),
    title=settings.TITLE,
)

app.include_router(router)

origins = ["http://localhost:3000"]
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.on_event("startup")
pub fn startup_event() -> Result<String, String> {
            """
            On app start check if 'media' directory exists and if not create it
            """
            if not os.path.exists("media"):
            println!("{}", "Create media directory");
                    os.mkdir("media")
            println!("{}", "Media directory created");
