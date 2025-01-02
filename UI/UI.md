# Rust UI Development

## **Introduction**
Building user interfaces with Rust can be achieved using several libraries and frameworks, each tailored for different use cases, from desktop applications to web interfaces. Below is a comprehensive guide to UI development with Rust.

---

## **Frameworks and Libraries for UI Development**

### 1. **WebAssembly (Wasm)**
- Framework: **Yew**
- Purpose: Build single-page web applications.

#### Example: Simple Counter App
```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let count = use_state(|| 0);

    let increment = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html! {
        <>
            <button onclick={increment}>{"Increase"}</button>
            <p>{format!("Counter: {}", *count)}</p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

### 2. **Desktop Applications**
- Framework: **Tauri**
- Purpose: Build lightweight cross-platform desktop apps.

#### Example: Hello World Desktop App
```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

---

## **Reactive Programming with Rust**

### 3. **Dioxus**
- Purpose: Similar to React, for building declarative UIs.

#### Example: Todo App
```rust
use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    let todos = use_state(&cx, || vec![]);
    let new_todo = use_state(&cx, || String::new());

    let add_todo = {
        let todos = todos.clone();
        let new_todo = new_todo.clone();
        move |_| {
            todos.modify(|list| list.push(new_todo.to_string()));
            new_todo.set(String::new());
        }
    };

    cx.render(rsx! {
        input {
            value: "{new_todo}",
            oninput: move |e| new_todo.set(e.value.clone())
        }
        button { onclick: add_todo, "Add Todo" }
        ul {
            todos.iter().map(|todo| rsx!(li { "{todo}" }))
        }
    })
}

fn main() {
    dioxus::desktop::launch(app);
}
```

---

## **Game Development UIs**

### 4. **Bevy Engine**
- Purpose: Build interactive games with UI components.

#### Example: Simple Button UI
```rust
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            ..Default::default()
        },
        material: materials.add(asset_server.load("button.png").into()),
        ..Default::default()
    });
}
```

---

## **Learning Resources**

- **Yew Framework Documentation**: [https://yew.rs/docs/](https://yew.rs/docs/)
- **Dioxus**: [https://dioxuslabs.com/](https://dioxuslabs.com/)
- **Tauri**: [https://tauri.app/](https://tauri.app/)
- **Bevy Engine**: [https://bevyengine.org/](https://bevyengine.org/)

---

### **Next Steps**
1. Experiment with different frameworks.
2. Build small projects using the examples above.
3. Explore advanced UI features like animations, state management, and performance optimization.
