# QuietDrop Frontend Development Guide

This document provides guidance for developers working on the QuietDrop frontend, which is built using Yew and WebAssembly for both desktop and mobile platforms with Tauri 2.0.

## Table of Contents

- [Overview](#overview)
- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Component Architecture](#component-architecture)
- [State Management](#state-management)
- [Tauri 2.0 Integration](#tauri-20-integration)
- [Cross-Platform UI Development](#cross-platform-ui-development)
- [Styling](#styling)
- [Testing](#testing)
- [Best Practices](#best-practices)
- [Common Challenges](#common-challenges)

## Overview

QuietDrop's frontend is built with Yew, a modern Rust framework for creating web applications. Unlike traditional web applications that use JavaScript, QuietDrop compiles Rust code to WebAssembly, providing type safety, memory safety, and performance benefits while maintaining a reactive user interface that works across desktop and mobile platforms.

## Technology Stack

- **Yew**: Rust-based frontend framework
- **WebAssembly**: Binary instruction format for browser execution
- **Tauri 2.0**: Cross-platform application framework
- **Tauri Commands**: Bridge between frontend and backend
- **Trunk**: Build tool for Rust web applications
- **CSS**: Styling (with responsive design)
- **Rust**: Programming language throughout the stack

## Getting Started

### Prerequisites

- Rust toolchain
- WebAssembly target
- Trunk (Yew build tool)
- Tauri CLI

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install Tauri CLI
cargo install tauri-cli

# For mobile development
# Android: Install Android SDK, NDK
# iOS: Install Xcode, iOS development certificates
```

### Running the Development Server

```bash
# Clone the repository (if not already done)
git clone https://github.com/chizy7/QuietDrop.git
cd QuietDrop

# For desktop development
cd quietdrop-tauri
cargo tauri dev

# For Android development
cd quietdrop-tauri
cargo tauri android dev

# For iOS development
cd quietdrop-tauri
cargo tauri ios dev
```

## Project Structure

```
quietdrop-tauri/
├── Cargo.toml          # Frontend dependencies
├── index.html          # HTML entry point
├── styles.css          # Global styles
├── src/                # Frontend Rust code
│   ├── main.rs         # Application entry point
│   ├── app.rs          # Main application component
│   ├── components/     # UI components
│   │   ├── mod.rs      # Component exports
│   │   ├── chat.rs     # Chat component
│   │   ├── login.rs    # Login component
│   │   ├── message.rs  # Message components
│   │   └── ...         # Other components
│   ├── models/         # Data models
│   │   ├── mod.rs      # Model exports
│   │   └── message.rs  # Message model
│   ├── services/       # Frontend services
│   │   ├── mod.rs      # Service exports
│   │   └── tauri.rs    # Tauri command wrappers
│   ├── utils/          # Utility functions
│   │   ├── mod.rs      # Utility exports
│   │   └── platform.rs # Platform detection utilities
│   └── platform/       # Platform-specific code
│       ├── mod.rs      # Platform exports
│       ├── desktop.rs  # Desktop-specific components
│       └── mobile.rs   # Mobile-specific components
└── src-tauri/          # Tauri backend (separate)
```

## Component Architecture

QuietDrop uses a component-based architecture with Yew's functional components, designed to be responsive across different platforms:

### Basic Component Structure

```rust
use yew::prelude::*;
use crate::utils::platform::is_mobile_device;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub username: String,
    #[prop_or_default]
    pub messages: Vec<Message>,
    pub on_send: Callback<String>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let message = use_state(|| String::new());
    let is_mobile = is_mobile_device();
    
    let onsubmit = {
        let on_send = props.on_send.clone();
        let message = message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_send.emit((*message).clone());
            message.set(String::new());
        })
    };
    
    let oninput = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                message.set(input.value());
            }
        })
    };
    
    html! {
        <div class={classes!("chat", if *is_mobile { "mobile" } else { "desktop" })}>
            <div class="messages">
                {for props.messages.iter().map(|msg| html! {
                    <div class="message">
                        <div class="sender">{&msg.sender}</div>
                        <div class="content">{&msg.content}</div>
                    </div>
                })}
            </div>
            <form onsubmit={onsubmit} class="message-form">
                <input 
                    type="text" 
                    value={(*message).clone()} 
                    oninput={oninput} 
                    placeholder="Type a message..." 
                    class={classes!(if *is_mobile { "mobile-input" } else { "desktop-input" })}
                />
                <button type="submit">{"Send"}</button>
            </form>
        </div>
    }
}
```

### Platform-Specific Components

For components that need different layouts on desktop vs. mobile:

```rust
use yew::prelude::*;
use crate::utils::platform::is_mobile_device;
use crate::platform::{desktop, mobile};

#[function_component(ConversationView)]
pub fn conversation_view() -> Html {
    let is_mobile = is_mobile_device();
    
    if *is_mobile {
        html! { <mobile::ConversationView /> }
    } else {
        html! { <desktop::ConversationView /> }
    }
}
```

### Component Composition

```rust
#[function_component(App)]
fn app() -> Html {
    let username = use_state(|| String::new());
    let messages = use_state(|| Vec::<Message>::new());
    let logged_in = use_state(|| false);
    let is_mobile = is_mobile_device();
    
    // Logic for handling login and messages
    
    html! {
        <div class={classes!("app", if *is_mobile { "mobile-app" } else { "desktop-app" })}>
            <Header username={(*username).clone()} is_mobile={*is_mobile} />
            
            if *logged_in {
                <Chat 
                    username={(*username).clone()} 
                    messages={(*messages).clone()} 
                    on_send={on_message_send} 
                />
            } else {
                <Login on_login={on_login} is_mobile={*is_mobile} />
            }
        </div>
    }
}
```

## State Management

QuietDrop uses Yew's built-in state management tools:

### Local Component State

```rust
// For simple state within a component
let counter = use_state(|| 0);

// Update state
let onclick = {
    let counter = counter.clone();
    Callback::from(move |_| {
        counter.set(*counter + 1);
    })
};
```

### Context for Global State

```rust
// Define a context
#[derive(Clone, PartialEq)]
struct AppContext {
    username: String,
    logged_in: bool,
    is_mobile: bool, // Platform information
}

// Provide context
#[function_component(App)]
fn app() -> Html {
    let is_mobile = is_mobile_device();
    let user_ctx = use_state(|| AppContext {
        username: String::new(),
        logged_in: false,
        is_mobile: *is_mobile,
    });
    
    html! {
        <ContextProvider<AppContext> context={(*user_ctx).clone()}>
            <AppContent />
        </ContextProvider<AppContext>>
    }
}

// Consume context in a child component
#[function_component(Header)]
fn header() -> Html {
    let user_ctx = use_context::<AppContext>().expect("No user context found");
    
    html! {
        <header class={classes!(if user_ctx.is_mobile { "mobile-header" } else { "desktop-header" })}>
            if user_ctx.logged_in {
                <span>{"Welcome, "}{&user_ctx.username}</span>
            } else {
                <span>{"Please log in"}</span>
            }
        </header>
    }
}
```

### Reducers for Complex State

```rust
enum Action {
    AddMessage(Message),
    ClearMessages,
    UpdateUser(String),
    SetMobile(bool),
}

struct AppState {
    messages: Vec<Message>,
    username: String,
    logged_in: bool,
    is_mobile: bool,
}

impl Reducible for AppState {
    type Action = Action;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::AddMessage(message) => {
                let mut messages = self.messages.clone();
                messages.push(message);
                Self {
                    messages,
                    username: self.username.clone(),
                    logged_in: self.logged_in,
                    is_mobile: self.is_mobile,
                }.into()
            },
            Action::ClearMessages => Self {
                messages: Vec::new(),
                username: self.username.clone(),
                logged_in: self.logged_in,
                is_mobile: self.is_mobile,
            }.into(),
            Action::UpdateUser(username) => Self {
                messages: self.messages.clone(),
                username,
                logged_in: true,
                is_mobile: self.is_mobile,
            }.into(),
            Action::SetMobile(is_mobile) => Self {
                messages: self.messages.clone(),
                username: self.username.clone(),
                logged_in: self.logged_in,
                is_mobile,
            }.into(),
        }
    }
}

