use std::error::Error;
use tower::util::BoxCloneService;

pub type DynError = Box<dyn Error + Send + Sync + 'static>;
pub type DynService<Req,Res> = BoxCloneService<Req, Res, DynError>;