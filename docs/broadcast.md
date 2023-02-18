# broadcast

event 会通过 room 作为 topic 使用 orkas 的 pubsub 机制进行广播。

每次广播都会携带一个 server id，用于标识这个广播是来自哪个 server 的。

## 最长链原则
除了第一个创建群组的 event，其他的 event 都会携带一个 prev_event_id，用于标识这个 event 是在哪个 event 之后发生的。

这样就可以通过 prev_event_id 来构建一个链，这个链就是这个群组的最长链。

当两个消息的 prev_event_id 相同时，就会产生分叉，这时候要遵循最长链原则，选择最长的链作为最终的链，然后将第二条链放置在第一条链的后面，并且把这个 event id 和 prev_event_id 广播出去。

服务器收到变动广播之后，需要将这个变动分发给所有的 websocket 连接，让他们更新。

## 未知的 prev_event_id
当服务器收到一个 event 时，如果这个 event 的 prev_event_id 未知，那么服务器会发起一个对来源服务器的请求，请求这个 event 的 prev_event_id，如果这个 event 的 prev_event_id 也未知，那么服务器会继续发起请求，直到找到一个已知的 prev_event_id 为止。

