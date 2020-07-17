# Record

## content editable 性能与节点数目关系

实验结果

contenteditable 外部大量节点，内部少量节点
1. contenteditable 在顶部时，性能较好，底部时性能较差，差距不大
2. 脱离文档流后性能显著提升
3. 节点数目少可以显著提高性能

input 比 contenteditable 有更好的性能

