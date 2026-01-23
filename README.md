<h1 align="center">
    bubble
</h1>
<h4 align="center">
Enterprise-level development framework.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/bubble/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-Apache2.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
    <a href="https://crates.io/crates/bubble">
<img src="https://img.shields.io/badge/crates-bubble-20B2AA.svg?style=flat&labelColor=0F1F2D&color=FFD700&logo=rust&logoColor=FFD700">
</a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

# Dependencies

```
                ┌───────────────┐
                │    bubble     │
                │  (Main Lib)   │
                └───────┬───────┘
                        │ Uses #[orm] macro
                        ▼
                ┌───────────────┐
                │ bubble-macro  │
                │ (Proc Macro)  │
                └───────┬───────┘
                        │ Depends on DatabaseConnection trait
                        ▼
                ┌───────────────┐
                │   bubble-db   │
                │ (DB Abstraction)│
                └───────────────┘
```

## Dependency

```
bubble → bubble-macro → bubble-db
```

## Responsibilities:

```
1. bubble: User-facing library with #[orm] macro usage
2. bubble-macro: Procedural macro that generates ORM code
3. bubble-db: Database abstraction layer with drivers
```
