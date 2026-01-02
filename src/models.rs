use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

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

/// API 响应列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoListResponse {
    pub list: Vec<Video>,
}

/// API 分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassListResponse {
    pub class: Vec<Class>,
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
    /// 尝试解析为视频列表
    pub fn try_video_list(&self) -> Option<Vec<Video>> {
        if let Some(videos) = &self.list {
            return Some(videos.clone());
        }
        
        if let Some(data) = &self.data {
            if let Ok(video_list) = serde_json::from_value::<VideoListResponse>(data.clone()) {
                return Some(video_list.list);
            }
            if let Ok(data_resp) = serde_json::from_value::<DataResponse<Vec<Video>>>(data.clone()) {
                return Some(data_resp.data);
            }
        }
        
        None
    }
    
    /// 尝试解析为分类列表
    pub fn try_class_list(&self) -> Option<Vec<Class>> {
        if let Some(classes) = &self.class {
            return Some(classes.clone());
        }
        
        if let Some(data) = &self.data {
            if let Ok(class_list) = serde_json::from_value::<ClassListResponse>(data.clone()) {
                return Some(class_list.class);
            }
            if let Ok(data_resp) = serde_json::from_value::<DataResponse<Vec<Class>>>(data.clone()) {
                return Some(data_resp.data);
            }
        }
        
        None
    }
    
    /// 尝试解析为单个视频
    pub fn try_video_data(&self) -> Option<Video> {
        if let Some(data) = &self.data {
            if let Ok(video) = serde_json::from_value::<DataResponse<Video>>(data.clone()) {
                return Some(video.data);
            }
            if let Ok(video) = serde_json::from_value::<Video>(data.clone()) {
                return Some(video);
            }
        }
        
        None
    }
    
    /// 尝试解析为单个分类
    pub fn try_class_data(&self) -> Option<Class> {
        if let Some(data) = &self.data {
            if let Ok(class) = serde_json::from_value::<DataResponse<Class>>(data.clone()) {
                return Some(class.data);
            }
            if let Ok(class) = serde_json::from_value::<Class>(data.clone()) {
                return Some(class);
            }
        }
        
        None
    }
}

/// 输出格式枚举
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Tabular,
    Json,
}

/// 输出数据
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum OutputData {
    Videos(Vec<Video>),
    Classes(Vec<Class>),
    Video(Video),
    Class(Class),
}