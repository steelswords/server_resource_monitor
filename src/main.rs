use yew::{yew_hooks,function_component, html, Html};
use plotly::{Plot, Scatter};
use yew::prelude::*;

#[function_component(PlotComponent)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();
        let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
        plot.add_trace(trace);

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

        use_effect_with_deps(move |_| {
            p.run();
            || ()
        }, (),
    );

    html! {
        <div id="plot-div"></div>
    }
}

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    //html! { <HelloWorld /> }
    plot_component
}

fn main() {
    yew::Renderer::<App>::new().render();
}