// Using the reducer in a component
#[function_component(App)]
fn app() -> Html {
    let is_mobile = is_mobile_device();
    let state = use_reducer(|| AppState {
        messages: Vec::new(),
        username: String::new(),
        logged_in: false,
        is_mobile: *is_mobile,
    });
    
    // Update mobile state when it changes
    {
        let state = state.clone();
        let is_mobile_value = *is_mobile;
        use_effect_with_deps(
            move |_| {
                state.dispatch(Action::SetMobile(is_mobile_value));
                || ()
            },
            is_mobile_value
        );
    }
    
    // Rest of component...
}
```

## Tauri 2.0 Integration

The frontend communicates with the Rust backend through Tauri commands, which work consistently across desktop and mobile:

### Command Invocation

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};
use gloo_utils::format::JsValueSerdeExt;

#[derive(Serialize)]
struct MessageRequest {
    content: String,
    recipient: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    id: String,
    status: String,
}

// Invoke a Tauri command
fn send_message(content: String, recipient: String) {
    let request = MessageRequest { content, recipient };
    
    spawn_local(async move {
        let response = invoke::<_, MessageResponse>(
            "send_message",
            &JsValue::from_serde(&request).unwrap()
        ).await;
        
        match response {
            Ok(result) => {
                // Handle success
                web_sys::console::log_1(&format!("Message sent: {}", result.id).into());
            },
            Err(e) => {
                // Handle error
                web_sys::console::error_1(&format!("Error: {}", e).into());
            }
        }
    });
}
```

