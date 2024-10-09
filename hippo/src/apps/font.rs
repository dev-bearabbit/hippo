use eframe::egui::{self, FontDefinitions, FontData, FontFamily};

pub fn configure_hangul_fonts(ctx: &egui::Context) {

    let mut fonts = FontDefinitions::default();

    // Regular 폰트 파일 설정
    fonts.font_data.insert(
        "nanum_regular".to_owned(),
        FontData::from_owned(include_bytes!("../../assets/fonts/NanumGothic.ttf").to_vec()),
    );

    // Bold 폰트 파일 설정
    fonts.font_data.insert(
        "nanum_bold".to_owned(),
        FontData::from_owned(include_bytes!("../../assets/fonts/NanumGothicBold.ttf").to_vec()),
    );

    // Proportional 폰트 설정: 기본 Regular 폰트를 우선으로 사용
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "nanum_regular".to_owned());

    // Strong (Bold) 폰트는 특별히 설정
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(1, "nanum_bold".to_owned());

    // Monospace 폰트 설정 (필요 시)
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "nanum_regular".to_owned());

    // 컨텍스트에 새로운 폰트 설정 적용
    ctx.set_fonts(fonts);
}