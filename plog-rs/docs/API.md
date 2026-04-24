# Plog API 说明

本文档说明当前 Rust API 的基础约定，接口实现以 `crates/api/src/routes/` 下的代码为准。

## 基础约定

- 统一 JSON 响应结构来自 `plog-contracts`
- 成功与错误响应分别由后端统一封装
- 请求上下文会携带 request id 和时间戳
- 错误对外输出保持最小化，不暴露内部异常细节

## 路由入口

- `GET /`：服务标识
- `GET /health`：健康检查
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `GET /api/auth/me`
- `POST /api/auth/refresh`
- `GET /api/posts`
- `POST /api/posts`
- `GET /api/posts/:id`
- `PUT /api/posts/:id`
- `DELETE /api/posts/:id`

## 说明

- 以前文档中出现的 `/api/v1/...` 版本前缀，不再作为当前主参考。
- 如果接口实现与文档不一致，请优先以代码为准并同步更新本文档。
