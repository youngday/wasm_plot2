use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("tea"))
        .line(Line::new(|data: &MyData| data.y2).with_name("coffee"));
    let x_ticks = TickLabels::default();
    let y_ticks = TickLabels::default();
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                AxisMarker::left_edge().into_inner(),
                AxisMarker::horizontal_zero().into_inner(),
                XGridLine::from_ticks(x_ticks).into_inner(),
                YGridLine::from_ticks(y_ticks).into_inner(),
                YGuideLine::over_mouse().into_inner(),
                XGuideLine::over_data().into_inner(),
                InsetLegend::bottom_right().into_inner(),
            ]
        />
    }
}
