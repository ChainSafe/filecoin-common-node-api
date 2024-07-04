//! Primitives for composing [`Service`]s

use std::{
    convert::Infallible,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{ready, Context, Poll},
};

use bytes::Bytes;
use futures::{
    future::{self, Either, Ready},
    Future, Sink,
};
use http_body::Body;
use http_body_util::BodyExt as _;
use pin_project_lite::pin_project;
use tower::{util::Oneshot, Service, ServiceExt as _};

const PANIC_MSG: &str = "future polled after completion";

#[derive(Debug, Clone)]
pub struct MapErr<S, F> {
    svc: S,
    map: F,
}

impl<S, F> MapErr<S, F> {
    pub fn new(svc: S, map: F) -> Self {
        Self { svc, map }
    }
}

impl<S, F, Req, Resp, E1, E2> Service<Req> for MapErr<S, F>
where
    S: Service<Req, Response = Resp, Error = E1> + Clone,
    F: FnMut(E1) -> E2 + Clone,
{
    type Response = Resp;
    type Error = E2;
    type Future = MapErrFuture<S, Req, F>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Req) -> Self::Future {
        MapErrFuture {
            fut: self.svc.clone().oneshot(req),
            map: self.map.clone(),
        }
    }
}
pin_project! {
    pub struct MapErrFuture<S: Service<Req>, Req, F> {
        #[pin]
        fut: Oneshot<S, Req>,
        map: F
    }
}

impl<S: Service<Req, Error = E1, Response = Resp>, Req, F, Resp, E1, E2> Future
    for MapErrFuture<S, Req, F>
where
    F: FnMut(E1) -> E2,
{
    type Output = Result<Resp, E2>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        Poll::Ready(match ready!(this.fut.poll(cx)) {
            Ok(it) => Ok(it),
            Err(e) => Err((this.map)(e)),
        })
    }
}

/// A [`Service`] which replaces inner [`Service::Error`]s with a default [`Service::Response`].
#[derive(Debug, Clone)]
pub struct UnwrapOrElse<S, F> {
    svc: S,
    map: F,
}

impl<S, F> UnwrapOrElse<S, F> {
    pub fn new(svc: S, map: F) -> Self {
        Self { svc, map }
    }
}

impl<S, F, Req, Resp, E> Service<Req> for UnwrapOrElse<S, F>
where
    S: Service<Req, Response = Resp, Error = E> + Clone,
    F: FnMut(E) -> Resp + Clone,
{
    type Response = Resp;
    type Error = Infallible;
    type Future = UnwrapOrElseFuture<S, Req, F>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Req) -> Self::Future {
        UnwrapOrElseFuture {
            fut: self.svc.clone().oneshot(req),
            map: self.map.clone(),
        }
    }
}
pin_project! {
    pub struct UnwrapOrElseFuture<S: Service<Req>, Req, F> {
        #[pin]
        fut: Oneshot<S, Req>,
        map: F
    }
}

impl<S: Service<Req, Error = E, Response = Resp>, Req, F, Resp, E> Future
    for UnwrapOrElseFuture<S, Req, F>
where
    F: FnMut(E) -> Resp,
{
    type Output = Result<Resp, Infallible>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        Poll::Ready(match ready!(this.fut.poll(cx)) {
            Ok(it) => Ok(it),
            Err(e) => Ok((this.map)(e)),
        })
    }
}

/// An empty [`Service`] which never responds.
#[derive(Debug)]
pub struct Todo<Req, Resp, E>(
    #[allow(clippy::type_complexity)] PhantomData<fn() -> (Req, Resp, E)>,
);

impl<Req, Resp, E> Clone for Todo<Req, Resp, E> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<Req, Resp, E> Todo<Req, Resp, E> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<Req, Resp, E> Service<Req> for Todo<Req, Resp, E> {
    type Response = Resp;
    type Error = E;
    type Future = future::Pending<Result<Resp, E>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Pending
    }

    fn call(&mut self, _: Req) -> Self::Future {
        future::pending()
    }
}