### Creating a Service Layer

For better organization and platform handling, wrap Tauri commands in service modules:

```rust
// services/message_service.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_utils::format::JsValueSerdeExt;
use crate::models::message::{Message, MessageRequest, MessageResponse};
use crate::utils::platform::is_mobile_device;

pub fn send_message(
    content: String,
    recipient: String,
    callback: impl Fn(Result<MessageResponse, String>) + 'static
) {
    let request = MessageRequest { content, recipient };
    let is_mobile = is_mobile_device();
    
    // Mobile-specific handling
    if *is_mobile {
        // Check network connectivity first
        spawn_local(async move {
            match invoke::<_, bool>("check_network_status", &JsValue::null()).await {
                Ok(connected) => {
                    if !connected {
                        callback(Err("No network connection available".to_string()));
                        return;
                    }
                    
                    // Continue with sending if connected
                    send_message_internal(request, callback);
                },
                Err(e) => {
                    callback(Err(format!("Failed to check network status: {}", e)));
                }
            }
        });
    } else {
        // Desktop path - proceed directly
        send_message_internal(
            request, callback);
    }
}

fn send_message_internal(
    request: MessageRequest,
    callback: impl Fn(Result<MessageResponse, String>) + 'static
) {
    spawn_local(async move {
        let response = invoke::<_, MessageResponse>(
            "send_message",
            &JsValue::from_serde(&request).unwrap()
        ).await;
        
        callback(response);
    });
}

pub fn get_messages(callback: impl Fn(Result<Vec<Message>, String>) + 'static) {
    spawn_local(async move {
        let response = invoke::<_, Vec<Message>>("get_messages", &JsValue::null()).await;
        callback(response);
    });
}
```

### Using Services in Components

