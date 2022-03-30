use std::str::FromStr;
use crate::models::{AssDialogue, ASSInfo, Color, Style};

fn skip_spaces(input: &str) -> &str {
    input.trim_start_matches(&[' ', '\t'])
}

#[derive(Debug)]
enum ParserState {
    /// Unknown
    Unknown,
    /// Script Info
    Info,
    /// V4 Styles
    /// V4+ Styles
    Style,
    /// Events
    Events,
    /// Fonts
    Fonts,
}

#[derive(Debug)]
struct AssTrack {
    parser_state: ParserState,
    style_format: Vec<String>,
    event_format: Vec<String>,

    info: Vec<ASSInfo>,
    styles: Vec<Style>,
    events: Vec<AssDialogue>,
}

impl AssTrack {
    fn new() -> Self {
        Self {
            parser_state: ParserState::Unknown,
            // https://github.com/libass/libass/blob/5f0e8450f834894b2745238e3d32ff4878710ec8/libass/ass.c#L44-L48
            style_format: vec![
                "Name", "Fontname", "Fontsize", "PrimaryColour", "SecondaryColour",
                "OutlineColour", "BackColour", "Bold", "Italic", "Underline", "StrikeOut",
                "ScaleX", "ScaleY", "Spacing", "Angle", "BorderStyle", "Outline", "Shadow",
                "Alignment", "MarginL", "MarginR", "MarginV", "Encoding",
            ].into_iter().map(|s| s.to_string()).collect(),
            event_format: vec![
                "Layer", "Start", "End", "Style", "Name",
                "MarginL", "MarginR", "MarginV", "Effect", "Text",
            ].into_iter().map(|s| s.to_string()).collect(),
            info: Vec::new(),
            styles: Vec::new(),
            events: Vec::new(),
        }
    }

