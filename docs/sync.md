# sync

sync用于在服务器间同步消息，客户端在执行`MessagePush`或者`GetMissingMessage`操作时，由`Home Server`向目标服务器发送。
目标服务器收到请求后也可向相关服务器发送，最后由相关服务器向Origin服务器回传。
