use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hecate::hecate_lib::{
    moderator,
    platform,
    sender,
    receiver,
    types::Test,
    utils,
};
use std::fs;

pub fn generate_test_parameters()-> Test {
    let mut path = utils::get_project_path();
    path.push("data/msgs/message1.txt");
    let msg: String= fs::read_to_string(path).unwrap();

    let id = utils::random_block(32);
    let m = moderator::setup_moderator();
    let p = platform::setup_platform();

    let token = moderator::generate_token(id.clone(), m.clone());
    let (mfrank, com) = sender::generate_frank(msg.to_string(), token.clone());
    let envelope = platform::sign_com(com.clone(), p.clone());
    let report = receiver::check_message(mfrank.clone(), envelope.clone(), m.sig_pk, p.sig_pk);
    let trace = moderator::inspect(report.clone(), m.clone(), p.sig_pk);

    Test{
        id,
        msg,
        token,
        mfrank,
        envelope,
        report,
        trace,
        moderator: m,
        platform: p,
    }

}

pub fn criterion_benchmark(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Local Functions");
    group.significance_level(0.1).sample_size(300);
    group.bench_function("Moderator :: Token Generation", |b| b.iter(||
        moderator::generate_token(
            black_box(test.id.clone()),
            black_box(test.moderator.clone()
        )
    )));
    group.bench_function("Sender :: Franking ", |b| b.iter(||
        sender::generate_frank(
            black_box(test.msg.clone()),
            black_box(test.token.clone()
        )
    )));
    group.bench_function("Platform :: Timestamp ", |b| b.iter(||
        platform::sign_com(
            black_box(test.envelope.com.clone()),
            black_box(test.platform.clone()
        )
    )));
    group.bench_function("Receiver :: Verify ", |b| b.iter(||
        receiver::check_message(
            black_box(test.mfrank.clone()),
            black_box(test.envelope.clone()),
            black_box(test.moderator.sig_pk.clone()),
            black_box(test.platform.sig_pk.clone()),
        )
    ));
    group.bench_function("Moderator :: Inspect and Trace ", |b| b.iter(||
        moderator::inspect(
            black_box(test.report.clone()),
            black_box(test.moderator.clone()),
            black_box(test.platform.sig_pk.clone()),
        )
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