```rust
use crate::services::message_service;
use crate::utils::platform::is_mobile_device;

#[function_component(ChatView)]
fn chat_view() -> Html {
    let messages = use_state(|| Vec::<Message>::new());
    let input_value = use_state(|| String::new());
    let status = use_state(|| String::new());
    let is_mobile = is_mobile_device();
    
    // Load messages on component mount
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                message_service::get_messages(move |result| {
                    match result {
                        Ok(new_messages) => messages.set(new_messages),
                        Err(e) => web_sys::console::error_1(&format!("Error: {}", e).into()),
                    }
                });
                || ()
            },
            ()
        );
    }
    
    let on_send = {
        let input_value = input_value.clone();
        let status = status.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let content = (*input_value).clone();
            
            if content.is_empty() {
                return;
            }
            
            status.set("Sending...".to_string());
            
            message_service::send_message(
                content,
                "recipient".to_string(),
                move |result| {
                    match result {
                        Ok(_) => {
                            status.set("Message sent".to_string());
                            input_value.set(String::new());
                        },
                        Err(e) => {
                            status.set(format!("Error: {}", e));
                        }
                    }
                }
            );
        })
    };
    
    html! {
        <div class={classes!("chat-container", if *is_mobile { "mobile" } else { "desktop" })}>
            <div class="messages-list">
                {for messages.iter().map(message_view)}
            </div>
            
            <form onsubmit={on_send} class="message-form">
                <input
                    type="text"
                    value={(*input_value).clone()}
                    oninput={on_input_change(input_value.clone())}
                    placeholder="Type a message..."
                    class={if *is_mobile { "mobile-input" } else { "desktop-input" }}
                />
                <button 
                    type="submit"
                    class={if *is_mobile { "mobile-button" } else { "desktop-button" }}
                >
                    {"Send"}
                </button>
            </form>
            
            <div class="status-text">{(*status).clone()}</div>
        </div>
    }
}
```

## Cross-Platform UI Development

### Platform Detection

```rust
// utils/platform.rs
use web_sys::{window, Navigator};
use yew::prelude::*;

pub fn is_mobile_device() -> UseStateHandle<bool> {
    let is_mobile = use_state(|| {
        window()
            .map(|win| {
                let navigator = win.navigator();
                let user_agent = navigator.user_agent().unwrap_or_default();
                
                user_agent.contains("Android") ||
                user_agent.contains("iPhone") ||
                user_agent.contains("iPad") ||
                user_agent.contains("Mobile")
            })
            .unwrap_or(false)
    });
    
    // Check for orientation changes that might indicate tablet/desktop mode
    {
        let is_mobile = is_mobile.clone();
        use_effect_with_deps(
            move |_| {
                let listener = window().and_then(|win| {
                    let is_mobile_clone = is_mobile.clone();
                    
                    let callback = Closure::wrap(Box::new(move || {
                        // Recheck mobile status after orientation change
                        let win = window().unwrap();
                        let navigator = win.navigator();
                        let user_agent = navigator.user_agent().unwrap_or_default();
                        
                        let is_mobile_device = 
                            user_agent.contains("Android") ||
                            user_agent.contains("iPhone") ||
                            user_agent.contains("iPad") ||
                            user_agent.contains("Mobile");
                        
                        // Update state if different
                        if is_mobile_device != *is_mobile_clone {
                            is_mobile_clone.set(is_mobile_device);
                        }
                    }) as Box<dyn FnMut()>);
                    
                    win.add_event_listener_with_callback(
                        "orientationchange",
                        callback.as_ref().unchecked_ref()
                    ).ok()?;
                    
                    Some(callback)
                });
                
                // Clean up
                move || {
                    // Keep the listener alive
                    drop(listener);
                }
            },
            ()
        );
    }
    
    is_mobile
}

// Additional platform capabilities detection
pub fn has_touch_support() -> bool {
    window()
        .map(|win| {
            win.navigator()
                .max_touch_points()
                .map(|points| points > 0)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

// Check screen size categories
pub fn screen_size_category() -> ScreenSize {
    window()
        .map(|win| {
            let width = win.inner_width().unwrap().as_f64().unwrap_or(0.0);
            
            if width < 480.0 {
                ScreenSize::Small // Mobile phone
            } else if width < 768.0 {
                ScreenSize::Medium // Tablet portrait
            } else if width < 1024.0 {
                ScreenSize::Large // Tablet landscape
            } else {
                ScreenSize::ExtraLarge // Desktop
            }
        })
        .unwrap_or(ScreenSize::Medium)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScreenSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}
```

