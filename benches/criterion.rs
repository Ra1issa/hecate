use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hecate::{
    moderator,
    platform,
    sender,
    receiver,
    forwarder,
    types::Test,
    utils,
};
use std::fs;
use core::time::Duration;
use ed25519_dalek::{Keypair, PublicKey};
use criterion::BenchmarkId;

pub fn generate_test_parameters()-> Test {

    let mut msg_vec = Vec::new();
    let mut mfrank_vec = Vec::new();
    let mut envelope_vec = Vec::new();
    let mut report_vec = Vec::new();

    let mut rng = rand::thread_rng();
    let id = utils::random_block(32, &mut rng);
    let m = moderator::setup_moderator(&mut rng);
    let mod_pk = (Keypair::from_bytes(&m.keypair).unwrap()).public;
    let p = platform::setup_platform(&mut rng);
    let plat_pk = (Keypair::from_bytes(&p.keypair).unwrap()).public;

    let token = moderator::generate_token(id.clone(), m.clone(), &mut rng);

    let msg_sizes = vec![10, 100, 250, 500, 750, 1000, 2500, 5000, 7500,  8000, 9000, 10000];
    for i in 0..msg_sizes.len(){
        let mut path = utils::get_data_path();
        let file = format!{"msgs/msg{:?}.txt", i};
        path.push(file);

        let msg: String= fs::read_to_string(path).unwrap();

        msg_vec.push(msg.clone());

        let (mfrank, com) = sender::generate_frank(msg.to_string(), token.clone(), &mut rng);
        let envelope = platform::sign_com(com.clone(), p.clone());
        let report = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);

        mfrank_vec.push(mfrank);
        envelope_vec.push(envelope);
        report_vec.push(report);
    }
    Test{
        id,
        msg: msg_vec,
        token,
        mfrank: mfrank_vec,
        envelope: envelope_vec,
        report: report_vec,
        moderator: m,
        mod_pk: mod_pk.to_bytes().to_vec(),
        platform: p,
        plat_pk: plat_pk.to_bytes().to_vec(),
        msg_sizes,
    }

}


pub fn criterion_benchmark_moderator(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Moderator");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    let plat_pk = PublicKey::from_bytes(&test.plat_pk).unwrap();

    for i in 0..test.msg_sizes.len() {
        let report = &test.report[i];
        let msg_size =  test.msg_sizes[i];
        group.bench_with_input(BenchmarkId::new("Inspect and Trace :: B", msg_size), &msg_size, |b,  &_s| {
            b.iter(||
                moderator::inspect(
                    black_box(report.clone()),
                    black_box(test.moderator.clone()),
                    black_box(plat_pk),
                )
            );
        });
    }

    let max_time = Duration::from_secs(5);
    group.measurement_time(max_time);

    let batch_sizes = [1, 10, 100, 1000, 10000];
    for i in 0..batch_sizes.len(){
        let batch_size = batch_sizes[i];

        if batch_size == 1000 {
            let max_time = Duration::from_secs(200);
            group.measurement_time(max_time);
        }else if batch_size == 10000 {
            let max_time = Duration::from_secs(1500);
            group.measurement_time(max_time);
        }

        group.bench_with_input(BenchmarkId::new("Generate Tokens :: ", batch_size), &batch_sizes, |b,  &_s| {
            b.iter(||
                moderator::generate_batch(
                    black_box(batch_sizes[i]),
                    black_box(test.id.clone()),
                    black_box(test.moderator.clone()),
                    black_box(&mut rng),
                )
            );
        });

    }
}

pub fn criterion_benchmark_sender(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let test = generate_test_parameters();

    let mut group = c.benchmark_group("Sender");
    group.significance_level(0.1).sample_size(300);


    for i in 0..test.msg_sizes.len() {
        let msg = test.msg[i].to_string();
        let msg_size =  test.msg_sizes[i];
        group.bench_with_input(BenchmarkId::new("Frank :: B", msg_size), &msg_size, |b,  &_s| {
            b.iter(||
                sender::generate_frank(
                    black_box(msg.clone()),
                    black_box(test.token.clone()),
                    black_box(&mut rng),
                )
            );
        });
    }
}

pub fn criterion_benchmark_receiver(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Receiver");
    group.significance_level(0.1).sample_size(300);

    let plat_pk = PublicKey::from_bytes(&test.plat_pk).unwrap();
    let mod_pk = PublicKey::from_bytes(&test.mod_pk).unwrap();

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    for i in 0..test.msg_sizes.len() {
        let mfrank = &test.mfrank[i];
        let envelope = &test.envelope[i];
        let msg_size =  test.msg_sizes[i];
        group.bench_with_input(BenchmarkId::new("Verify :: B", msg_size), &msg_size, |b, &_s| {
            b.iter(||
                receiver::check_message(
                    black_box(mfrank.clone()),
                    black_box(envelope.clone()),
                    black_box(mod_pk),
                    black_box(plat_pk),
                )
            );
        });
    }
}

pub fn criterion_benchmark_platform(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Platform");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    group.bench_function("Sign and Timestamp ", |b| b.iter(||
        platform::sign_com(
            black_box(test.envelope[0].com.clone()),
            black_box(test.platform.clone()),
        )
    ));
}

pub fn criterion_benchmark_forwarder(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Forwarder");
    group.significance_level(0.1).sample_size(300);

    let mut rng = rand::thread_rng();

    for i in 0..test.msg_sizes.len() {
        let mfrank = &test.mfrank[i];
        let envelope = &test.envelope[i];
        let msg_size =  test.msg_sizes[i];
        group.bench_with_input(BenchmarkId::new("Forward :: B", msg_size), &msg_size, |b,  &_s| {
            b.iter(||
                forwarder::forward(
                    black_box(Some(mfrank.clone())),
                    black_box(None),
                    black_box(envelope.clone()),
                    black_box(&mut rng),
                )
            );
        });
    }
}



criterion_group!(benches,
    criterion_benchmark_sender,
    // criterion_benchmark_receiver,
    // criterion_benchmark_forwarder,
    // criterion_benchmark_platform,
    // criterion_benchmark_moderator
);
criterion_main!(benches);
