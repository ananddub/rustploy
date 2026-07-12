use std::fmt::Display;
use std::future::{Ready, ready};
use thiserror::Error;
use tower::retry::Policy;

#[derive(Debug, Error)]
pub enum RetryPolicyError<E>
where
    E: std::error::Error + 'static,
{
    #[error("retryable: {0}")]
    Retry(E),
    #[error("not retryable: {0}")]
    NotRetryable(E),
}

#[derive(Clone)]
pub struct MyRetryPolicy {
    pub retries: usize,
}

impl<Req, Res, E> Policy<Req, Res, RetryPolicyError<E>> for MyRetryPolicy
where
    Req: Clone,
    E: Display + std::error::Error,
{
    type Future = Ready<()>;

    fn retry(
        &mut self,
        _req: &mut Req,
        result: &mut Result<Res, RetryPolicyError<E>>,
    ) -> Option<Self::Future> {
        match result {
            Err(RetryPolicyError::Retry(err)) if self.retries > 0 => {
                self.retries -= 1;

                tracing::warn!(
                        retries_left = self.retries,
                        error = %err,
                        "Retrying request"
                );
                Some(ready(()))
            }

            Err(RetryPolicyError::NotRetryable(err)) => {
                tracing::info!("Not retryable: {}", err);
                None
            }

            _ => None,
        }
    }

    fn clone_request(&mut self, req: &Req) -> Option<Req> {
        Some(req.clone())
    }
}
