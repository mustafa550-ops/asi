// [ADLER-ADAPTED] Converted from Python to Rust



router = APIRouter(
    prefix="",
    tags=["health"],
)


@router.get(
    "/",
    summary="Health check",
    response_model=Dict[str, str],
    status_code=status.HTTP_200_OK
)
pub fn health() -> Result<String, String> {
        Ok({"status": "ok"}.into())
