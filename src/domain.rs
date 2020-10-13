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
