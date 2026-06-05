#[derive(Debug,serde::Deserialize, serde::Serialize)]
#[serde(tag = "key", content = "field", rename_all = "UPPERCASE")]
enum Message {
    Ready,
    Config(String),
    ConfigErr(String),
    EndSession
}

#[derive(Debug,serde::Deserialize, serde::Serialize)]
#[serde(tag = "key", content = "field", rename_all = "UPPERCASE")]
enum Config {
    Error(String),
    Ok(serde_json::Value)
}

struct Context {
    pub window: winit::window::Window,
    pub webview: wry::WebView,
}

struct App {
    pub mainw: Option<Context>,
    pub attr: winit::window::WindowAttributes,
    pub proxy: winit::event_loop::EventLoopProxy<Message>
}

impl App {
    fn new(proxy: winit::event_loop::EventLoopProxy<Message>) -> Self {
        let mut attr = winit::window::WindowAttributes::default();
        attr.title = "DDM".into();
        Self {
            mainw: None, 
            attr: attr,
            proxy
        }
    }

    fn window_builder(
        &mut self, 
        event_loop: &winit::event_loop::ActiveEventLoop
    ) -> (winit::window::Window, wry::WebViewBuilder) {
        let mut attr = self.attr.clone();
        attr.visible = false;
        let proxy = self.proxy.clone();
        let webview_builder = wry::WebViewBuilder::new().with_devtools(true).with_ipc_handler(
            move |m: wry::http::Request<String>| {
                serde_json::from_str(m.body()).map(|msg: Message| proxy.send_event(msg));
            },
        );

        let window = event_loop.create_window(attr).unwrap();
        (window, webview_builder)
    }

    pub fn main_proc(_url: &str, req: wry::http::Request<Vec<u8>>) -> wry::http::Response<std::borrow::Cow<'static, [u8]>> {
        const ENTRY: &[u8] = include_bytes!("../front/dist/index.html");
        const JS: &[u8] = include_bytes!("../front/dist/assets/index.js");
        const CSS: &[u8] = include_bytes!("../front/dist/assets/index.css");
        let uri = req.uri();
        let res = match uri.path() {
            "/" => {
                wry::http::Response::builder()
                    .status(wry::http::StatusCode::OK)
                    .header("Content-Type", "text/html")
                    .body(std::borrow::Cow::Borrowed(ENTRY))
                    .unwrap()
            }
            "/assets/index.js" => {
                wry::http::Response::builder()
                    .status(wry::http::StatusCode::OK)
                    .header("Content-Type", "application/javascript")
                    .body(std::borrow::Cow::Borrowed(JS))
                    .unwrap()
            }
            "/assets/index.css" => {
                wry::http::Response::builder()
                    .status(wry::http::StatusCode::OK)
                    .header("Content-Type", "text/css")
                    .body(std::borrow::Cow::Borrowed(CSS))
                    .unwrap()
            }
            _ => {
                wry::http::Response::default() 
            }
        };
        res
    }

    fn bootstrap(&mut self) {
        let mut home = std::env::home_dir().unwrap();
        home.push(".config");
        home.push("ddm");
        home.push("repo.json");
        let message = match std::fs::read_to_string(&home) {
            Err(e) => {
                Message::ConfigErr(e.to_string())
            }
            Ok(json) => {
                Message::Config(json)
           }
        };
        self.proxy.send_event(message);
    }
}

impl winit::application::ApplicationHandler<Message> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.mainw = {
            let (window, webview_builder) = self.window_builder(event_loop);
            let webview = if cfg!(debug_assertions) {
                webview_builder
                    .with_url("http://localhost:5173/")
                    .build(&window)
                    .unwrap()
                } else {
                webview_builder
                    .with_url("main://index.html")
                    .with_custom_protocol("main".into(), move |url, req| App::main_proc(url, req))
                    .build(&window)
                    .unwrap()
            };
            Some(Context {
                window,
                webview
            })
        };
    }

    fn user_event(
        &mut self, 
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: Message
    ) {
        match event {
            Message::Ready => {
                self.bootstrap();
                if let Some (context ) = &self.mainw {
                    context.window.set_visible(true);
                    context.window.set_resizable(true);
                    context.webview.set_visible(true);
                }
            }
            Message::Config(_) | Message::ConfigErr(_) => {
                if let Some (context ) = &self.mainw {
                    if let Ok(json) = serde_json::to_value(&event) {
                        context.webview.evaluate_script(&format!("window.receive({});", json));
                    }
                }
            }
            Message::EndSession => {
                event_loop.exit()
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent;
        match event {
            WindowEvent::CloseRequested => {
                self.proxy.send_event(Message::EndSession);
            }
            _ => ()
        }
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::with_user_event().build().unwrap();
    let mut app = App::new(event_loop.create_proxy());
    event_loop.run_app(&mut app);
}
