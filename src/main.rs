#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "tag", content = "payload", rename_all = "UPPERCASE")]
enum Message {
    
}

struct Context {
    pub window: winit::window::Window,
    pub webview: wry::WebView,
}

struct App {
    pub mainw: Option<Context>,
    pub playw: Option<Context>,
    pub attr: winit::window::WindowAttributes,
    pub proxy: winit::event_loop::EventLoopProxy<Message>
}

impl App {
    fn new(proxy: winit::event_loop::EventLoopProxy<Message>) -> Self {
        let mut attr = winit::window::WindowAttributes::default();
        attr = attr.with_decorations(false);
        Self {
            mainw: None,
            playw: None,
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

    pub fn main_proc(url: &str, req: wry::http::Request<Vec<u8>>) -> wry::http::Response<std::borrow::Cow<'static, [u8]>> {
        wry::http::Response::default() 
    }

    
    pub fn play_proc(url: &str, req: wry::http::Request<Vec<u8>>) -> wry::http::Response<std::borrow::Cow<'static, [u8]>> {
         wry::http::Response::default()       
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

        self.playw = {
            let (window, webview_builder) = self.window_builder(event_loop);
            let webview = if cfg!(debug_assertions) {
                webview_builder
                    .with_url("http://localhost:5173/")
                    .build(&window)
                    .unwrap()
                } else {
                webview_builder
                    .with_url("play://index.html")
                    .with_custom_protocol("play".into(), move |url, req| App::play_proc(url, req))
                    .build(&window)
                    .unwrap()
            };
            Some(Context {
                window,
                webview
            })
        };
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::with_user_event().build().unwrap();
    let mut app = App::new(event_loop.create_proxy());
    event_loop.run_app(&mut app);
    println!("Hello, world!");
}
