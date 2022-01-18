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
use core::time::Duration;

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


pub fn criterion_benchmark_moderator(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Moderator");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(1000);
    group.measurement_time(max_time);

    let batch_sizes = [1, 10, 100, 1000, 10000];
    for i in 0..batch_sizes.len(){
        let test_name = format!{"Moderator :: Generate {:?} tokens", batch_sizes[i].to_string()};
        group.bench_function(test_name, |b| b.iter(||
            moderator::generate_batch(
                black_box(batch_sizes[i]),
                black_box(test.id.clone()),
                black_box(test.moderator.clone()),
            )
        ));
    }

    group.bench_function("Moderator :: Inspect and Trace ", |b| b.iter(||
        moderator::inspect(
            black_box(test.report.clone()),
            black_box(test.moderator.clone()),
            black_box(test.platform.sig_pk.clone()),
        )
    ));

}

pub fn criterion_benchmark_sender(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Sender");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(1000);
    group.measurement_time(max_time);

    static B: usize = 1;
    static KB: usize = 1024;

    let msg_sizes = [10 * B, 100 * B, KB, 10 * KB, 100 * KB];
    for i in 0..4 as usize{
        let mut path = utils::get_project_path();
        let file = format!{"data/msgs/message{:?}.txt", i};
        path.push(file);

        let msg: String= fs::read_to_string(path).unwrap();
        let test_name = format!{"Frank :: Plaintext Size {:?}B", msg_sizes[i].to_string()};
        group.bench_function(test_name, |b| b.iter(||
            sender::generate_frank(
                black_box(msg.clone()),
                black_box(test.token.clone()),
            )
        ));
    }
}

pub fn criterion_benchmark_receiver(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Receiver");
    group.significance_level(0.1).sample_size(300);

    group.bench_function("Verify Message", |b| b.iter(||
        receiver::check_message(
            black_box(test.mfrank.clone()),
            black_box(test.envelope.clone()),
            black_box(test.moderator.sig_pk.clone()),
            black_box(test.platform.sig_pk.clone()),
        )
    ));

}

pub fn criterion_benchmark_platform(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Platform");
    group.significance_level(0.1).sample_size(300);

    group.bench_function("Sign and Timestamp ", |b| b.iter(||
        platform::sign_com(
            black_box(test.envelope.com.clone()),
            black_box(test.platform.clone()
        )
    )));
}



criterion_group!(benches,
    criterion_benchmark_moderator,
    criterion_benchmark_sender,
    criterion_benchmark_receiver,
    criterion_benchmark_platform);
criterion_main!(benches);
