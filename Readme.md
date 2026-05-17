## SimpleChat

> **SimpleChat** — is a unified client-server application designed to transfer and broadcast messages across all active connections. The project is built using the **Rust** programming language and the **Slint** as user interface.

#### Architectural features:
- **Multithreading** — The application separates UI and backend logic threads to make the interface never freeze during heavy operations.
- **Asynchronous** — Core commands and network I/O operations are working asynchronously in an isolated thread pool.
- **Command Bridge** — Communication between the UI and the core execution layers are *pre-defined* & *compile-time safe* command options.
- **Input Validation** — Real-time verification to ensure that IP addresses, Port numbers, and Usernames satisfies requirements before establishing connection.
- **Data autosave** — Automatically stores and updates previously entered user configurations (IP, Port, Username) for comfortable application restarts.
- **Logging** — Structured application logging to track critical runtime events, peer connections, and errors.


### Technological stack:
- **Programming Language:** Rust (2024)
- **GUI:** Slint (v1.16.1), display-info (v0.5.9)
- **Logging:** log (v0.4), pretty_env_logger (v0.5)
- **Async:** tokio (v1.0)
- **Configurator:** serde (v1.0.228), serde_json (v1.0.149)

### Tested on:
- Windows 11
- Windows 11 (WSL Debian)

---
## **Q/A**:
###### (It's just easier to write answers directly)
##### **How you planned and approached the app:**
> **Backstory:**
> 	Personally, since I've met lots of architectural problems & Python limits, I lost my interest making heavy applications on a interpreter language. So I've decided start learning Rust. I learned some basic syntax, wanting to recreate UniCon on Rust using different ways to build it. When I only had project structure plan, when I've seen career affairs, I sent my CV to make an attempt to be hired. Then I've got an email from your company, and decided to get practice on selected project (SimpleChat), since I've already got some skills from UniCon.
>
> I've decided to build this app in more appropriate architectural way. I've taken my time figure out how should it work, if I want make it in my way:
> Why chat? Well, it's because I thought it would be pretty simple.
>
> **Core** → Personally, this is what I want to do in my pet project, so I decided to build some sort of simple core.
> **Bridge** → Well, I can understand how it works, but yet that was an AI idea, so I had lots of conversation regarding it and trying to make it work. Next time it might be easier, but I still haven't acknowledged all the steps.
> **Configurator** → Basically, I think this is what every application should have, if not a script or any specific program.
> **Network** → I'd worked on pet project UniCon, I have some experience in this field, so I decided just to use them again.
> **Logger** → Making debugging experience better.
> **GUI** → I got basic experience dealing with UI, so I thought there is no problem with that.

##### **Why you chose certain tools, frameworks, or libraries:**
> **Rust** → Memory safety, C++ like performance, compiling language, trend.
> **Slint** → CSS-like syntax, Native performance (compiles with code), cross-platform.
> **Logger** → Structured logging, Colored output (readability), easy to setup.
> **display-info** → That's the only easy way I found how to center window before it shows on the user's screen. Works great, but not on linux (WSL Debian). Simple to use & solves DPI ratio.
> **Tokio** → The only asynchronous library I heard of. Got recommended by AI.


##### **Flowcharts for key feature flows:**
1. Starting animation:
```
Start program -> Slint renders first frame -> Triggers event -> Changes bool value -> Slint triggers bool change -> Changes opacity to 1 -> keyword animate entercepts and inperpolates opacity from start value(0) to end value(1) within set amount of time(1500ms).
```
2. Mode choice (Server) (page index 2):
```
UI button triggers -> Button triggers Rust function -> UI_Manager sends command to core executor -> executor starts Network functions -> (2 outcomes)

1. Success -> Network sends command to executor -> executor sends responce to UI_manager -> UI manager switches pages
2. Failed -> Network sends command to executor -> executor sends responce to UI_manager -> UI manager switches to starting page, unlocks mode buttons, making ip & port entries red.
```

---
## Personal Thoughts:
> This project was made within 7 days of marathon. It does not satisfy all of my goals, including that I planned for MVP. I had to use AI to at least make it work. And I have not yet acknowledged all used libraries since It's my first experience making MVP project on Rust & Slint. I underestimated my ambitious features, which led me into this result.
>
> Nevertheless, this project represents fully working project, which satisfies all its MVP goals. Thus, I can say it is completed and will not be developed in the future, since it has completed its purpose.
