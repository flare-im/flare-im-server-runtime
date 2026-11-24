//! Control-Plane assertion wire constants shared by both sides of the boundary.
//!
//! The platform Control Plane (flare-social-server) signs control assertions; the Core
//! data plane (flare-core-server) verifies them. The two sides live in separate
//! repositories and may not depend on each other, so these constants were historically
//! duplicated in both. This module is now the single source of truth: it lives in the
//! one crate both sides already depend on (`flare-core-base`), keeping the issuer,
//! audiences, subjects and TTL from drifting apart.
//!
//! Mirrored types (verifier in core, signer in the platform) still live in their own
//! repositories; only the wire values are pinned here.

/// Issuer of every control assertion the platform signs.
pub const FLARE_PLATFORM_ISSUER: &str = "flare-platform";

/// Audience of a projection-write assertion (Capability `TenantProjection`, Online
/// `KickTenant`).
pub const CORE_PROJECTION_AUDIENCE: &str = "flare-core-projection";

/// Audience of the read-only `ClientBuildStats.GetConnectionBuilds` assertion.
pub const CORE_BUILD_STATS_AUDIENCE: &str = "flare-core-build-stats";

/// Subject of the platform projection worker.
pub const PROJECTION_WORKER_SUBJECT: &str = "flare-platform-worker";

/// Subject of the platform control scheduler.
pub const SCHEDULER_SUBJECT: &str = "flare-platform-control-scheduler";

/// Maximum assertion lifetime in seconds. The platform never mints one longer; the
/// verifier rejects `exp - iat` beyond it.
pub const PLATFORM_CONTROL_TOKEN_TTL_SECONDS: u64 = 60;