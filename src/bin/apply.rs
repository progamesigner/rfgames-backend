use lazy_static::lazy_static;
use regex::Regex;
use rfgames_api_backend::{server, FromForm};
use serde::{self, Deserialize, Deserializer};
use serde_json::{json, Value};
use std::fmt::{Display, Formatter, Result as FMTResult};

#[derive(Deserialize)]
struct Boolean(bool);

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Profession {
    Warrior,
    Guardian,
    Revenant,
    Ranger,
    Thief,
    Engineer,
    Elementalist,
    Necromancer,
    Mesmer,
}

#[derive(Deserialize)]
struct ApplicationForm {
    #[serde(deserialize_with = "arenanet_account_deserializer")]
    account: String,
    #[serde(deserialize_with = "discord_account_deserializer")]
    discord: String,
    age: Boolean,
    goals: Boolean,
    times: Boolean,
    microphone: Boolean,
    commands: Boolean,
    main: Profession,
    alt: Profession,
    message: String,
}

impl Display for Boolean {
    fn fmt(&self, formatter: &mut Formatter) -> FMTResult {
        Display::fmt(
            match self.0 {
                true => "Yes",
                false => "No",
            },
            formatter,
        )
    }
}

impl Display for Profession {
    fn fmt(&self, formatter: &mut Formatter) -> FMTResult {
        Display::fmt(
            match self {
                Profession::Warrior => "Warrior / Berserker / Spellbreaker",
                Profession::Guardian => "Guardian / Dragonhunter / Firebrand",
                Profession::Revenant => "Revenant / Herald / Renegade",
                Profession::Ranger => "Ranger / Druid / Soulbeast",
                Profession::Thief => "Thief / Daredevil / Deadeye",
                Profession::Engineer => "Engineer / Scrapper / Holosmith",
                Profession::Elementalist => "Elementalist / Tempest / Weaver",
                Profession::Necromancer => "Necromancer / Reaper / Scourge",
                Profession::Mesmer => "Mesmer / Chronomancer / Mirage",
            },
            formatter,
        )
    }
}

impl FromForm for ApplicationForm {
    fn into_body(data: &Self, id: &str) -> Value {
        json!({
            "embeds": [{
                "title": format!(
                    "Application {} from \"{}\" ({})",
                    id,
                    data.account,
                    data.discord
                ),
                "description": format!("
Is age 18 or older: {}
Does understand visions and goals: {}
Does run with us in weekend: {}
Does have a working microphone: {}
Does have a willing to command: {}
Professions: {} & {}
**Messages**
```{}```
",
                    data.age,
                    data.goals,
                    data.times,
                    data.microphone,
                    data.commands,
                    data.main,
                    data.alt,
                    data.message
                )
            }]
        })
    }

    fn prefix() -> String {
        "AP".into()
    }
}

fn arenanet_account_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    lazy_static! {
        static ref ARENANET_REGEX: Regex = Regex::new(r"^[\w\s]{3,27}\.\d{4}$").unwrap();
    }

    let account = String::deserialize(deserializer)?;

    if ARENANET_REGEX.is_match(&account) {
        Ok(account)
    } else {
        Err(serde::de::Error::custom("wrong Arenanet account format"))
    }
}

fn discord_account_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    lazy_static! {
        static ref DISCORD_REGEX: Regex = Regex::new(r"^.*#[0-9]{4}$").unwrap();
    }

    let account = String::deserialize(deserializer)?;

    if DISCORD_REGEX.is_match(&account) {
        Ok(account)
    } else {
        Err(serde::de::Error::custom("wrong Discord account format"))
    }
}

fn main() {
    env_logger::init();

    server::start::<ApplicationForm>();
}
