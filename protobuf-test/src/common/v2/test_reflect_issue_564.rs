use super::test_reflect_issue_564_pb::*;

use protobuf::descriptor::field_descriptor_proto;
use protobuf::reflect::ReflectValueRef;
use protobuf::MessageDyn;

fn visit_fields(mes: &dyn MessageDyn) {
    let mut seen_enum = false;
    let mut seen_message = false;

    let descriptor = mes.descriptor_dyn();
    for field in descriptor.fields() {
        if field.has_field(mes) {
            match field.get_proto().get_field_type() {
                field_descriptor_proto::Type::TYPE_ENUM => {
                    match field.get_singular(mes) {
                        Some(ReflectValueRef::Enum(e, value)) => {
                            assert_eq!(
                                "TEST_ENUM_VALUE_B",
                                e.get_value_by_number(value).unwrap().get_name()
                            );
                            assert!(!seen_enum);
                            seen_enum = true;
                        }
                        Some(..) => panic!("not an enum"),
                        None => panic!("must be set"),
                    };
                }
                field_descriptor_proto::Type::TYPE_MESSAGE => {
                    let _ = field.get_message(mes);
                    assert!(!seen_message);
                    seen_message = true;
                }
                _ => {}
            }
        }
    }
    assert!(seen_enum);
    assert!(seen_message);
}

#[test]
fn test() {
    let mut mes = Issue564::default();
    mes.set_ee(issue564::TestEnum::TEST_ENUM_VALUE_B);
    mes.set_mm(Default::default());
    visit_fields(&mes);
}
