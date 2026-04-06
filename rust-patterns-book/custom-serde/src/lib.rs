#[warn(rust_2018_idioms)]
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess}; 
use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct HumanDuration {
    pub hours: usize,
    pub minutes: usize,
    pub seconds: usize,
}

impl HumanDuration {
    fn new(hours: usize, minutes: usize, seconds: usize) -> Self {
        Self {
            hours, 
            minutes, 
            seconds
        }
    }
}

impl Serialize for HumanDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where S: Serializer
    {
        let mut state = serializer.serialize_struct("HumanDuration", 3)?;  
        // len denotes the number of fields to serialize
        state.serialize_field("h", &self.hours)?;
        state.serialize_field("m", &self.minutes)?;
        state.serialize_field("s", &self.seconds)?;
        
        state.end()
    }
}

impl<'de> Deserialize<'de> for HumanDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where
        D: Deserializer<'de> 
    {
        enum Field { Hours, Minutes, Seconds }
        const FIELDS: &[&str] = &["h", "m", "s"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error> 
            where
                D: Deserializer<'de> 
            {
                struct FieldVisitor;
                
                impl<'de> Visitor<'de> for FieldVisitor { 
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("'h(hours)' or 'm(minutes)' or 's(seconds)'") 
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "h" => Ok(Field::Hours),
                            "m" => Ok(Field::Minutes),
                            "s" => Ok(Field::Seconds),
                            _   => Err(de::Error::unknown_field(value, FIELDS)),          
                        }
                    }
                }
                
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        struct HumanDurationVisitor;

        impl<'de> Visitor<'de> for HumanDurationVisitor {
            type Value = HumanDuration;
        
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct HumanDuration") 
            }
            
            // add a method to construct HumanDuration object from a map like representation(example json serialization)  
            fn visit_map<V>(self, mut map: V) -> Result<HumanDuration, V::Error> 
            where 
                V: MapAccess<'de> 
            {
                let mut hours = None;
                let mut minutes = None;
                let mut seconds = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Hours => {
                            if hours.is_some() {
                                return Err(de::Error::duplicate_field("hours"));
                            } 

                            hours = Some(map.next_value()?);
                        },
                        Field::Minutes => { 
                            if minutes.is_some() {
                                return Err(de::Error::duplicate_field("minutes"));
                            } 

                            minutes = Some(map.next_value()?);
                        },
                        Field::Seconds => {
                            if seconds.is_some() {
                                return Err(de::Error::duplicate_field("seconds"));
                            } 

                            seconds = Some(map.next_value()?);
                        },
                    }
                }
                let hours = hours.ok_or_else(|| de::Error::missing_field("hours"))?;
                let minutes = minutes.ok_or_else(|| de::Error::missing_field("minutes"))?;
                let seconds = seconds.ok_or_else(|| de::Error::missing_field("seconds"))?;
                
                // construct a new HumanDuration object out of the local variables 
                Ok(HumanDuration::new(hours, minutes, seconds)) 
            }
        }

        deserializer.deserialize_struct("HumanDuration", FIELDS, HumanDurationVisitor)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let over_the_wire_data = "{\"h\":1,\"m\":25,\"s\":32}";
        
        let data: HumanDuration = serde_json::from_str(over_the_wire_data).ok().unwrap();
        
        assert_eq!(data, HumanDuration::new(1, 25, 32));
    }

    #[test]
    fn test_serialization() {
        let demo = HumanDuration {
            hours: 1,
            minutes: 25,
            seconds: 32,
        };

        let serialized = serde_json::to_string(&demo).ok().unwrap();
        println!("{:?}", &serialized);
    }
}
