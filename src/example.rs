#[macro_use]
extern crate gettersetter;

#[derive(GetSet)]
pub struct KafkaConfig {
    #[getset(skip_setter)]
    group_id: String,
    topic: String,
    #[getset(getter_name = "topic_get")]
    brokers: String,
    partitions: u16,
    auto_offset_reset: Option<String>,
}

fn main() {
    let config = KafkaConfig {
        group_id: "group_id".to_string(),
        topic: "topic".to_string(),
        brokers: "brokers".to_string(),
        partitions: 10,
        auto_offset_reset: Some("earliest".to_string()),
    };

    println!("{}", config.get_group_id());
    //    config.set_group_id("group_id_1".to_string()); // This call will give method not found as we have skipped.
    println!("{}", config.topic_get()); //Change default method name for get.
}
