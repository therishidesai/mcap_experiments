use serde_derive::{Serialize, Deserialize};
use serde_cbor;
use crossbeam_channel::{unbounded, RecvError};
use mcap::{Channel, Schema, records::MessageHeader, Writer};
use std::{collections::BTreeMap, fs, io::BufWriter};

#[derive(Serialize, Deserialize, Debug)]
struct Pose {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let (sender, receiver) = unbounded::<Vec<u8>>();

    let handle = std::thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(pose) => {
                    let deserialized_pose: Pose = serde_cbor::from_slice(&pose).unwrap();

                    println!("{:?}", deserialized_pose);
                },
                Err(RecvError) => return,
            }
        }
    });


    let mut x = 1.0;
    let mut y = 2.0;

    let mut out = Writer::new(
        BufWriter::new(fs::File::create("out.mcap").unwrap())
    ).unwrap();

    let pose_channel = Channel {
        topic: String::from("pose"),
        schema: Some(Schema {
            name: String::from("Pose"),
            encoding: String::from(""),
            data: std::borrow::Cow::Borrowed(&[]),
        }.into()),
        message_encoding: String::from("cbor"),
        metadata: BTreeMap::default()
    };
    let channel_id = out.add_channel(&pose_channel).unwrap();

    for i in 0..25 {
        let pose = Pose { x, y, z: 3.0 };
        let serialized_pose = serde_cbor::to_vec(&pose).unwrap();
        sender.send(serialized_pose.clone());
        let pub_time = std::time::SystemTime::now();
        let pub_time_epoch = pub_time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        out.write_to_known_channel(
            &MessageHeader {
                channel_id,
                sequence: i,
                log_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
                publish_time: pub_time_epoch.as_millis() as u64,
            },
            &serialized_pose
        ).unwrap();


        x += 1.0;
        y += 1.0;
        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    drop(sender);

    handle.join().unwrap();
}
