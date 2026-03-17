use crate::models::*;
use anyhow::Result;

/// 响应处理器
pub struct Handler;

impl Handler {
    /// 解析 JSON 响应为强类型数据
    pub fn parse_response(json_str: &str) -> Result<ApiResponseWrapper> {
        let response: ApiResponseWrapper = serde_json::from_str(json_str)?;
        Ok(response)
    }

    /// 根据动作类型获取相应的数据
    pub fn extract_data<'a>(wrapper: &'a ApiResponseWrapper, action: &str) -> OutputData<'a> {
        match action {
            "class" => {
                if let Some(classes) = wrapper.class.as_ref() {
                    OutputData::Classes(classes)
                } else if let Some(class) = wrapper.try_class_data() {
                    OutputData::Class(Box::new(class))
                } else if let Some(videos) = wrapper.list.as_ref() {
                    OutputData::Videos(videos)
                } else {
                    OutputData::Classes(&[])
                }
            }
            _ => {
                if let Some(videos) = wrapper.list.as_ref() {
                    OutputData::Videos(videos)
                } else if let Some(video) = wrapper.try_video_data() {
                    OutputData::Video(Box::new(video))
                } else if let Some(classes) = wrapper.class.as_ref() {
                    OutputData::Classes(classes)
                } else {
                    OutputData::Videos(&[])
                }
            }
        }
    }

    /// 格式化输出
    pub fn format_output(data: OutputData, format: OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Tabular => Self::format_tabular(data),
            OutputFormat::Json => Self::format_json(data),
        }
    }

    fn format_video_tabular(video: &Video) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            video.vod_id,
            video.vod_name,
            video.type_name,
            video.vod_time,
            video.vod_remarks,
            video.vod_blurb.as_deref().unwrap_or(""),
            video.vod_play_from,
            video.vod_pic.as_deref().unwrap_or(""),
            video.vod_play_url.as_deref().unwrap_or(""),
            video.vod_actor.as_deref().unwrap_or(""),
            video.vod_director.as_deref().unwrap_or(""),
            video.vod_area.as_deref().unwrap_or(""),
            video.vod_lang.as_deref().unwrap_or(""),
            video.vod_year.as_deref().unwrap_or("")
        )
    }

    fn format_class_tabular(class: &Class) -> String {
        format!("{}\t{}\t{}", class.type_id, class.type_name, class.type_pid)
    }

    /// 制表符格式输出
    fn format_tabular(data: OutputData) -> Result<String> {
        let estimated_size = match &data {
            OutputData::Videos(v) => v.len() * 256,
            OutputData::Classes(c) => c.len() * 64,
            OutputData::Video(_) => 256,
            OutputData::Class(_) => 64,
        };
        let mut output = String::with_capacity(estimated_size);

        match data {
            OutputData::Videos(videos) => {
                for video in videos {
                    output.push_str(&Self::format_video_tabular(video));
                    output.push('\n');
                }
            }
            OutputData::Classes(classes) => {
                for class in classes {
                    output.push_str(&Self::format_class_tabular(class));
                    output.push('\n');
                }
            }
            OutputData::Video(video) => {
                output.push_str(&Self::format_video_tabular(&video));
                output.push('\n');
            }
            OutputData::Class(class) => {
                output.push_str(&Self::format_class_tabular(&class));
                output.push('\n');
            }
        }

        Ok(output)
    }

    /// JSON 格式输出
    fn format_json(data: OutputData) -> Result<String> {
        let json_data = match data {
            OutputData::Videos(videos) => serde_json::to_value(videos)?,
            OutputData::Classes(classes) => serde_json::to_value(classes)?,
            OutputData::Video(video) => serde_json::to_value(*video)?,
            OutputData::Class(class) => serde_json::to_value(*class)?,
        };

        Ok(serde_json::to_string_pretty(&json_data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_video() -> Video {
        Video {
            vod_id: "1".to_string(),
            vod_name: "Test".to_string(),
            type_id: "1".to_string(),
            type_name: "Movie".to_string(),
            vod_en: None,
            vod_time: "2024-01-01".to_string(),
            vod_remarks: "HD".to_string(),
            vod_blurb: None,
            vod_play_from: "src".to_string(),
            vod_pic: None,
            vod_play_url: None,
            vod_actor: None,
            vod_director: None,
            vod_area: None,
            vod_lang: None,
            vod_year: None,
        }
    }

    #[test]
    fn test_parse_response() {
        let json = r#"{"list": []}"#;
        let wrapper = Handler::parse_response(json).unwrap();
        assert!(wrapper.list.is_some());
    }

    #[test]
    fn test_extract_data_videos() {
        let json = r#"{"list": [{"vod_id": "1", "vod_name": "Test", "type_id": "1", "type_name": "Movie", "vod_time": "", "vod_remarks": "", "vod_play_from": ""}]}"#;
        let wrapper = Handler::parse_response(json).unwrap();
        let data = Handler::extract_data(&wrapper, "videolist");
        match data {
            OutputData::Videos(v) => assert_eq!(v.len(), 1),
            _ => panic!("Expected Videos"),
        }
    }

    #[test]
    fn test_format_tabular_video() {
        let video = make_video();
        let data = OutputData::Video(Box::new(video));
        let output = Handler::format_output(data, OutputFormat::Tabular).unwrap();
        assert!(output.starts_with("1\tTest\tMovie"));
    }

    #[test]
    fn test_format_json_video() {
        let video = make_video();
        let data = OutputData::Video(Box::new(video));
        let output = Handler::format_output(data, OutputFormat::Json).unwrap();
        assert!(output.contains("\"vod_id\": \"1\""));
        assert!(output.contains("\"vod_name\": \"Test\""));
    }
}
