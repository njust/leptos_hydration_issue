use crate::error_template::{AppError, ErrorTemplate};
use html::ul;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/issue-repro.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage ssr=SsrMode::PartiallyBlocked/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! { <NestedArgs cnt=1/> }
}

#[server]
async fn data_fn(ts: u64) -> Result<(), ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(ts * 100)).await;

    Ok(())
}

#[component]
fn NestedArgs(cnt: u64) -> impl IntoView {
    let load_data = create_blocking_resource(move || cnt, data_fn);
    view! {
        <div>
            <Suspense fallback=move || {
                view! { Loading }
            }>
                {move || {
                    load_data
                        .and_then(|_| {
                            let mut views = vec![];
                            if cnt <= 3 {
                                let list = (0..cnt)
                                    .into_iter()
                                    .fold(
                                        ul(),
                                        |list, i| {
                                            let el = view! {
                                                <li>
                                                    <div>{cnt} - {i}</div>
                                                    <NestedArgs cnt=cnt + 1/>
                                                </li>
                                            }
                                                .into_view();
                                            list.child(el)
                                        },
                                    );
                                views.push(list.into_view());
                            }
                            views
                        })
                }}

            </Suspense>
        </div>
    }
}
