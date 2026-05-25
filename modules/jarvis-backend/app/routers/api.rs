// [ADLER-ADAPTED] Converted from Python to Rust



router = APIRouter(prefix="/api/v1")

router.include_router(health.router)
router.include_router(media.router)
router.include_router(chat.router)
