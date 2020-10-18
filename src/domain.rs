use imgui::{Style, StyleColor};
use implot::{
    push_style_color, push_style_var_f32, push_style_var_i32, push_style_var_imvec2, ImVec2,
    Marker, PlotColorElement, StyleVar,
};

pub fn style_seaborn() {
    const IMPLOT_AUTO_COL: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, -1.0);

    push_style_color(
        &PlotColorElement::Line,
        IMPLOT_AUTO_COL.0,
        IMPLOT_AUTO_COL.1,
        IMPLOT_AUTO_COL.2,
        IMPLOT_AUTO_COL.3,
    );
    push_style_color(
        &PlotColorElement::Fill,
        IMPLOT_AUTO_COL.0,
        IMPLOT_AUTO_COL.1,
        IMPLOT_AUTO_COL.2,
        IMPLOT_AUTO_COL.3,
    );
    push_style_color(
        &PlotColorElement::MarkerOutline,
        IMPLOT_AUTO_COL.0,
        IMPLOT_AUTO_COL.1,
        IMPLOT_AUTO_COL.2,
        IMPLOT_AUTO_COL.3,
    );
    push_style_color(
        &PlotColorElement::MarkerFill,
        IMPLOT_AUTO_COL.0,
        IMPLOT_AUTO_COL.1,
        IMPLOT_AUTO_COL.2,
        IMPLOT_AUTO_COL.3,
    );
    push_style_color(&PlotColorElement::ErrorBar, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::FrameBg, 1.00, 1.00, 1.00, 1.00);
    push_style_color(&PlotColorElement::PlotBg, 0.92, 0.92, 0.95, 1.00);
    push_style_color(&PlotColorElement::PlotBorder, 0.00, 0.00, 0.00, 0.00);
    push_style_color(&PlotColorElement::LegendBackground, 0.92, 0.92, 0.95, 1.00);
    push_style_color(&PlotColorElement::LegendBorder, 0.80, 0.81, 0.85, 1.00);
    push_style_color(&PlotColorElement::LegendText, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::TitleText, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::InlayText, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::XAxis, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::XAxisGrid, 1.00, 1.00, 1.00, 1.00);
    push_style_color(&PlotColorElement::YAxis, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::YAxisGrid, 1.00, 1.00, 1.00, 1.00);
    push_style_color(&PlotColorElement::YAxis2, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::YAxisGrid2, 1.00, 1.00, 1.00, 1.00);
    push_style_color(&PlotColorElement::YAxis3, 0.00, 0.00, 0.00, 1.00);
    push_style_color(&PlotColorElement::YAxisGrid3, 1.00, 1.00, 1.00, 1.00);
    push_style_color(&PlotColorElement::Selection, 1.00, 0.65, 0.00, 1.00);
    push_style_color(&PlotColorElement::Query, 0.23, 0.10, 0.64, 1.00);
    push_style_color(&PlotColorElement::Crosshairs, 0.23, 0.10, 0.64, 0.50);

    push_style_var_f32(&StyleVar::LineWeight, 1.5);
    push_style_var_i32(&StyleVar::Marker, Marker::None as i32);
    push_style_var_f32(&StyleVar::MarkerSize, 4.0);
    push_style_var_f32(&StyleVar::MarkerWeight, 1.0);
    push_style_var_f32(&StyleVar::FillAlpha, 1.0);
    push_style_var_f32(&StyleVar::ErrorBarSize, 5.0);
    push_style_var_f32(&StyleVar::ErrorBarWeight, 1.5);
    push_style_var_f32(&StyleVar::DigitalBitHeight, 8.0);
    push_style_var_f32(&StyleVar::DigitalBitGap, 4.0);
    push_style_var_f32(&StyleVar::PlotBorderSize, 0.0);
    push_style_var_f32(&StyleVar::MinorAlpha, 1.0);
    push_style_var_imvec2(&StyleVar::MajorTickLen, ImVec2 { x: 0.0, y: 0.0 });
    push_style_var_imvec2(&StyleVar::MinorTickLen, ImVec2 { x: 0.0, y: 0.0 });
    push_style_var_imvec2(&StyleVar::MajorTickSize, ImVec2 { x: 0.0, y: 0.0 });
    push_style_var_imvec2(&StyleVar::MinorTickSize, ImVec2 { x: 0.0, y: 0.0 });
    push_style_var_imvec2(&StyleVar::MajorGridSize, ImVec2 { x: 1.2, y: 1.2 });
    push_style_var_imvec2(&StyleVar::MinorGridSize, ImVec2 { x: 1.2, y: 1.2 });
    push_style_var_imvec2(&StyleVar::PlotPadding, ImVec2 { x: 12.0, y: 12.0 });
    push_style_var_imvec2(&StyleVar::LabelPadding, ImVec2 { x: 5.0, y: 5.0 });
    push_style_var_imvec2(&StyleVar::LegendPadding, ImVec2 { x: 5.0, y: 5.0 });
    push_style_var_imvec2(&StyleVar::InfoPadding, ImVec2 { x: 5.0, y: 5.0 });
    push_style_var_imvec2(&StyleVar::PlotMinSize, ImVec2 { x: 300.0, y: 225.0 });
}

