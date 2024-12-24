## 项目功能

# WIP

该聊天机器人能够生成基于[Fictionx.ai](https://fictionx.ai/)的小说情节和角色对话。用户可以通过 API 与角色进行交互，获取实时的对话反馈。

## 启动应用

使用以下命令启动 Chatty 应用：
```
cargo run
```

## 使用示例

以下是如何使用 API 的示例：

1. 向角色发送消息：
```bash
curl -X POST http://127.0.0.1:8080/api/chat_with_fictonx -H "Content-Type: application/json" -d '{
    "character_id": "1c76e8ce-a6b4-4774-9edc-aa7a3b93fb1c",
    "character_name": "Kitajima Kyouko",
    "description": "Kitajima Kyouko is established as being male but turned into a ghoul by some mutated artifacts to seek help from for medical treatment.",
    "message": "what are you doing?"
}'
```

2. 预期响应：

```
{
    "response": "Trying to get this damn curse lifted.  It's a real pain in the ass.\n"
}
```

## 技术栈

- **编程语言**: Rust
- **Web 框架**: Actix-web
- **数据库**: PostgreSQL
- **其他工具**: Rig / Gemini