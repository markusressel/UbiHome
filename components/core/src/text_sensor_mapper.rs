#[macro_export]
macro_rules! template_text_sensor {
    ($component_name:ident, $text_sensor_extension:ident) => {
        use $crate::sensor::SensorBase;
        use $crate::sensor::UnknownSensor;

        #[allow(non_camel_case_types)]
        #[derive(Clone, Deserialize, Debug)]
        #[serde(tag = "platform")]
        #[serde(rename_all = "camelCase")]
        pub enum TextSensorKind {
            $component_name($text_sensor_extension),
            #[serde(untagged)]
            Unknown(UnknownSensor),
        }

        #[derive(Clone, Deserialize, Debug)]
        pub struct TextSensor {
            #[serde(flatten)]
            pub default: SensorBase,

            #[serde(flatten)]
            pub extra: TextSensorKind,
        }
    };
}
