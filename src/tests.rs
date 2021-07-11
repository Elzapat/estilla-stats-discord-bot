use crate::utils::*;

#[tokio::test]
async fn uuid_from_username() {
    assert_eq!(get_uuid_from_username("Elzapat".to_string()).await.unwrap(), "bb1784e458ee40749ae248684656aa59");
}

#[test]
fn uuid_untrimming() {
    assert_eq!(untrim_uuid("bb1784e458ee40749ae248684656aa59".to_string()), "bb1784e4-58ee-4074-9ae2-48684656aa59")
}

#[tokio::test]
async fn user_from_uuid() {
    assert_eq!(get_username_from_uuid("bb1784e4-58ee-4074-9ae2-48684656aa59".to_string()).await.unwrap(), "Elzapat")
}

#[test]
fn getting_longest_len_in_string_vec() {
    assert_eq!(5, longest_length_in_string_vec(&vec!["12".to_string(), "123".to_string(), "01234".to_string(), "123".to_string()]));
}
