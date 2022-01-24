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

pub fn generate_test_parameters()-> Test {
    let mut path = utils::get_data_path();
    path.push("msgs/msg6.txt");
    let msg: String= fs::read_to_string(path).unwrap();

    let mut rng = rand::thread_rng();
    let id = utils::random_block(32, &mut rng);
    let m = moderator::setup_moderator(&mut rng);
    let mod_pk = (Keypair::from_bytes(&m.keypair).unwrap()).public;
    let p = platform::setup_platform(&mut rng);
    let plat_pk = (Keypair::from_bytes(&p.keypair).unwrap()).public;

    let token = moderator::generate_token(id.clone(), m.clone(), &mut rng);
    let (mfrank, com) = sender::generate_frank(msg.to_string(), token.clone(), &mut rng);
    let envelope = platform::sign_com(com.clone(), p.clone());
    let report = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);
    let trace = moderator::inspect(report.clone(), m.clone(), plat_pk);

    Test{
        id,
        msg,
        token,
        mfrank,
        envelope,
        report,
        trace,
        moderator: m,
        mod_pk: mod_pk.to_bytes().to_vec(),
        platform: p,
        plat_pk: plat_pk.to_bytes().to_vec(),
    }

}


pub fn criterion_benchmark_moderator(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Moderator");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(10);
    group.measurement_time(max_time);

    let plat_pk = PublicKey::from_bytes(&test.plat_pk).unwrap();

    group.bench_function("Moderator :: Inspect and Trace ", |b| b.iter(||
        moderator::inspect(
            black_box(test.report.clone()),
            black_box(test.moderator.clone()),
            black_box(plat_pk),
        )
    ));

    let max_time = Duration::from_secs(5);
    group.measurement_time(max_time);

    let batch_sizes = [1, 10, 100, 1000, 10000];
    for i in 0..batch_sizes.len(){
        if batch_sizes[i] == 1000 {
            let max_time = Duration::from_secs(200);
            group.measurement_time(max_time);
        }else if batch_sizes[i] == 10000 {
            let max_time = Duration::from_secs(1500);
            group.measurement_time(max_time);
        }

        let test_name = format!{"Moderator :: Generate {:?} tokens", batch_sizes[i].to_string()};
        group.bench_function(test_name, |b| b.iter(||
            moderator::generate_batch(
                black_box(batch_sizes[i]),
                black_box(test.id.clone()),
                black_box(test.moderator.clone()),
                black_box(&mut rng),
            )
        ));
    }
}

pub fn criterion_benchmark_sender(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let test = generate_test_parameters();

    let mut group = c.benchmark_group("Sender");
    group.significance_level(0.1).sample_size(300);


    let msg_sizes = [10.0, 50.0, 100.0, 250.0, 500.0, 750.0, 1.0, 2.5, 5.0, 7.5, 10.0];
    for i in 0..msg_sizes.len(){

        let mut path = utils::get_data_path();
        let file = format!{"msgs/msg{:?}.txt", i};
        path.push(file);

        let msg: String= fs::read_to_string(path).unwrap();
        let mut test_name = str::replace(&msg_sizes[i].to_string(),".0", "");
        if i < 6{
            test_name = format!{"Frank :: Plaintext Size {:?}B", test_name};
        }else{
            test_name = format!{"Frank :: Plaintext Size {:?}KB", test_name};
        }
        group.bench_function(test_name, |b| b.iter(||
            sender::generate_frank(
                black_box(msg.clone()),
                black_box(test.token.clone()),
                black_box(&mut rng),
            )
        ));
    }
}

pub fn criterion_benchmark_receiver(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Receiver");
    group.significance_level(0.1).sample_size(300);
    let plat_pk = PublicKey::from_bytes(&test.plat_pk).unwrap();
    let mod_pk = PublicKey::from_bytes(&test.mod_pk).unwrap();
    group.bench_function("Verify Message", |b| b.iter(||
        receiver::check_message(
            black_box(test.mfrank.clone()),
            black_box(test.envelope.clone()),
            black_box(mod_pk),
            black_box(plat_pk),
        )
    ));

}

pub fn criterion_benchmark_platform(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Platform");
    group.significance_level(0.1).sample_size(300);

    let max_time = Duration::from_secs(15);
    group.measurement_time(max_time);

    group.bench_function("Sign and Timestamp ", |b| b.iter(||
        platform::sign_com(
            black_box(test.envelope.com.clone()),
            black_box(test.platform.clone()),
        )
    ));
}

pub fn criterion_benchmark_forwarder(c: &mut Criterion) {
    let test = generate_test_parameters();
    let mut group = c.benchmark_group("Forwarder");
    group.significance_level(0.1).sample_size(300);

    let mut rng = rand::thread_rng();
    group.bench_function("Forward", |b| b.iter(||
        forwarder::forward(
            black_box(Some(test.mfrank.clone())),
            black_box(None),
            black_box(test.envelope.clone()),
            black_box(&mut rng),
        )
    ));
}



criterion_group!(benches,
    criterion_benchmark_sender,
    criterion_benchmark_receiver,
    criterion_benchmark_forwarder,
    criterion_benchmark_platform,
    criterion_benchmark_moderator
);
criterion_main!(benches);
