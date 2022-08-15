use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use test_grpc::proto::{echo_client::EchoClient, EchoRequest};
use tokio::runtime::Runtime;
use tonic::transport::Channel;

fn make_client(
    endpoints: impl IntoIterator<Item = &'static str>,
) -> (Runtime, EchoClient<Channel>) {
    let basic_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let channel = basic_rt.block_on(async {
        let endpoints = endpoints.into_iter().map(|a| Channel::from_static(a));
        let channel = Channel::balance_list(endpoints);
        EchoClient::new(channel)
    });

    (basic_rt, channel)
}

#[inline]
fn request_blocking(rt: &Runtime, client: &mut EchoClient<Channel>, message: String) {
    rt.block_on(async {
        let request = tonic::Request::new(EchoRequest { message });
        let _response = client.unary_echo(request).await.unwrap();
    });
}

fn grpc_sha512(c: &mut Criterion) {
    let mut group = c.benchmark_group("grpc_sha512");
    group.sample_size(1000);

    for size in [1, 2, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128] {
        let size = size * 1024;
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("net", size), &size, |b, &size| {
            let message: &str = &"abcd".repeat(size / 4);
            let (rt, mut client) = make_client(["http://localhost:5143"]);
            b.iter(|| request_blocking(&rt, &mut client, message.into()))
        });

        group.bench_with_input(BenchmarkId::new("rust", size), &size, |b, &size| {
            let message: &str = &"abcd".repeat(size / 4);
            let (rt, mut client) = make_client(["http://[::1]:50051"]);
            b.iter(|| request_blocking(&rt, &mut client, message.into()))
        });
    }
    group.finish();
}

criterion_group!(benches, grpc_sha512);
criterion_main!(benches);