    fn process_line<'input>(&mut self, input: &'input str) -> &'input str {
        let input = skip_spaces(input);
        if input.starts_with("[Script Info]") {
            self.parser_state = ParserState::Info;
            &input[13..]
        } else if input.starts_with("[V4 Styles]") {
            // track_type = SSA
            self.parser_state = ParserState::Style;
            &input[11..]
        } else if input.starts_with("[V4+ Styles]") {
            // track_type = ASS
            self.parser_state = ParserState::Style;
            &input[12..]
        } else if input.starts_with("[Events]") {
            self.parser_state = ParserState::Events;
            &input[8..]
        } else if input.starts_with("[Fonts]") {
            self.parser_state = ParserState::Fonts;
            &input[7..]
        } else if input.starts_with("[") {
            self.parser_state = ParserState::Unknown;
            self.skip_line(input)
        } else {
            match self.parser_state {
                ParserState::Info => self.process_info_line(input),
                ParserState::Style => self.process_style_line(input),
                ParserState::Events => self.process_events_line(input),
                ParserState::Fonts => self.process_fonts_line(input),
                ParserState::Unknown => self.skip_line(input),
            }
        }
    }

    fn process_info_line<'input>(&mut self, input: &'input str) -> &'input str {
        let line_end = input.find('\n').unwrap_or(input.len());
        let line = &input[..line_end];
        let (key, value) = line.split_once(':').unwrap_or((input, ""));
        self.info.push(ASSInfo { key: key.trim().to_string(), value: value.trim().to_string() });
        &input[line_end..]
    }

    fn process_style_line<'input>(&mut self, input: &'input str) -> &'input str {
        let line_end = input.find('\n').unwrap_or(input.len());

        let line = &input[..line_end];
        if line.starts_with("Format:") {
            self.style_format = line[7..].trim().split(',').map(|s| s.trim().to_string()).collect();
        } else if line.starts_with("Style:") {
            let mut line = line[6..].trim();
            let mut style = Style {
                raw: line.to_string(),
                name: "Default".to_string(),
                font: "Arial".to_string(),
                font_size: 18.0,
                primary_color: Color::DEFAULT_PRIMARY,
                secondary_color: Color::DEFAULT_SECONDARY,
                outline_color: Color::DEFAULT,
                shadow_color: Color::DEFAULT,
                bold: true,
                italic: false,
                underline: false,
                strikeout: false,
                scale_x: 1.0,
                scale_y: 1.0,
                spacing: 0.0,
                angle: 0.0,
                border_style: 1,
                outline: 2.0,
                shadow: 3.0,
                alignment: 2,
                margin_left: 20,
                margin_right: 20,
                margin_vertical: 20,
                encoding: 0,
            };
            for ty in self.style_format.iter() {
                let (content, line_remaining) = line.split_once(',').unwrap_or((line, ""));
                match ty.as_str() {
                    "Name" => style.name = content.to_string(),
                    "Fontname" => style.font = content.to_string(),
                    "PrimaryColour" => style.primary_color = content.parse().unwrap_or(Color::DEFAULT_PRIMARY),
                    "SecondaryColour" => style.secondary_color = content.parse().unwrap_or(Color::DEFAULT_SECONDARY),
                    "OutlineColour" => style.outline_color = content.parse().unwrap_or(Color::DEFAULT),
                    "BackColour" => style.shadow_color = content.parse().unwrap_or(Color::DEFAULT),
                    "Fontsize" => style.font_size = content.parse().unwrap_or(style.font_size),
                    "Bold" => style.bold = content.parse().unwrap_or(1) > 0,
                    "Italic" => style.italic = content.parse().unwrap_or(0) > 0,
                    "Underline" => style.underline = content.parse().unwrap_or(0) > 0,
                    "StrikeOut" => style.strikeout = content.parse().unwrap_or(0) > 0,
                    "Spacing" => style.spacing = content.parse().unwrap_or(style.spacing),
                    "Angle" => style.angle = content.parse().unwrap_or(style.angle),
                    "BorderStyle" => style.border_style = content.parse().unwrap_or(style.border_style),
                    "Alignment" => style.border_style = content.parse().unwrap_or(style.alignment),
                    "MarginL" => style.margin_left = content.parse().unwrap_or(style.margin_left),
                    "MarginR" => style.margin_right = content.parse().unwrap_or(style.margin_right),
                    "MarginV" => style.margin_vertical = content.parse().unwrap_or(style.margin_vertical),
                    "Encoding" => style.encoding = content.parse().unwrap_or(style.encoding),
                    "ScaleX" => style.scale_x = content.parse().unwrap_or(style.scale_x),
                    "ScaleY" => style.scale_y = content.parse().unwrap_or(style.scale_y),
                    "Outline" => style.outline = content.parse().unwrap_or(style.outline),
                    "Shadow" => style.shadow = content.parse().unwrap_or(style.shadow),
                    _ => {
                        panic!("ty = {}, content = {}", ty, content);
                    }
                }

                line = line_remaining;
            }
            self.styles.push(style);
        }

        &input[line_end..]
    }

    fn process_events_line<'input>(&mut self, input: &'input str) -> &'input str {
        let line_end = input.find('\n').unwrap_or(input.len());
        let line = &input[..line_end];

        if let Some(line) = line.strip_prefix("Format:") {
            self.event_format = line.trim().split(',').map(|s| s.trim().to_string()).collect();
        } else if let Some((is_comment, mut line)) = line.strip_prefix("Dialogue:").map_or_else(
            || line.strip_prefix("Comment:").map(|s| (true, s)),
            |s| Some((false, s)),
        ) {
            let mut event = AssDialogue {
                comment: is_comment,
                layer: 0,
                margin_left: 0,
                margin_right: 0,
                margin_vertical: 0,
                start_time: Default::default(),
                end_time: Default::default(),
                style: "Default".to_string(),
                actor: "".to_string(),
                effect: "".to_string(),
                text: "".to_string(),
            };
            for ty in self.event_format.iter() {
                if ty.as_str() != "Text" {
                    // other properties
                    let (content, line_remaining) = line.split_once(',').unwrap_or((line, ""));
                    match ty.as_str() {
                        "Layer" => event.layer = content.parse().unwrap_or(0),
                        "Start" => event.start_time = content.parse().unwrap_or(event.start_time),
                        "End" => event.end_time = content.parse().unwrap_or(event.end_time),
                        "Style" => event.style = content.to_string(),
                        "Name" => event.actor = content.to_string(),
                        "MarginL" => event.margin_left = content.parse().unwrap_or(event.margin_left),
                        "MarginR" => event.margin_right = content.parse().unwrap_or(event.margin_right),
                        "MarginV" => event.margin_vertical = content.parse().unwrap_or(event.margin_vertical),
                        "Effect" => event.effect = content.to_string(),
                        _ => {
                            panic!("ty = {}, content = {}", ty, content);
                        }
                    }
                    line = line_remaining;
                } else {
                    // Text
                    event.text = line.to_string();
                    break;
                }
            }
            self.events.push(event);
        }

        &input[line_end..]
    }

    fn process_fonts_line<'input>(&mut self, input: &'input str) -> &'input str {
        let line_end = input.find('\n').unwrap_or(input.len());
        let line = &input[..line_end];

        if let Some(line) = line.strip_prefix("fontname:") {
            let line = line.trim();
        }

        &input[line_end..]
    }

    fn skip_line<'input>(&mut self, input: &'input str) -> &'input str {
        let line_end = input.find('\n').unwrap_or(input.len());
        &input[line_end..]
    }
}

