# Liki4TeloxideBot

## Config

改 .env 文件

```
# ERROR/WARN/INFO/DEBUG/TRACE
RUST_LOG=DEBUG

# 86400s=1d
MEDIA_GROUP_MAPPING_TIMEOUT=86400

TELOXIDE_TOKEN=<BOT_TOKEN>
# Uncomment TELOXIDE_PROXY to use proxy
# TELOXIDE_PROXY=http://127.0.0.1:7890/

# Uncomment MEME_API_URL to use Python Version API
# MEME_API_URL=http://meme.generator.com/
MEME_MEDIA_GROUP_HANDLE_TIMEOUT=5
```

TELOXIDE_PROXY 开了 bot 就会走代理

MEME_API_URL 开了 generator 就会走 python 版的 api

别的没了

## Usage

### /meme list

![image](https://github.com/user-attachments/assets/20fc90a7-6545-4543-80af-927430ad2e79)

### /meme search xxx

![image](https://github.com/user-attachments/assets/05742cd4-9638-4d83-a91c-0ee756fc1a01)

是个 keyword/key -> key 的映射，但因为后来其他命令也会从映射找，没什么用

### /meme info xxx

![image](https://github.com/user-attachments/assets/4854b95d-31ba-435f-a8f8-68cb240c256e)

会发出来 preview 和参数，但我懒得处理参数

### /meme random

![image](https://github.com/user-attachments/assets/c1d1d4e0-62f9-45a3-9913-666ac39deb2e)

会随机一个入参条件符合的 meme，这里是用的图片数组里最后加进来的发送人头像了


### /meme generate

![image](https://github.com/user-attachments/assets/67648b69-9eb1-4887-85b8-dfebafdef1da)

用入参生成一张 meme


### 入参方式

random 和 generate 的入参逻辑都是一样的

文字就是跟在命令后面，最后再拼上发送人的 first_name，但要注意的是 at 的文字也会被放进来，所以发的时候按照先发文字再 at 人的顺序来

![image](https://github.com/user-attachments/assets/a19d7abe-112a-417b-935d-bf44450c28e3)


图片就是 直接发的图片+at的人的头像+回复的消息的图片+发送人的头像，按顺序拼成数组，反正大概是那么个意思

![image](https://github.com/user-attachments/assets/54f3b0ac-76f7-476b-9e4e-36bd919e991c)

![image](https://github.com/user-attachments/assets/6a4ba2b3-ec11-4078-86f5-caf7eceb6862)

![image](https://github.com/user-attachments/assets/51571764-d698-4519-b28a-ad12dd5b15e1)



然后参数最后会根据 meme 对应的最大入参量截断，比如你给一个 2 个入参的 meme 发 3 个参数，最后生效的也只是前 2 个

## Generator

Rust 版在：https://github.com/MemeCrafters/meme-generator-rs
Python 版在：https://github.com/MemeCrafters/meme-generator


