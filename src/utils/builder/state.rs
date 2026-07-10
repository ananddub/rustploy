use std::error::Error;
use std::marker::PhantomData;

use crate::utils::builder::git::{GitCloneRequest, git_clone};
use crate::utils::tower::retry_policy::MyRetryPolicy;
use tower::util::BoxCloneService;
use tower::{ServiceBuilder, ServiceExt};
use crate::utils::builder::custom_type::DynService;

pub struct BuilderState<E> {
    pub git_clone: DynService<GitCloneRequest, ()>,
    _marker: PhantomData<E>,
}

impl<E> BuilderState<E> {
    pub fn new() -> Self {
        let git_service = ServiceBuilder::new()
            .buffer(100)
            .concurrency_limit(10)
            .retry(MyRetryPolicy { retries: 3 })
            .service_fn(git_clone)
            .boxed_clone();

        Self {
            git_clone: git_service,
            _marker: PhantomData,
        }
    }
}