### Responsive Components

Create components that adapt to different platforms and screen sizes:

```rust
#[function_component(MessageList)]
fn message_list() -> Html {
    let messages = use_state(|| Vec::<Message>::new());
    let is_mobile = is_mobile_device();
    let screen_size = screen_size_category();
    
    // Determine list style based on screen size
    let list_style = match screen_size {
        ScreenSize::Small => "mobile-list compact",
        ScreenSize::Medium => "mobile-list",
        ScreenSize::Large | ScreenSize::ExtraLarge => "desktop-list",
    };
    
    // Determine how many messages to show
    let visible_message_count = match screen_size {
        ScreenSize::Small => 5,
        ScreenSize::Medium => 10,
        ScreenSize::Large => 15,
        ScreenSize::ExtraLarge => 20,
    };
    
    html! {
        <div class={list_style}>
            {for messages.iter().take(visible_message_count).map(|msg| {
                html! {
                    <MessageItem 
                        message={msg.clone()} 
                        compact={screen_size == ScreenSize::Small}
                        on_mobile={*is_mobile}
                    />
                }
            })}
        </div>
    }
}
```

### Conditional UI Elements

Show or hide elements based on platform:

```rust
#[function_component(AppHeader)]
fn app_header() -> Html {
    let is_mobile = is_mobile_device();
    
    html! {
        <header class="app-header">
            <div class="logo">{"QuietDrop"}</div>
            
            // Show menu icon on mobile, full menu on desktop
            if *is_mobile {
                <button class="menu-button">{"☰"}</button>
            } else {
                <nav class="desktop-menu">
                    <a href="#messages">{"Messages"}</a>
                    <a href="#contacts">{"Contacts"}</a>
                    <a href="#settings">{"Settings"}</a>
                    <a href="#logout">{"Logout"}</a>
                </nav>
            }
        </header>
    }
}
```

### Touch vs. Mouse Interactions

Handle different input methods:

```rust
#[function_component(InteractiveElement)]
fn interactive_element() -> Html {
    let is_mobile = is_mobile_device();
    let has_touch = has_touch_support();
    
    let on_interact = {
        Callback::from(move |e: MouseEvent| {
            // Handle interaction
            e.prevent_default();
            // Action logic
        })
    };
    
    let on_touch = {
        Callback::from(move |e: TouchEvent| {
            // Handle touch with different behavior if needed
            e.prevent_default();
            // Touch-specific logic
        })
    };
    
    html! {
        <div 
            class={classes!("interactive", if *is_mobile { "touch-target" } else { "mouse-target" })}
            onclick={on_interact}
            ontouchstart={if has_touch { on_touch } else { Callback::noop() }}
        >
            {"Interact with me"}
        </div>
    }
}
```

## Event Listening

Listen for Tauri events from the backend:

```rust
use gloo_utils::window;
use js_sys::Function;
use wasm_bindgen::prelude::*;

// Listen for events from backend
fn setup_event_listeners() -> Option<Closure<dyn FnMut(JsValue)>> {
    let messages = use_state(|| Vec::<Message>::new());
    
    let messages_clone = messages.clone();
    let callback = Closure::wrap(Box::new(move |event: JsValue| {
        let message: Message = event.into_serde().unwrap();
        let mut current_messages = (*messages_clone).clone();
        current_messages.push(message);
        messages_clone.set(current_messages);
    }) as Box<dyn FnMut(JsValue)>);
    
    window()
        .and_then(|w| w.get("__TAURI__").dyn_into::<JsValue>().ok())
        .and_then(|t| js_sys::Reflect::get(&t, &"event".into()).ok())
        .and_then(|e| js_sys::Reflect::get(&e, &"listen".into()).ok())
        .and_then(|l| l.dyn_into::<Function>().ok())
        .map(|listen_fn| {
            listen_fn.call3(
                &JsValue::NULL,
                &"new-message".into(),
                &callback.as_ref().into(),
            ).unwrap();
            
            // Return the callback to keep it alive
            callback
        })
}

#[function_component(MessageMonitor)]
fn message_monitor() -> Html {
    // Set up event listener on component mount
    let _listener = use_state(|| setup_event_listeners());
    
    // Component body
    html! {
        <div>{"Monitoring for new messages..."}</div>
    }
}
```

## Styling

QuietDrop uses CSS for styling with responsive design principles:

### Responsive Design

```css
/* styles.css */
:root {
    --primary-color: #264653;
    --secondary-color: #2a9d8f;
    --accent-color: #e9c46a;
    --text-color: #333;
    --background-color: #f5f5f5;
    
    /* Spacing variables for consistency */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;
    
    /* Touch targets */
    --touch-target-size: 44px;
}

/* Base styles */
body {
    font-family: 'Inter', sans-serif;
    margin: 0;
    padding: 0;
    background-color: var(--background-color);
    color: var(--text-color);
}

/* Responsive containers */
.app {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--spacing-md);
}

/* Desktop styles */
.desktop-app {
    display: grid;
    grid-template-columns: 250px 1fr;
    grid-gap: var(--spacing-lg);
}

/* Mobile styles */
.mobile-app {
    display: flex;
    flex-direction: column;
}

/* Responsive components */
.chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
}

.desktop .chat-container {
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.mobile .chat-container {
    border-radius: 0;
    box-shadow: none;
}

/* Touch targets for mobile */
.mobile-button {
    min-height: var(--touch-target-size);
    min-width: var(--touch-target-size);
    padding: var(--spacing-md);
}

.desktop-button {
    height: 32px;
    padding: 0 var(--spacing-md);
}

/* Media queries for responsive design */
@media (max-width: 768px) {
    .desktop-only {
        display: none;
    }
    
    .input-group {
        flex-direction: column;
    }
}

@media (min-width: 769px) {
    .mobile-only {
        display: none;
    }
    
    .input-group {
        flex-direction: row;
    }
}
```

### Applying Styles in Components

```rust
html! {
    <div class={classes!(
        "message-input",
        if *is_mobile { "mobile" } else { "desktop" },
        match screen_size {
            ScreenSize::Small => "compact",
            ScreenSize::Medium => "standard",
            ScreenSize::Large | ScreenSize::ExtraLarge => "expanded",
        }
    )}>
        // Component content
    </div>
}
```

## Testing

Testing the frontend requires specific tools for WebAssembly and accommodating both desktop and mobile platforms:

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_message_parsing() {
        let json = r#"{"content":"Hello","sender":"Alice","timestamp":"2023-01-01T12:00:00Z"}"#;
        let message: Message = serde_json::from_str(json).unwrap();
        
        assert_eq!(message.content, "Hello");
        assert_eq!(message.sender, "Alice");
    }
}
```

### Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::start_app;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_responsive_component() {
        // Simulate mobile viewport
        js_sys::eval("window.innerWidth = 375; window.innerHeight = 667;").unwrap();
        window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
        
        // Mount the component
        let handle = yew::start_app_in_element::<ResponsiveComponent>(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("output")
                .unwrap()
        );
        
        // Give time for platform detection to run
        gloo_timers::future::TimeoutFuture::new(100).await;
        
        // Check if mobile class is applied
        let component = gloo_utils::document()
            .query_selector(".responsive-component")
            .unwrap()
            .unwrap();
        
        assert!(component.class_list().contains("mobile"), 
               "Component should have 'mobile' class on small viewport");
        
        // Simulate desktop viewport
        js_sys::eval("window.innerWidth = 1280; window.innerHeight = 800;").unwrap();
        window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
        
        // Give time for platform detection to run
        gloo_timers::future::TimeoutFuture::new(100).await;
        
        // Check if desktop class is applied
        assert!(component.class_list().contains("desktop"), 
               "Component should have 'desktop' class on large viewport");
        
        // Clean up
        handle.destroy();
    }
}
```

## Best Practices

### 1. Cross-Platform Design

