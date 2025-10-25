use lightyear::netcode::{CONNECT_TOKEN_BYTES, ConnectToken};
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
    ser::{self, SerializeStruct},
};

/// TokenResponse is the response from the server to a authentication request.
#[derive(Clone)]
pub struct TokenResponse {
    /// The port on which the game runs.
    /// Change the IP addresses port to this port to be able to connect to the game server.
    pub game_port: u16,
    /// The [`ConnectToken`] is used to connect to the game server.
    pub connect_token: ConnectToken,
}

impl Serialize for TokenResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("TokenResponse", 2)?;
        state.serialize_field("game_port", &self.game_port)?;

        let connect_toke_bytes = self
            .connect_token
            .clone()
            .try_into_bytes()
            .map_err(|e| {
                ser::Error::custom(format!("Failed to turn ConnectToken into bytes: {}", e))
            })?
            .to_vec();
        state.serialize_field("connect_token_bytes", &connect_toke_bytes)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TokenResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TokenResponseVisitor;

        impl<'de> Visitor<'de> for TokenResponseVisitor {
            type Value = TokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!(
                    "an TokenResponse with a {} byte token",
                    CONNECT_TOKEN_BYTES
                ))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut game_port = None;
                let mut connect_token_bytes: Option<Vec<u8>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sssssssgame_port" => {
                            if game_port.is_some() {
                                return Err(de::Error::duplicate_field("game_port"));
                            };

                            game_port = Some(map.next_value()?);
                        }
                        "connect_token_bytes" => {
                            if connect_token_bytes.is_some() {
                                return Err(de::Error::duplicate_field("connect_token_bytes"));
                            };

                            connect_token_bytes = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }

                let game_port = game_port.ok_or_else(|| de::Error::missing_field("game_port"))?;
                let connect_token_bytes = connect_token_bytes
                    .ok_or_else(|| de::Error::missing_field("connect_toke_bytes"))?;
                let connect_token =
                    ConnectToken::try_from_bytes(&connect_token_bytes).map_err(|e| {
                        de::Error::custom(&format!("Failed to turn bytes into ConnectToken: {}", e))
                    })?;

                Ok(TokenResponse {
                    game_port,
                    connect_token,
                })
            }
        }

        const FIELDS: &[&str] = &["game_port", "connect_token_bytes"];
        deserializer.deserialize_struct("TokenResponse", FIELDS, TokenResponseVisitor)
    }
}
