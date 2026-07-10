use crate::utils::tower::retry_policy::RetryPolicyError;
#[derive(Clone)]
pub struct GitCloneRequest {
    pub url: String,
    pub path: String,
}
pub async fn git_clone(request: GitCloneRequest) -> Result<(), RetryPolicyError<git2::Error>> {
    git2::Repository::clone(&request.url, &request.path).map_err(|e|
        if e.class() == git2::ErrorClass::Net {
            RetryPolicyError::Retry(e)
        } else {
            RetryPolicyError::NotRetryable(e)
        }
    )?;
    Ok(())
}