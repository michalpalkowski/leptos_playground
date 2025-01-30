use leptos::{logging::log, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_router::path;
use leptos::server_fn::codec::{StreamingText, TextStream};
use futures::{channel::mpsc, StreamExt};
use leptos::task::spawn_local;
use leptos::{
    component, server, view, IntoView,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-playground.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (stream_value, set_stream_value) = signal(0);
    let (is_counting, set_is_counting) = signal(false);

    let start_counting = move |_| {
        set_is_counting.set(true);
        
        spawn_local(async move {
            if let Ok(mut stream) = get_counter().await {
                let mut stream = stream.into_inner();
                
                while let Some(Ok(value)) = stream.next().await {
                    if let Ok(number) = value.parse::<i32>() {
                        log!("Received new value: {}", number);
                        set_stream_value.set(number);
                    }
                }
            }
        });
    };

    view! {
        <div>
            <h1>"Welcome to Leptos!"</h1>
            <div>
                <button 
                    on:click=start_counting
                    disabled=move || is_counting.get()
                >
                    "Start Counting"
                </button>
                <div>
                    "Counter value: " {stream_value}
                </div>
            </div>
        </div>
    }
}

#[server(output = StreamingText)]
pub async fn get_counter() -> Result<TextStream, ServerFnError> {
    use futures::stream::{self, Stream};
    use std::time::Duration;
    use tokio::time::sleep;

    let counter_stream = stream::unfold(0, |count| async move {
        if count >= 100 {
            None
        } else {
            sleep(Duration::from_secs(1)).await;
            let new_count = count + 1;
            Some((Ok(new_count.to_string()), new_count))
        }
    });

    Ok(TextStream::new(counter_stream))
}