impl FromStr for AssTrack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut track = AssTrack::new();
        let mut input = s;

        loop {
            input = input.trim_start_matches(['\r', '\n', '\u{fffe}']);
            if input.is_empty() {
                break;
            }

            input = track.process_line(input);
        }

        Ok(track)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ass() {
        let track: AssTrack = "[Script Info]
Title: Default Aegisub file
ScriptType: v4.00+
WrapStyle: 0
ScaledBorderAndShadow: yes
YCbCr Matrix: TV.709
PlayResX: 1280
PlayResY: 720

[Aegisub Project Garbage]
Audio File: ../视频/434omake.mp4
Video File: ../视频/434omake.mp4
Video AR Mode: 4
Video AR Value: 1.777778
Active Line: 12
Video Position: 861

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,32,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,3.2,3.2,2,16,16,16,1
Style: pyon,方正准圆_GBK,72,&H00765BEA,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,3.2,2,16,16,16,1
Style: Mocho,方正准圆_GBK,72,&H00BA90ED,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,3.2,2,16,16,16,1
Style: kraz,方正准圆_GBK,72,&H00CF9564,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,3.2,2,16,16,16,1
Style: 标注,方正准圆_GBK,72,&H00009CFF,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,3.2,7,16,16,16,1
Style: E,方正准圆_GBK,72,&H005E4D59,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,1.6,2,16,16,16,1
Style: staff,方正准圆_GBK,72,&H005E4D59,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,6.4,1.6,7,16,16,16,1
Style: 多人,方正准圆_GBK,72,&H00B3BB49,&H000000FF,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0,0,1,4.8,3.2,2,16,16,16,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:00.00,0:00:04.99,staff omake,,0,0,0,,\\N{\\fad(300,300)}片源：RabbitC
Dialogue: 0,0:00:00.00,0:00:04.99,staff omake,,0,0,0,,\\N{\\fad(300,300)}翻译：八足  MochizukiShigure
Dialogue: 0,0:00:00.00,0:00:04.99,staff omake,,0,0,0,,\\N{\\fad(300,300)}校对：后母辣酱
Dialogue: 0,0:00:00.00,0:00:04.99,staff omake,,0,0,0,,\\N{\\fad(300,300)}时间轴：某昨P".parse().unwrap();
        println!("{:#?}", track);
    }
}