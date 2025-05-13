use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg(feature = "extension")]
use {
    super::search_results::SearchResult, postcard::from_bytes, postcard::to_allocvec,
    std::error::Error, std::io::Read, std::io::Write,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormResultsRequest {
    pub form_id: String,
    pub results: Vec<FormResult>,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormResult {
    pub id: String,
    pub value: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionRequest {
    pub request_type: RequestType,
    pub get_results_request: Option<GetResultsRequest>,
    pub run_action_request: Option<RunActionRequest>,
    pub form_results_request: Option<FormResultsRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestType {
    GetResults,
    RunAction,
    FormResults,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResultsRequest {
    pub search_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunActionRequest {
    pub action: String,
    pub args: Vec<String>,
}

impl ExtensionRequest {
    pub fn new_get_results_request(search_text: &str) -> Self {
        Self {
            request_type: RequestType::GetResults,
            get_results_request: Some(GetResultsRequest::new(search_text)),
            run_action_request: None,
            form_results_request: None,
        }
    }

    pub fn new_run_extension_action_request(action: &str, args: &Vec<String>) -> Self {
        Self {
            request_type: RequestType::RunAction,
            get_results_request: None,
            run_action_request: Some(RunActionRequest::new(action, args)),
            form_results_request: None,
        }
    }

    pub fn new_form_results_request(
        form_id: &str,
        results: &Vec<FormResult>,
        args: &Vec<String>,
    ) -> Self {
        Self {
            request_type: RequestType::FormResults,
            get_results_request: None,
            run_action_request: None,
            form_results_request: Some(FormResultsRequest {
                form_id: form_id.to_owned(),
                results: results.to_owned(),
                args: args.to_owned(),
            }),
        }
    }
}

impl GetResultsRequest {
    pub fn new(search_text: &str) -> Self {
        Self {
            search_text: search_text.to_owned(),
        }
    }
}

impl RunActionRequest {
    pub fn new(action: &str, args: &Vec<String>) -> Self {
        Self {
            action: action.to_owned(),
            args: args.to_owned(),
        }
    }
}

impl FormResultsRequest {
    pub fn get_result(&self, id: &str) -> Result<FormResult, String> {
        let result = self.results.iter().find(|r| r.id == id);

        if let Some(result) = result {
            Ok(result.to_owned())
        } else {
            Err(format!("Could not find result with id: {}", id))
        }
    }

    pub fn get_string_value(&self, id: &str) -> Result<String, String> {
        let result = self.get_result(id)?;
        Ok(result.value)
    }

    pub fn get_bool_value(&self, id: &str) -> Result<bool, String> {
        let result = self.get_result(id)?;

        if result.value == "true" {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_usize_value(&self, id: &str) -> Result<usize, String> {
        let result = self.get_result(id)?;
        if let Ok(number) = result.value.parse::<usize>() {
            Ok(number)
        } else {
            Err("Could not parse result to a number".to_string())
        }
    }

    pub fn get_path_value(&self, id: &str) -> Result<PathBuf, String> {
        let result = self.get_result(id)?;
        Ok(PathBuf::from(&result.value))
    }
}

// =================================================================
// ==== API
// =================================================================

#[cfg(feature = "extension")]
pub fn return_search_results(results: &Vec<SearchResult>) {
    use std::process::exit;

    let bytes = to_allocvec(results).expect("Error getting results as bytes");
    std::io::stdout()
        .write_all(&bytes)
        .expect("Error sendind results");

    exit(0)
}

#[cfg(feature = "extension")]
pub fn get_request() -> Result<ExtensionRequest, Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    std::io::stdin().read_to_end(&mut buffer)?;

    let request: ExtensionRequest = from_bytes(&buffer)?;

    Ok(request)
}
