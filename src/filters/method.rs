use http::Method;

use ::filter::{And, Cons, Filter, filter_fn, filter_fn_cons, HList};
use ::never::Never;
use ::reject::{CombineRejection, Rejection};

/// Wrap a `Filter` in a new one that requires the request method to be `GET`.
pub fn get<F>(filter: F) -> And<impl Filter<Extract=(), Error=Rejection> + Copy, F>
where
    F: Filter + Clone,
    F::Extract: HList,
    F::Error: CombineRejection<Rejection>,
{
    method_is(|| &Method::GET).and(filter)
}

/// Wrap a `Filter` in a new one that requires the request method to be `POST`.
pub fn post<F>(filter: F) -> And<impl Filter<Extract=(), Error=Rejection> + Copy, F>
where
    F: Filter + Clone,
    F::Extract: HList,
    F::Error: CombineRejection<Rejection>,
{
    method_is(|| &Method::POST).and(filter)
}

/// Wrap a `Filter` in a new one that requires the request method to be `PUT`.
pub fn put<F>(filter: F) -> And<impl Filter<Extract=(), Error=Rejection> + Copy, F>
where
    F: Filter + Clone,
    F::Extract: HList,
    F::Error: CombineRejection<Rejection>,
{
    method_is(|| &Method::PUT).and(filter)
}

/// Wrap a `Filter` in a new one that requires the request method to be `DELETE`.
pub fn delete<F>(filter: F) -> And<impl Filter<Extract=(), Error=Rejection> + Copy, F>
where
    F: Filter + Clone,
    F::Extract: HList,
    F::Error: CombineRejection<Rejection>,
{
    method_is(|| &Method::DELETE).and(filter)
}

/// Extract the `Method` from the request.
pub fn method() -> impl Filter<Extract=Cons<Method>, Error=Never> + Copy {
    filter_fn_cons(|route| {
        Ok::<_, Never>(route.method().clone())
    })
}

fn method_is<F>(func: F) -> impl Filter<Extract=(), Error=Rejection> + Copy
where
    F: Fn() -> &'static Method + Copy,
{
    filter_fn(move |route| {
        let method = func();
        trace!("method::{:?}?: {:?}", method, route.method());
        if route.method() == method {
            Ok(())
        } else {
            Err(::reject::method_not_allowed())
        }
    })
}

