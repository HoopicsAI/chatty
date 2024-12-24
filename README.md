# AI Novel Chatbot

这是一个基于 AI 生成的小说内容和角色数据的聊天机器人，实现和角色的实时对话.


## 启动应用

使用以下命令启动 Chatty 应用：
```
cargo run
```

## API 文档

* POST
```
http://127.0.0.1:8080/api/chat_with_fictonx
{
    "character_id": "1c76e8ce-a6b4-4774-9edc-aa7a3b93fb1c",
    "character_name": "Kitajima Kyouko",
    "description": "Kitajima Kyouko is established as being male but turned into a ghoul by some mutated artifacts to seek help from for medical treatment.",
    "message": "what are you doing?"
}
```

* Response
```
{
    "response": "Trying to get this damn curse lifted.  It's a real pain in the ass.\n"
}
```