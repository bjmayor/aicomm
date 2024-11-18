# swiftide-pgvector

一个用于在 PostgreSQL 数据库中进行向量存储和检索的 Rust crate。该 crate 基于 [pgvector](https://github.com/pgvector/pgvector) 扩展,为 SwiftIDE 提供向量数据库支持。

## 功能特性

- 支持向量的存储和检索
- 支持余弦相似度、欧几里得距离等多种向量相似度计算方法
- 与 SQLx 和 PostgreSQL 深度集成
- 异步 API 支持

## 安装

将以下依赖添加到 `Cargo.toml`:
