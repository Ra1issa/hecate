use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hecate::{
    moderator,
    platform,
    sender,
    receiver,
    forwarder,
    tests,
    types::{Moderator, Platform, Token, Test},
    utils,
};
use std::fs;
use core::time::Duration;
use ed25519_dalek::{Keypair, PublicKey};
use criterion::BenchmarkId;
use rand::rngs::OsRng;

pub fn generate_test_parameters()-> Test {

    let mut msg_vec = Vec::new();
    let mut mfrank_vec = Vec::new();
    let mut envelope_vec = Vec::new();
    let mut report_vec = Vec::new();


    let mut rng = OsRng{};
    let id = tests::ID.to_vec();
    let m: Moderator = bincode::deserialize(tests::MOD).unwrap();
    let mod_pk = (Keypair::from_bytes(&m.keypair).unwrap()).public;
    let p: Platform = bincode::deserialize(tests::PLAT).unwrap();
    let plat_pk = (Keypair::from_bytes(&p.keypair).unwrap()).public;

    let token:Token = bincode::deserialize(tests::TOKEN).unwrap();
    let msg_sizes = vec![10, 100, 250, 500, 750, 1000, 2500, 5000, 7500,  8000, 9000, 10000];
    for i in 0..msg_sizes.len(){
        let mut path = utils::get_data_path();
        let file = format!{"msgs/msg{:?}.txt", i};
        path.push(file);

        let msg: String= fs::read_to_string(path).unwrap();

        msg_vec.push(msg.clone());

        let (mfrank, com) = sender::generate_frank(msg.to_string(), token.clone(), &mut rng);
        let envelope = platform::sign_com(com.clone(), p.clone());
        let new_mfrank = receiver::check_authorship(mfrank.clone(), envelope.clone());
        let report = receiver::check_message(new_mfrank.clone(), mod_pk, plat_pk);

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


pub fn moderator(c: &mut Criterion) {
    let mut rng = OsRng{};
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Moderator");
    group.sample_size(300);

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

    let max_time = Duration::from_secs(10);
    group.measurement_time(max_time);

    let batch_sizes = [1, 10, 100, 500, 1000, 2500, 5000, 7500, 10000];
    for i in 0..batch_sizes.len(){
        let batch_size = batch_sizes[i];

        if batch_size >= 1000 {
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

pub fn sender(c: &mut Criterion) {
    let mut rng = OsRng{};
    let test = generate_test_parameters();

    let mut group = c.benchmark_group("Sender");
    group.sample_size(300);

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    for i in 0..test.msg_sizes.len() {
        let msg = test.msg[i].to_string();
        let msg_size =  test.msg_sizes[i];
        let nonce = utils::random_block(12, &mut rng);
        let aad = "".as_bytes();
        let mut enc_sk = vec![0 as u8; 32];
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

pub fn receiver(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Receiver");
    group.sample_size(300);

    let plat_pk = PublicKey::from_bytes(&test.plat_pk).unwrap();
    let mod_pk = PublicKey::from_bytes(&test.mod_pk).unwrap();

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    for i in 0..test.msg_sizes.len() {
        let mfrank = &test.mfrank[i];
        let envelope = &test.envelope[i];
        let msg_size =  test.msg_sizes[i];
        group.bench_with_input(BenchmarkId::new("Verify :: B", msg_size), &msg_size, |b, &_s| {
            b.iter(||{
                let new_mfrank = receiver::check_authorship(
                    black_box(mfrank.clone()),
                    black_box(envelope.clone()),
                );
                receiver::check_message(
                    black_box(new_mfrank.clone()),
                    black_box(mod_pk),
                    black_box(plat_pk),
                )
            });
        });
    }
}

pub fn platform(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Platform");
    group.sample_size(300);

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    group.bench_function("Sign and Timestamp ", |b| b.iter(||
        platform::sign_com(
            black_box(test.envelope[0].com.clone()),
            black_box(test.platform.clone()),
        )
    ));
}

pub fn forwarder(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Forwarder");
    group.sample_size(300);

    let mut rng = OsRng{};

    for i in 0..test.msg_sizes.len() {
        let mfrank = &test.mfrank[i];
        let envelope = &test.envelope[i];
        let msg_size =  test.msg_sizes[i];
        let new_mfrank = receiver::check_authorship(mfrank.clone(), envelope.clone());
        group.bench_with_input(BenchmarkId::new("Forward :: B", msg_size), &msg_size, |b,  &_s| {
            b.iter(||
                forwarder::forward(
                    black_box(new_mfrank.clone()),
                    black_box(&mut rng),
                )
            );
        });
    }
}



criterion_group!(benches,
    sender,
    receiver,
    forwarder,
    platform,
    moderator
);
criterion_main!(benches);
