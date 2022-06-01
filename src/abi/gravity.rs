    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewGravatar {
            pub id: ethabi::Uint,
            pub owner: Vec<u8>,
            pub display_name: String,
            pub image_url: String,
        }
        impl NewGravatar {
            const TOPIC_ID: [u8; 32] = [
                154u8,
                179u8,
                174u8,
                251u8,
                43u8,
                166u8,
                220u8,
                18u8,
                145u8,
                10u8,
                193u8,
                188u8,
                228u8,
                105u8,
                44u8,
                245u8,
                195u8,
                192u8,
                208u8,
                108u8,
                255u8,
                22u8,
                50u8,
                124u8,
                100u8,
                163u8,
                239u8,
                120u8,
                34u8,
                139u8,
                19u8,
                11u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 192usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<NewGravatar, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                Ok(Self {
                    image_url: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    display_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn must_decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> NewGravatar {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => panic!("Unable to decode logs.NewGravatar event: {:#}", e),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdatedGravatar {
            pub id: ethabi::Uint,
            pub owner: Vec<u8>,
            pub display_name: String,
            pub image_url: String,
        }
        impl UpdatedGravatar {
            const TOPIC_ID: [u8; 32] = [
                118u8,
                87u8,
                27u8,
                122u8,
                137u8,
                122u8,
                21u8,
                9u8,
                198u8,
                65u8,
                88u8,
                117u8,
                104u8,
                33u8,
                138u8,
                41u8,
                0u8,
                24u8,
                251u8,
                220u8,
                139u8,
                154u8,
                114u8,
                79u8,
                23u8,
                183u8,
                127u8,
                240u8,
                238u8,
                194u8,
                44u8,
                12u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 192usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<UpdatedGravatar, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                Ok(Self {
                    image_url: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    display_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn must_decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> UpdatedGravatar {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!("Unable to decode logs.UpdatedGravatar event: {:#}", e)
                    }
                }
            }
        }
    }