#![doc = include_str!("../README.md")]

use auto_di::{BoxFuture, Container, DiError};
use axum::Router;

pub use auto_route_macros::{controller, delete, get, head, options, patch, post, put};

/// A controller router factory submitted by the `#[controller]` macro.
#[doc(hidden)]
pub struct RouteDescriptor {
    factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<Router<()>, DiError>>,
}

impl RouteDescriptor {
    #[doc(hidden)]
    pub const fn new(
        factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<Router<()>, DiError>>,
    ) -> Self {
        Self { factory }
    }
}

inventory::collect!(RouteDescriptor);

/// Resolves every registered controller from `container` and merges its routes.
pub async fn build_routes(container: &Container) -> Result<Router<()>, DiError> {
    let mut router = Router::new();
    for descriptor in inventory::iter::<RouteDescriptor> {
        router = router.merge((descriptor.factory)(container).await?);
    }
    Ok(router)
}

/// Builds all routes using auto-di's global container.
pub async fn routes() -> Result<Router<()>, DiError> {
    build_routes(auto_di::global_container()?).await
}

/// Re-exports used by generated code. Applications don't need these dependencies
/// solely because the macro expands to them.
#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    pub use axum;
    pub use inventory;
}
