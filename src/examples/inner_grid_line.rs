use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(|data: &MyData| data.y1)
        .line(|data: &MyData| data.y2);
    let y_ticks = TickLabels::aligned_floats();
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                XGridLine::default().into_inner(),
                YGridLine::from_ticks(y_ticks).into_inner(),
            ]
        />
    }
}
