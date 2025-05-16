use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(|data: &MyData| data.y1)
        .line(|data: &MyData| data.y2)
        .with_x_range(0.0, 10.0)
        .with_y_range(-10.0, 10.0);
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                AxisMarker::bottom_edge().into_inner(),
                AxisMarker::horizontal_zero().into_inner(),
                AxisMarker::top_edge().into_inner(),
                AxisMarker::left_edge().with_arrow(false).into_inner(),
            ]
        />
    }
}