pub fn style_ui(style: &mut Style) {
    const IMPLOT_AUTO_COL: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, -1.0);

    style.child_rounding = 3.0;
    style.grab_rounding = 0.0;
    style.window_rounding = 0.0;
    style.scrollbar_rounding = 3.0;
    style.frame_rounding = 3.0;
    style.window_title_align = [0.5, 0.5];

    style.colors[StyleColor::Text as usize] = [0.73, 0.73, 0.73, 1.00];
    style.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    style.colors[StyleColor::WindowBg as usize] = [0.26, 0.26, 0.26, 0.95];
    style.colors[StyleColor::ChildBg as usize] = [0.28, 0.28, 0.28, 1.00];
    style.colors[StyleColor::PopupBg as usize] = [0.26, 0.26, 0.26, 1.00];
    style.colors[StyleColor::Border as usize] = [0.26, 0.26, 0.26, 1.00];
    style.colors[StyleColor::BorderShadow as usize] = [0.26, 0.26, 0.26, 1.00];
    style.colors[StyleColor::FrameBg as usize] = [0.16, 0.16, 0.16, 1.00];
    style.colors[StyleColor::FrameBgHovered as usize] = [0.16, 0.16, 0.16, 1.00];
    style.colors[StyleColor::FrameBgActive as usize] = [0.16, 0.16, 0.16, 1.00];
    style.colors[StyleColor::TitleBg as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::TitleBgCollapsed as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::TitleBgActive as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::MenuBarBg as usize] = [0.26, 0.26, 0.26, 1.00];
    style.colors[StyleColor::ScrollbarBg as usize] = [0.21, 0.21, 0.21, 1.00];
    style.colors[StyleColor::ScrollbarGrab as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::CheckMark as usize] = [0.78, 0.78, 0.78, 1.00];
    style.colors[StyleColor::SliderGrab as usize] = [0.74, 0.74, 0.74, 1.00];
    style.colors[StyleColor::SliderGrabActive as usize] = [0.74, 0.74, 0.74, 1.00];
    style.colors[StyleColor::Button as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::ButtonHovered as usize] = [0.43, 0.43, 0.43, 1.00];
    style.colors[StyleColor::ButtonActive as usize] = [0.11, 0.11, 0.11, 1.00];
    style.colors[StyleColor::Header as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::HeaderHovered as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::HeaderActive as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::ResizeGrip as usize] = [0.36, 0.36, 0.36, 1.00];
    style.colors[StyleColor::ResizeGripHovered as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::ResizeGripActive as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::PlotLines as usize] = [0.39, 0.39, 0.39, 1.00];
    style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    style.colors[StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    style.colors[StyleColor::TextSelectedBg as usize] = [0.32, 0.52, 0.65, 1.00];
    style.colors[StyleColor::ModalWindowDimBg as usize] = [0.20, 0.20, 0.20, 0.50];
}
