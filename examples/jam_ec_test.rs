use reed_solomon_erasure;
//use hex_literal;
use hex;

fn main() {
let r = reed_solomon_erasure::galois_16::ReedSolomon::new(342, 681).unwrap();
     let original = (0..684).map(|i| i as u8).collect::<Vec<_>>();
    //let original = hex_literal::hex!["1234567890"];

    let mut data: Vec<Vec<[u8; 2]>> = original.chunks_exact(2)
		.map(|x| x.chunks_exact(2).map(|s| [s[0], s[1]]).collect())
		.collect();
	data.reserve(1023);
	for _ in 0..681 {
		data.push(vec![[0, 0]; 1]);
	}

    // Construct the parity shards
    r.encode(&mut data).unwrap();

	for i in 0..1023 {
		println!("{i}: {}", hex::encode(&data[i].iter().flatten().cloned().collect::<Vec<_>>()[..]));
	}

}