- Design for mobile first, then scale up to desktop
- Use relative units (rem, %, vh/vw) instead of fixed pixels
- Test on various device sizes and orientations
- Create responsive components that adapt to available space

### 2. Performance Optimization

- Minimize state updates to reduce re-renders
- Use `use_memo` for expensive computations
- Implement virtualized lists for large datasets
- Optimize assets and bundle size for mobile
- Consider API call batching and caching

### 3. Touch-Friendly UIs

- Use larger touch targets for mobile (minimum 44×44 pixels)
- Provide appropriate feedback for touch interactions
- Avoid hover-dependent interactions on mobile
- Test with touch events, not just mouse events

### 4. Platform-Specific UX

- Follow platform UI conventions when appropriate
- Adapt layouts for different screen sizes
- Consider different input methods (touch vs. mouse/keyboard)
- Handle platform-specific capabilities gracefully

### 5. Offline Support

- Implement offline detection and handling
- Cache essential data for offline use
- Provide clear feedback about connection status
- Queue operations for later execution when offline

### 6. Code Organization

- Group related components in modules
- Use services for backend communication
- Separate platform-specific code when possible
- Create utility functions for common operations

## Common Challenges

### 1. Platform Detection Limitations

WebAssembly has limited access to device information. Use a combination of:

```rust
// Check user agent
let is_mobile_by_agent = window()
    .map(|win| {
        let user_agent = win.navigator().user_agent().unwrap_or_default();
        user_agent.contains("Mobile") || user_agent.contains("Android")
    })
    .unwrap_or(false);

// Check screen dimensions
let is_mobile_by_size = window()
    .map(|win| {
        let width = win.inner_width().unwrap().as_f64().unwrap_or(0.0);
        width < 768.0
    })
    .unwrap_or(false);

// Check touch capability
let has_touch = window()
    .map(|win| win.navigator().max_touch_points() > 0)
    .unwrap_or(false);

let is_probably_mobile = is_mobile_by_agent || (is_mobile_by_size && has_touch);
```

### 2. Different Interaction Models

Handle both touch and mouse interactions:

```rust
// Common handler that works for both
let on_interact = {
    let message = message.clone();
    Callback::from(move |_| {
        // Action that works for both
    })
};

html! {
    <div 
        onclick={on_interact.clone()}
        ontouchend={on_interact}
        class={if *is_mobile { "touch-item" } else { "clickable-item" }}
    >
        {"Interact with me"}
    </div>
}
```

### 3. Tauri Command Handling

Handle platform-specific command behavior:

```rust
// In Tauri backend
#[tauri::command]
async fn platform_specific_command(app: AppHandle) -> Result<String, String> {
    #[cfg(desktop)]
    {
        // Desktop-specific implementation
        let path = app.path_resolver().app_data_dir().unwrap();
        // ...
        return Ok("Desktop operation completed".to_string());
    }
    
    #[cfg(target_os = "android")]
    {
        // Android-specific implementation
        // ...
        return Ok("Android operation completed".to_string());
    }
    
    #[cfg(target_os = "ios")]
    {
        // iOS-specific implementation
        // ...
        return Ok("iOS operation completed".to_string());
    }
    
    // Fallback
    Err("Operation not supported on this platform".to_string())
}
```

## Resources

- [Yew Documentation](https://yew.rs/docs/)
- [Tauri 2.0 API](https://v2.tauri.app/reference/javascript/api/)
- [WebAssembly MDN Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [Mobile UI Design Patterns](https://www.smashingmagazine.com/2018/02/comprehensive-guide-to-mobile-app-design/)

## Conclusion

The QuietDrop frontend leverages Yew and WebAssembly through Tauri 2.0 to create a unified codebase that works across desktop and mobile platforms. By following responsive design principles and handling platform-specific considerations, the application provides a consistent and optimized experience on all devices.

When developing for the QuietDrop frontend, remember to always test on multiple platforms and screen sizes, optimize for performance, and create adaptive UI components that work well with both touch and pointer input methods.