/// An transparent [`Service`] which forwards to an inner service.
#[derive(Debug)]
pub struct Assert<Req, Resp, E, S: Service<Req, Response = Resp, Error = E>> {
    svc: S,
    _phantom: PhantomData<fn() -> Req>,
}

impl<Req, Resp, E, S: Service<Req, Response = Resp, Error = E>> Service<Req>
    for Assert<Req, Resp, E, S>
{
    type Response = Resp;
    type Error = E;
    type Future = S::Future;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }
    fn call(&mut self, req: Req) -> Self::Future {
        self.svc.call(req)
    }
}

impl<Req, Resp, E, S: Service<Req, Response = Resp, Error = E>> Clone for Assert<Req, Resp, E, S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            svc: self.svc.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<Req, Resp, E, S: Service<Req, Response = Resp, Error = E>> Assert<Req, Resp, E, S> {
    #[allow(unused)]
    pub fn new(svc: S) -> Self {
        Self {
            svc,
            _phantom: PhantomData,
        }
    }
}

/// A simple [`Service`] which attempts to transform a request before passing it
/// to another service.
#[derive(Debug, Clone)]
pub struct TryMapRequest<S, F> {
    svc: S,
    try_map: F,
}

impl<S, F> TryMapRequest<S, F> {
    pub fn new(svc: S, try_map: F) -> Self {
        Self { svc, try_map }
    }
}

impl<S, R1, R2, F> Service<R1> for TryMapRequest<S, F>
where
    F: FnMut(R1) -> Result<R2, S::Error>,
    S: Service<R2>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<Ready<Result<S::Response, S::Error>>, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    fn call(&mut self, req: R1) -> Self::Future {
        match (self.try_map)(req) {
            Err(e) => Either::Left(future::ready(Err(e))),
            Ok(r2) => Either::Right(self.svc.call(r2)),
        }
    }
}

/// A [`Service`] that collects [`http::Request`]s in front of another service
#[derive(Debug, Clone)]
pub struct CollectRequest<S>(pub S);

impl<S, B, Resp, E> Service<http::Request<B>> for CollectRequest<S>
where
    S: Service<http::Request<Bytes>, Response = Resp, Error = E> + Clone,
    B: Body,
    B::Error: Into<S::Error>,
{
    type Response = S::Response;
    type Error = E;
    type Future = CollectRequestFuture<B, S>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        // In case the inner service has state that's driven to readiness and
        // not tracked by clones (such as `Buffer`), pass the version we have
        // already called `poll_ready` on into the future, and leave its clone
        // behind.
        let clone = self.0.clone();
        CollectRequestFuture::new(req, mem::replace(&mut self.0, clone))
    }
}

pin_project! {
    pub struct CollectRequestFuture<B: Body, S: Service<http::Request<Bytes>>> {
        parts: Option<http::request::Parts>,
        #[pin]
        collect: Option<http_body_util::combinators::Collect<B>>,
        service: Option<S>,
        #[pin]
        oneshot: Option<tower::util::Oneshot<S, http::Request<Bytes>>>
    }
}

impl<B: Body, S: Service<http::Request<Bytes>>> CollectRequestFuture<B, S> {
    fn new(request: http::Request<B>, service: S) -> Self {
        let (parts, body) = request.into_parts();
        Self {
            parts: Some(parts),
            collect: Some(body.collect()),
            service: Some(service),
            oneshot: None,
        }
    }
}

impl<B: Body, S: Service<http::Request<Bytes>>> Future for CollectRequestFuture<B, S>
where
    B::Error: Into<S::Error>,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        if let Some(collect) = this.collect.as_mut().as_pin_mut() {
            let body = ready!(collect.poll(cx));
            this.collect.set(None);
            match body {
                Ok(body) => {
                    let request = http::Request::from_parts(
                        this.parts.take().expect(PANIC_MSG),
                        body.to_bytes(),
                    );
                    let svc = this.service.take().expect(PANIC_MSG);
                    this.oneshot.set(Some(svc.oneshot(request)))
                }
                Err(e) => return Poll::Ready(Err(e.into())),
            }
        }
        match this.oneshot.as_pin_mut() {
            Some(oneshot) => oneshot.poll(cx),
            None => panic!("{}", PANIC_MSG),
        }
    }
}

