<h1 align="center">
    bubble
</h1>
<h4 align="center">
企业级开发框架.
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

# 依赖关系

```
                ┌───────────────┐
                │    bubble     │
                │    (主库)     │
                └───────┬───────┘
                        │ 使用 #[orm] 宏
                        ▼
                ┌───────────────┐
                │ bubble-macro  │
                │  (过程宏库)    │
                └───────┬───────┘
                        │ 依赖 DatabaseConnection trait
                        ▼
                ┌───────────────┐
                │   bubble-db   │
                │ (数据库抽象)   │
                └───────────────┘
```

## 依赖

```
bubble → bubble-macro → bubble-db
```

## 职责

```
bubble：面向用户的库，使用 #[orm] 宏
bubble-macro：生成 ORM 代码的过程宏
bubble-db：带有驱动程序的数据库抽象层
```
