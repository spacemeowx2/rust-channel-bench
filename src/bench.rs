use bencher::{black_box, Bencher};

const LIMIT: usize = 10;
const TIMES: usize = 100_000;

#[cfg(feature = "tokio02")]
fn get_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
}

#[cfg(feature = "asyncstd")]
fn get_rt() -> AsyncStdRt {
    AsyncStdRt
}

#[cfg(feature = "asyncstd")]
struct AsyncStdRt;
#[cfg(feature = "asyncstd")]
impl AsyncStdRt {
    fn spawn<F, T>(&mut self, future: F) -> async_std::task::JoinHandle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::spawn(future)
    }
    fn block_on<F, T>(&mut self, future: F) -> T
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::block_on(future)
    }
}

fn tokio_bounded(b: &mut Bencher) {
    use tokio::sync::mpsc;
    let mut rt = get_rt();

    b.iter(|| {
        let (mut tx, mut rx) = mpsc::channel::<()>(LIMIT);
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn tokio_unbounded(b: &mut Bencher) {
    use tokio::sync::mpsc;
    let mut rt = get_rt();

    b.iter(|| {
        let (tx, mut rx) = mpsc::unbounded_channel::<()>();
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn async_bounded(b: &mut Bencher) {
    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = async_channel::bounded::<()>(LIMIT);
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn async_unbounded(b: &mut Bencher) {
    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = async_channel::unbounded::<()>();
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn futures_bounded(b: &mut Bencher) {
    use futures::channel::mpsc;
    use futures::prelude::*;

    let mut rt = get_rt();

    b.iter(|| {
        let (mut tx, mut rx) = mpsc::channel::<()>(LIMIT);
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.next().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn futures_unbounded(b: &mut Bencher) {
    use futures::channel::mpsc;
    use futures::prelude::*;

    let mut rt = get_rt();

    b.iter(|| {
        let (mut tx, mut rx) = mpsc::unbounded::<()>();
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.next().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn crossfire_mpsc_bounded(b: &mut Bencher) {
    use crossfire::mpsc;

    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = mpsc::bounded_future_both::<()>(LIMIT);
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn crossfire_mpsc_unbounded(b: &mut Bencher) {
    use crossfire::mpsc;

    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = mpsc::unbounded_future::<()>();
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn crossfire_mpmc_bounded(b: &mut Bencher) {
    use crossfire::mpmc;

    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = mpmc::bounded_future_both::<()>(LIMIT);
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).await.unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

fn crossfire_mpmc_unbounded(b: &mut Bencher) {
    use crossfire::mpmc;

    let mut rt = get_rt();

    b.iter(|| {
        let (tx, rx) = mpmc::unbounded_future::<()>();
        let t1 = rt.spawn(async move {
            for _ in 0..TIMES {
                tx.send(()).unwrap();
            }
        });
        let t2 = rt.spawn(async move {
            for _ in 0..TIMES {
                black_box(rx.recv().await.unwrap());
            }
        });
        rt.block_on(async {
            t1.await;
            t2.await;
        });
    });
}

bencher::benchmark_group!(
    tokio_channels,
    tokio_bounded,
    tokio_unbounded
);

bencher::benchmark_group!(
    async_channels,
    async_bounded,
    async_unbounded
);

bencher::benchmark_group!(
    futures_channels,
    futures_bounded,
    futures_unbounded
);

bencher::benchmark_group!(
    crossfire_mpsc_channels,
    crossfire_mpsc_bounded,
    crossfire_mpsc_unbounded
);

bencher::benchmark_group!(
    crossfire_mpmc_channels,
    crossfire_mpmc_bounded,
    crossfire_mpmc_unbounded
);

bencher::benchmark_main!(tokio_channels, async_channels, futures_channels, crossfire_mpsc_channels, crossfire_mpmc_channels);
