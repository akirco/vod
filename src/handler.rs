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
    pub fn extract_data(wrapper: &ApiResponseWrapper, action: &str) -> OutputData {
        match action {
            "class" => {
                if let Some(classes) = wrapper.try_class_list() {
                    OutputData::Classes(classes.to_vec())
                } else if let Some(class) = wrapper.try_class_data() {
                    OutputData::Class(class)
                } else if let Some(videos) = wrapper.try_video_list() {
                    OutputData::Videos(videos.to_vec())
                } else {
                    OutputData::Classes(Vec::new())
                }
            }
            _ => {
                if let Some(videos) = wrapper.try_video_list() {
                    OutputData::Videos(videos.to_vec())
                } else if let Some(video) = wrapper.try_video_data() {
                    OutputData::Video(video)
                } else if let Some(classes) = wrapper.try_class_list() {
                    OutputData::Classes(classes.to_vec())
                } else {
                    OutputData::Videos(Vec::new())
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
        let mut output = String::new();

        match data {
            OutputData::Videos(videos) => {
                for video in &videos {
                    output.push_str(&Self::format_video_tabular(video));
                    output.push('\n');
                }
            }
            OutputData::Classes(classes) => {
                for class in &classes {
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
            OutputData::Video(video) => serde_json::to_value(video)?,
            OutputData::Class(class) => serde_json::to_value(class)?,
        };

        Ok(serde_json::to_string_pretty(&json_data)?)
    }
}
