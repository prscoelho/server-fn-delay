use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(Foo, "/api")]
async fn server_foo(with_delay: bool) -> Result<bool, ServerFnError> {
    use tokio::time::{sleep, Duration};

    if with_delay {
        sleep(Duration::from_secs(5)).await;
    }

    Ok(with_delay)
}

async fn fetch_server_foo(with_delay: Option<bool>) -> Option<bool> {
    if let Some(with_delay) = with_delay {
        server_foo(with_delay).await.ok()
    } else {
        None
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (with_delay, set_with_delay) = create_signal(cx, None);
    let resource = create_resource(cx, with_delay, fetch_server_foo);

    let on_click_with_delay = move |_| {
        set_with_delay(Some(true));
    };

    let on_click_without_delay = move |_| {
        set_with_delay(Some(false));
    };

    view! { cx,
        <button on:click=on_click_with_delay>"With delay"</button>
        <button on:click=on_click_without_delay>"Without delay"</button>
        { move || with_delay().map(|delay| if delay {
            view! { cx, <p>"Input has delay"</p> }
        } else {
            view! { cx, <p>"Input has no delay"</p> }
        }) }
        <Suspense fallback=move || view!{ cx, <p>"Loading.."</p> }>
        { move || resource.read(cx).flatten().map(|delay| if delay {
            view! { cx, <p>"Output had delay"</p> }
            } else {
                view! { cx, <p>"Output had no delay"</p> }
            }) }
        </Suspense>
    }
}
