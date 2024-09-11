use reed_solomon_erasure;
//use hex_literal;
use hex;
use std::mem;

fn main() {
    //replicating_sage_test();
    gav_test();
}

fn gav_test() {
    let original = (0..684).map(|i| i as u8).collect::<Vec<_>>();
    let mut data: Vec<Vec<[u8; 2]>> = original
        .chunks_exact(2)
        .map(|x| x.chunks_exact(2).map(|s| [s[0], s[1]]).collect())
        .collect();

    let data_rs_erasure = encode_using_reed_solomon_erasure(data.clone(), 342, 681);
    let data_rs_simd = encode_using_reed_solomon_simd(data, 342, 681);
    for i in 0..1023 {
        let cur_shard_erasure = &data_rs_erasure[i]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()[..];
        let cur_shard_simd = &data_rs_simd[i]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()[..];

        println!(
            "{i}: {}, { }",
            hex::encode(cur_shard_erasure),
            hex::encode(cur_shard_simd)
        );
        //assert_eq!(cur_shard_erasure[0], cur_shard_simd[0]);
        //assert_eq!(cur_shard_erasure[1], cur_shard_simd[1]);
    }
}
fn replicating_sage_test() {
    //        m1 = From_V(int_to_field_vec(1, 16))
    //        m2 = From_V(int_to_field_vec(2, 16))
    let m1: [u8; 2] = [1, 0];
    let m2: [u8; 2] = [2, 0];
    let mut message: Vec<Vec<[u8; 2]>> = vec![vec![m1], vec![m2]];
    message.reserve(4);
    println!("{:?}", message);

    // assert(field_element_into_cantor_int(py(cantor_basis[1])) == 0)
    // assert(field_element_into_cantor_int(py(cantor_basis[1]+1)) == 3)
    // assert(field_element_into_cantor_int(py(cantor_basis[2])) == 14)
    // assert(field_element_into_cantor_int(py(cantor_basis[2]+1)) == 13)

    let data_rs_erasure = encode_using_reed_solomon_erasure(message.clone(), 2, 4);
    let data_rs_simd = encode_using_reed_solomon_simd(message, 2, 4);
    for i in 0..6 {
        println!(
            "{i}: {}",
            hex::encode(
                &data_rs_erasure[i]
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>()[..]
            )
        );
        println!(
            "{i}: {}",
            hex::encode(
                &data_rs_simd[i]
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>()[..]
            )
        );
    }
}

fn next_power_of_2(n: u16) -> u16 {
    //if n is a power of 2 just return it.
    let last_power_of_2 = 1 << 15 - n.leading_zeros();
    return match last_power_of_2 == n {
        true => n,
        _ => last_power_of_2 << 1,
    };
}

fn encode_using_reed_solomon_erasure(
    mut data: Vec<Vec<[u8; 2]>>,
    no_of_data_shards: usize,
    no_of_recovery_shards: usize,
) -> Vec<Vec<[u8; 2]>> {
    //reed solomon simd enforce to smaller power of 2 less than current data shard so we need to artifically guarantee that.

    let extended_no_of_data_shards = next_power_of_2(no_of_data_shards as u16) as usize;

    data.reserve(extended_no_of_data_shards + no_of_recovery_shards);
    for _ in no_of_data_shards..(no_of_recovery_shards + extended_no_of_data_shards) {
        data.push(vec![[0, 0]; 1]);
    }

    let r = reed_solomon_erasure::galois_16_cantor::ReedSolomon::new(
        extended_no_of_data_shards,
        no_of_recovery_shards,
    )
    .unwrap();

    // Construct the parity shards
    r.encode(&mut data).unwrap();

    let mut shrinked_data = data.clone();
    for i in 0..no_of_recovery_shards {
        shrinked_data[no_of_data_shards + i] = data[extended_no_of_data_shards + i].clone();
    }

    shrinked_data
}

fn encode_using_reed_solomon_simd(
    mut data: Vec<Vec<[u8; 2]>>,
    no_of_data_shards: usize,
    no_of_recovery_shards: usize,
) -> Vec<Vec<[u8; 2]>> {
    // This initializes all the needed tables.
    reed_solomon_simd::engine::DefaultEngine::new();

    // CREATE ORIGINAL
    let shard_bytes: usize = 64;

    let mut original = vec![vec![0u8; shard_bytes]; no_of_data_shards];

    let mut i: usize = 0;
    for original_shard in &mut original {
        original_shard[0] = data[i][0][0];
        original_shard[32] = data[i][0][1];
        i += 1;
    }

    // ENCODE
    let recovery =
        reed_solomon_simd::encode(no_of_data_shards, no_of_recovery_shards, &original).unwrap();
    let recovered = (0..no_of_recovery_shards)
        .map(|i| vec![[recovery[i][0], recovery[i][32]]])
        .collect::<Vec<Vec<[u8; 2]>>>();

    [data, recovered].concat()
}
