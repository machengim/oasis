use crate::entity::{state::State, token::Token};
use std::{future::Future, pin::Pin};
use tide::{Next, Request, Result};

pub fn load_token<'a>(
    mut req: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    let token = match Token::from_cookie(&req) {
        Ok(v) => v,
        _ => Token::default(),
    };

    Box::pin(async move {
        req.set_ext(token);
        Ok(next.run(req).await)
    })
}
