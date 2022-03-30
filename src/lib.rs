pub mod auto4;
pub mod traits;
pub mod models;
pub mod track;

#[cfg(test)]
mod tests {
    use crate::auto4::Auto4;

    #[test]
    fn it_works() {
        let auto4 = Auto4::new("[Script Info]
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
Dialogue: 0,0:00:00.00,0:00:04.99,staff omake,,0,0,0,,\\N{\\fad(300,300)}时间轴：某昨P").unwrap();
        assert_eq!("19", auto4.eval_ret_string(r#"#aegisub.subtitle"#).unwrap());
        assert_eq!("info", auto4.eval_ret_string(r#"aegisub.subtitle[0].class"#).unwrap());
        assert_eq!("style", auto4.eval_ret_string(r#"aegisub.subtitle[7].class"#).unwrap());
    }
}