/// A [`Service`] that collects [`http::Request`]s in front of another service
#[derive(Debug, Clone)]
pub struct CollectResponse<S>(pub S);

impl<S, Req, B> Service<Req> for CollectResponse<S>
where
    S: Service<Req, Response = http::Response<B>>,
    B: Body,
    B::Error: Into<S::Error>,
{
    type Response = http::Response<Bytes>;
    type Error = S::Error;
    type Future = CollectResponseFuture<S::Future, B>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        CollectResponseFuture::new(self.0.call(req))
    }
}

pin_project! {
    pub struct CollectResponseFuture<F, B: Body> {
        #[pin]
        svc_fut: Option<F>,
        parts: Option<http::response::Parts>,
        #[pin]
        collect: Option<http_body_util::combinators::Collect<B>>,
    }
}

impl<F, B: Body> CollectResponseFuture<F, B> {
    fn new(svc_fut: F) -> Self {
        Self {
            svc_fut: Some(svc_fut),
            parts: None,
            collect: None,
        }
    }
}

impl<F, B: Body, E> Future for CollectResponseFuture<F, B>
where
    F: Future<Output = Result<http::Response<B>, E>>,
    B::Error: Into<E>,
{
    type Output = Result<http::Response<Bytes>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        if let Some(svc_fut) = this.svc_fut.as_mut().as_pin_mut() {
            let res = ready!(svc_fut.poll(cx));
            this.svc_fut.set(None);
            match res {
                Ok(resp) => {
                    let (parts, body) = resp.into_parts();
                    *this.parts = Some(parts);
                    this.collect.set(Some(body.collect()));
                }
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
        match this.collect.as_mut().as_pin_mut() {
            Some(collect) => {
                let res = ready!(collect.poll(cx));
                this.collect.set(None);
                match res {
                    Ok(body) => Poll::Ready(Ok(http::Response::from_parts(
                        this.parts.take().expect(PANIC_MSG),
                        body.to_bytes(),
                    ))),
                    Err(e) => Poll::Ready(Err(e.into())),
                }
            }
            None => panic!("{}", PANIC_MSG),
        }
    }
}

/// A [`Service`] that forwards responses to a [`Sink`].
///
/// - If the sink is closed (i.e, [`Sink::poll_ready`] returns an [`Err`]),
///   the response will not be forwarded to it, and the service will continue.
/// - Responses will wait for the sink to [flush](Sink::poll_flush),
///   and [`Err`]s will likewise be ignored.
#[derive(Debug, Clone)]
pub struct TeeOk<S, T> {
    svc: S,
    tee: T,
}

impl<S, T> TeeOk<S, T> {
    pub fn new(svc: S, tee: T) -> Self {
        Self { svc, tee }
    }
}

impl<S, SvcReq, SvcResp, SvcErr, T> Service<SvcReq> for TeeOk<S, T>
where
    S: Service<SvcReq, Response = SvcResp, Error = SvcErr>,
    T: Sink<SvcResp> + Clone,
    SvcResp: Clone,
{
    type Response = SvcResp;
    type Error = SvcErr;
    type Future = TeeOkFuture<S::Future, SvcResp, T>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    fn call(&mut self, req: SvcReq) -> Self::Future {
        TeeOkFuture::new(self.svc.call(req), self.tee.clone())
    }
}

pin_project! {
    pub struct TeeOkFuture<SvcFut, SvcResp, T> {
        #[pin]
        svc_fut: Option<SvcFut>,
        output: Option<SvcResp>,
        #[pin]
        tee: T,
        state: State
    }
}

impl<SvcFut, SvcResp, T> TeeOkFuture<SvcFut, SvcResp, T> {
    fn new(svc_fut: SvcFut, tee: T) -> Self {
        Self {
            svc_fut: Some(svc_fut),
            output: None,
            tee,
            state: State::PollSvcFut,
        }
    }
}

#[derive(Debug)]
enum State {
    PollSvcFut,
    PollTeeReady,
    PollFlush,
    Fin,
}

impl<SvcFut, SvcResp, SvcErr, T> Future for TeeOkFuture<SvcFut, SvcResp, T>
where
    SvcFut: Future<Output = Result<SvcResp, SvcErr>>,
    T: Sink<SvcResp>,
    SvcResp: Clone,
{
    type Output = Result<SvcResp, SvcErr>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.state {
                State::PollSvcFut => {
                    let svc_fut = this.svc_fut.as_mut().as_pin_mut().expect(PANIC_MSG);
                    match ready!(svc_fut.poll(cx)) {
                        Ok(ok) => {
                            *this.state = State::PollTeeReady;
                            *this.output = Some(ok)
                        }
                        Err(e) => {
                            *this.state = State::Fin;
                            return Poll::Ready(Err(e));
                        }
                    }
                }
                State::PollTeeReady => {
                    let mut tee = this.tee.as_mut();
                    match ready!(tee.as_mut().poll_ready(cx)) {
                        Ok(()) => match tee.start_send(this.output.clone().expect(PANIC_MSG)) {
                            Ok(()) => {
                                *this.state = State::PollFlush;
                            }
                            Err(_ignore_sink_err) => {
                                *this.state = State::Fin;
                                return Poll::Ready(Ok(this.output.take().expect(PANIC_MSG)));
                            }
                        },
                        Err(_ignore_sink_err) => {
                            *this.state = State::Fin;
                            return Poll::Ready(Ok(this.output.take().expect(PANIC_MSG)));
                        }
                    }
                }
                State::PollFlush => {
                    let _ignore_sink_err = ready!(this.tee.poll_flush(cx));
                    *this.state = State::Fin;
                    return Poll::Ready(Ok(this.output.take().expect(PANIC_MSG)));
                }
                State::Fin => panic!("{}", PANIC_MSG),
            }
        }
    }
}

