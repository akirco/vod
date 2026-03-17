use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: i32,
    pub msg: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API error (code={}): {}", self.code, self.msg)
    }
}

impl std::error::Error for ApiError {}

/// 将数字或字符串反序列化为字符串
fn deserialize_string_from_number_or_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> Visitor<'de> for StringOrNumberVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

/// 视频信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    #[serde(deserialize_with = "deserialize_string_from_number_or_string")]
    pub vod_id: String,
    pub vod_name: String,
    #[serde(deserialize_with = "deserialize_string_from_number_or_string")]
    pub type_id: String,
    pub type_name: String,
    pub vod_en: Option<String>,
    pub vod_time: String,
    pub vod_remarks: String,
    pub vod_blurb: Option<String>,
    pub vod_play_from: String,
    pub vod_pic: Option<String>,
    pub vod_play_url: Option<String>,
    pub vod_actor: Option<String>,
    pub vod_director: Option<String>,
    pub vod_area: Option<String>,
    pub vod_lang: Option<String>,
    pub vod_year: Option<String>,
}

/// 分类信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    #[serde(deserialize_with = "deserialize_string_from_number_or_string")]
    pub type_id: String,
    pub type_name: String,
    #[serde(deserialize_with = "deserialize_string_from_number_or_string")]
    pub type_pid: String,
}

/// 通用数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

/// 响应包装器，兼容原有的 JSON 结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseWrapper {
    pub list: Option<Vec<Video>>,
    pub class: Option<Vec<Class>>,
    pub data: Option<serde_json::Value>,
    pub code: Option<i32>,
    pub msg: Option<String>,
}

impl ApiResponseWrapper {
    /// 检查 API 响应是否包含错误
    pub fn check_error(&self) -> Result<(), ApiError> {
        if let Some(code) = self.code
            && code != 1 && code != 0
        {
            return Err(ApiError {
                code,
                msg: self
                    .msg
                    .clone()
                    .unwrap_or_else(|| "Unknown error".to_string()),
            });
        }
        Ok(())
    }

    /// 检查响应是否为空（无数据）
    pub fn is_empty(&self) -> bool {
        self.list.as_ref().is_none_or(|v| v.is_empty())
            && self.class.as_ref().is_none_or(|c| c.is_empty())
            && self.data.is_none()
    }

    /// 尝试解析为单个视频
    pub fn try_video_data(&self) -> Option<Video> {
        self.data.as_ref().and_then(|data| {
            let data_clone = data.clone();
            // Try direct deserialization first (most common case)
            serde_json::from_value::<Video>(data_clone)
                .ok()
                .or_else(|| {
                    // Try wrapped in DataResponse if direct fails
                    serde_json::from_value::<DataResponse<Video>>(data.clone())
                        .map(|r| r.data)
                        .ok()
                })
        })
    }

    /// 尝试解析为单个分类
    pub fn try_class_data(&self) -> Option<Class> {
        self.data.as_ref().and_then(|data| {
            let data_clone = data.clone();
            // Try direct deserialization first (most common case)
            serde_json::from_value::<Class>(data_clone)
                .ok()
                .or_else(|| {
                    // Try wrapped in DataResponse if direct fails
                    serde_json::from_value::<DataResponse<Class>>(data.clone())
                        .map(|r| r.data)
                        .ok()
                })
        })
    }
}

/// 输出格式枚举
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Tabular,
    Json,
}

/// 输出数据
#[derive(Debug)]
pub enum OutputData<'a> {
    Videos(&'a [Video]),
    Classes(&'a [Class]),
    Video(Box<Video>),
    Class(Box<Class>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_video_with_numeric_id() {
        let json = r#"{
            "vod_id": 12345,
            "vod_name": "Test Video",
            "type_id": 1,
            "type_name": "Movie",
            "vod_time": "2024-01-01",
            "vod_remarks": "HD",
            "vod_play_from": "source1"
        }"#;
        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.vod_id, "12345");
        assert_eq!(video.type_id, "1");
    }

    #[test]
    fn test_api_response_wrapper_video_list() {
        let json = r#"{
            "list": [
                {"vod_id": "1", "vod_name": "Video 1", "type_id": "1", "type_name": "Movie", "vod_time": "2024-01-01", "vod_remarks": "HD", "vod_play_from": "src"}
            ]
        }"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        assert!(wrapper.list.is_some());
        assert_eq!(wrapper.list.unwrap().len(), 1);
    }

    #[test]
    fn test_api_response_error() {
        let json = r#"{"code": 500, "msg": "Internal error"}"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        let result = wrapper.check_error();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, 500);
        assert_eq!(err.msg, "Internal error");
    }

    #[test]
    fn test_api_response_success_code() {
        let json = r#"{"code": 1, "list": []}"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        assert!(wrapper.check_error().is_ok());
    }

    #[test]
    fn test_is_empty() {
        let json = r#"{}"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        assert!(wrapper.is_empty());

        let json = r#"{"list": []}"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        assert!(wrapper.is_empty());

        let json = r#"{"list": [{"vod_id": "1", "vod_name": "V", "type_id": "1", "type_name": "M", "vod_time": "", "vod_remarks": "", "vod_play_from": ""}]}"#;
        let wrapper: ApiResponseWrapper = serde_json::from_str(json).unwrap();
        assert!(!wrapper.is_empty());
    }
}
