use serde::{self, Deserialize, Serialize};

/// Snapshot data of instrument from server
/// extracted it from the WebSocket via browser debugging.
///
/// # Source example:
///
/// "a[\"{\\\"message\\\":\\\"pid-945629::{\\\\\\\"pid\\\\\\\":\\\\\\\"945629\\\\\\\",\\\\\\\"last_dir\\\\\\\":\\\\\\\"redBg\\\\\\\",\\\\\\\"last_numeric\\\\\\\":18951.2,\\\\\\\"last\\\\\\\":\\\\\\\"18,951.2\\\\\\\",\\\\\\\"bid\\\\\\\":\\\\\\\"18,954.0\\\\\\\",\\\\\\\"ask\\\\\\\":\\\\\\\"18,956.0\\\\\\\",\\\\\\\"high\\\\\\\":\\\\\\\"19,956.0\\\\\\\",\\\\\\\"low\\\\\\\":\\\\\\\"18,279.0\\\\\\\",\\\\\\\"last_close\\\\\\\":\\\\\\\"19,188.0\\\\\\\",\\\\\\\"pc\\\\\\\":\\\\\\\"-236.8\\\\\\\",\\\\\\\"pcp\\\\\\\":\\\\\\\"-1.23%\\\\\\\",\\\\\\\"pc_col\\\\\\\":\\\\\\\"redFont\\\\\\\",\\\\\\\"turnover\\\\\\\":\\\\\\\"21.50K\\\\\\\",\\\\\\\"turnover_numeric\\\\\\\":21503,\\\\\\\"time\\\\\\\":\\\\\\\"19:21:50\\\\\\\",\\\\\\\"timestamp\\\\\\\":1606850510}\\\"}\"]"
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Snapshot {
    pub pid: String,

    //#[serde(skip_deserializing)]
    pub last_dir: Option<Box<str>>,
    pub last_numeric: f32,
    pub last: Box<str>,
    pub bid: String,
    pub ask: String,
    pub high: String,
    pub low: String,

    #[serde(default)]
    pub last_close: String,

    //#[serde(skip_deserializing)]
    pub pc: String,

    //#[serde(skip_deserializing)]
    pub pcp: String,

    //#[serde(skip_deserializing)]
    pub pc_col: String,

    //#[serde(skip_deserializing)]
    #[serde(default)]
    pub turnover: String,

    //#[serde(skip_deserializing)]
    #[serde(default)]
    pub turnover_numeric: String,

    //#[serde(skip_deserializing)]
    pub time: String,
    pub timestamp: u64,
}

impl Snapshot {
    /// Given original data from forexpros wss server, returns the Snapshot with extracted data.
    pub fn from_str<'a>(src: &'a str) -> Self {
        let idx_start = src.find("::{").expect("Expect the opening brace");
        let idx_end = src.find("}").expect("Expect the closing brace");

        let src = &src[idx_start + 2..idx_end + 1].replace("\\\\\\", "");

        serde_json::from_str(src).unwrap()
    }
}