#[test]
fn tee_ok() {
    let (tee, teed) = futures::channel::mpsc::unbounded();
    let resp = futures::executor::block_on_stream(tower::ServiceExt::call_all(
        TeeOk::new(
            tower::util::service_fn(
                |i: u32| async move { Ok::<_, core::convert::Infallible>(i * 2) },
            ),
            tee,
        ),
        futures::stream::iter([1, 2, 3, 4]),
    ))
    .map(Result::unwrap);
    itertools::assert_equal(resp, [2, 4, 6, 8]);
    itertools::assert_equal(futures::executor::block_on_stream(teed), [2, 4, 6, 8]);
}

/// A [`Service`] that pairs a request with its response.
#[derive(Debug, Clone)]
pub struct Correlate<S>(pub S);

impl<S, SvcReq, SvcResp, SvcErr> Service<SvcReq> for Correlate<S>
where
    S: Service<SvcReq, Response = SvcResp, Error = SvcErr>,
    SvcReq: Clone,
{
    type Response = (SvcReq, SvcResp);

    type Error = SvcErr;

    type Future = CorrelateFuture<S::Future, SvcReq>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: SvcReq) -> Self::Future {
        CorrelateFuture::new(self.0.call(req.clone()), req)
    }
}

pin_project! {
    pub struct CorrelateFuture<F, A> {
        #[pin]
        inner: F,
        attach: Option<A>,
    }
}

impl<F, A> CorrelateFuture<F, A> {
    fn new(inner: F, attach: A) -> Self {
        Self {
            inner,
            attach: Some(attach),
        }
    }
}

impl<F, A, T, E> Future for CorrelateFuture<F, A>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<(A, T), E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.inner.poll(cx));
        let a = this.attach.take().expect(PANIC_MSG);
        Poll::Ready(match res {
            Ok(t) => Ok((a, t)),
            Err(e) => Err(e),
        })
    }
}
