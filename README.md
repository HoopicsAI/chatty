# AI Novel Chatbot

这是一个基于 AI 生成的小说内容和角色数据的聊天机器人，实现和角色的实时对话.


## 启动应用

使用以下命令启动 Chatty 应用：
```
cargo run
```

## API 文档

```
curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "character_id": "char123",
    "message": "Hello, this is a test message!"
  }'
```