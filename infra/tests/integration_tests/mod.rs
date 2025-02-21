use crate::integration_tests::settings::Settings;
mod settings;

#[test]
pub fn should_xxx_test() {

    let settings =  Settings::unsafe_get_lazy();

    println!("{:?}", settings);

    assert!(true)
}