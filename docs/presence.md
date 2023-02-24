# State Management and Presence

The presence API allows you to track the presence of users in a room. This is useful for showing which users are currently active in a room.

打个比方来讲，如果你在一个聊天室里，你可以看到谁在线，谁离开了，谁正在输入等。

由于底层消息传输协议是 [Orkas](https://github.com/limit-lab/orkas)，我们将使用 Orkas 内置的 CRDT 来实现这个功能。

在收到和发送消息时，我们会首先验证消息的正确性，如更改Room的名称，更改Room的权限等，这些都是需要验证的，再验证通过后，我们会将消息广播到整个网络，这样所有的节点都会收到这条消息。

当收到消息时，我们先会验证消息的正确性，如果验证通过，我们会将消息存储到本地。

所以可能会出现在同一个room中，两个不同的服务器的状态不一致的情况，这是可以接受的。