#![allow(non_camel_case_types)]

pub mod request;
pub mod response;

pub use request::Request;
pub use response::Response;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum R_Result {
    OK,
    ERROR {
        error_message: String,
    }
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum R_PlayerSubCommand {
    Play {
        audio_label: String,
    },
    Pause,
    Resume,
    Clear,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum R_StatusSubCommand {
    ALL {
        output: String,
    },
	CurrentAudio {
        output: Option<String>,
    },
	IsPaused {
        output: String,
    },
	IsQueueEmpty {
        output: String,
    }
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum R_Command {
    Status {
        sub_command: R_StatusSubCommand,
    },
    Reload,
    Search {
        search_result: Vec<String>,
    },
    Player {
        sub_command: R_PlayerSubCommand,
    }
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct R_Packet {
    pub result: R_Result,
    pub command: R_Command,
}
