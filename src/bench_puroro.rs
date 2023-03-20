use criterion::{black_box, Criterion};
use puroro::{Message, MessageView};
use std::io::Read;
use std::ops::Deref;

pub trait Serialize {
    type Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize,
    <T as Serialize>::Message: Message + Deref,
    <<T as Serialize>::Message as Deref>::Target: MessageView,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/puroro", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.serialize_pb().to_bytes(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message.to_bytes(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    message.to_bytes(&mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box::<T::Message>(
                Message::from_bytes_iter(black_box(deserialize_buffer.bytes())).unwrap(),
            );
        })
    });

    crate::bench_size(name, "puroro", deserialize_buffer.as_slice());

    group.finish();
}
