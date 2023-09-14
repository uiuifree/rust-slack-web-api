#[derive(Debug)]
pub enum SlackError {
    ApiError(SlackApiError),
    HttpError(SlackHttpError),
    SystemError(SlackSystemError),
}

impl From<SlackApiError> for SlackError {
    fn from(value: SlackApiError) -> Self {
        SlackError::ApiError(value)
    }
}

impl From<SlackHttpError> for SlackError {
    fn from(value: SlackHttpError) -> Self {
        SlackError::HttpError(value)
    }
}

impl From<SlackSystemError> for SlackError {
    fn from(value: SlackSystemError) -> Self {
        SlackError::SystemError(value)
    }
}

#[derive(Debug)]
pub struct SlackApiError {
    pub status: u16,
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

#[derive(Debug)]
pub struct SlackHttpError {
    pub status: u16,
    pub http_response_body: Option<String>,
}

impl SlackHttpError {
    pub fn new(status: u16, http_response_body: String) -> SlackHttpError {
        SlackHttpError {
            status,
            http_response_body: Some(http_response_body),
        }
    }
}

#[derive(Debug)]
pub struct SlackSystemError {
    pub message: Option<String>,
}

impl SlackSystemError {
    pub fn new(message: String) -> SlackSystemError {
        SlackSystemError {
            message: Some(message),
        }
    }
}
