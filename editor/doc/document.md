# Document

## Data Structure

## Expressed as HTML

## Schema

## Indexing
说明基于如下示例
```html
<root>  
  0   1 2 3 4    5
   <p> O n e </p>

  5            6   7 8 9 10     11    12             13
   <blockquote> <p> T w o  <img>  </p>  </blockquote>
</root>
```
### Offset
**通过偏移确定文档中的任意位置**

**偏移的计算规则**
1. 根节点内容最左端的偏移为  0
1. 进入或者离开一个非叶子节点会使偏移 +1
1. 每一个文字会使偏移 +1
1. 叶子节点会使 +1

**节点体积的计算规则**
1. 文本节点的体积是文字数目
1. 叶子节点的体积是 1
1. 非叶子节点的体积是所有子节点体积之和 +2

### Path
**Path 是指从根节点到任意位置的路径**
```rust
struct Path {
    path: Vec<Step>,       // 从根节点到指定位置的节点路径，不包含文本节点
    depth: usize,          // 指定位置在节点树中的深度，根节点的深度为 0，遇到非叶子子节点深度 +1，以此类推。文本节点不计算深度
    offset: usize,         // 指定位置的偏移表示
    parent_offset: usize,  // 指定位置到他的父节点内容最左端的偏移。当指定位置在文本节点内部时，父节点是指文本节点的父节点
    text_offset: usize,    // 当指定位置在一个文本节点内部时，该值表示指定位置相对文本节点左端的偏移。当指定位置不再文本节点内部时，该值为 0
}

struct Step {
    node:   Rc<dyn Node>,  // 节点的引用
    index: usize,          // 路径上下一个节点是当前节点的第几个子节点
    offset: usize,         // 路径上下一个节点相对于根节点内容最左端的偏移
}
```
Step 中 index 属性的边界情况：比如偏移为 11 的位置对应的 index 是多少 ？

规定空隙中的位置在计算 index 时，与空隙后面的的节点 index 相同，如果后面没有节点，index 值是父节点的子节点数量。（比较危险）

使用 Path 表示偏移 3 对应的位置
```rust
const PATH: Path = Path {
    path: vec![
        Step { node: root, index: 0, offset: 0 },
        Step { node: paragraph, index: 0, offset: 1 },
    ],
    depth: 1,
    offset: 3,
    parent_offset: 2,
    text_offset: 2,
};
```
使用 Path 表示偏移 5 对应的位置
```rust
const PATH: Path = Path {
    path: vec![
        Step { node: root, index: 1, offset: 5 },
    ],
    depth: 0,
    offset: 5,
    parent_offset: 5,
    text_offset: 0,
};
```
使用 Path 表示偏移 10 对应的位置
```rust
const PATH: Path = Path {
    path: vec![
        Step { node: root, index: 1, offset: 5 },
        Step { node: blockquote, index: 0, offset: 6 },
        Step { node: paragrpah, index: 1, offset: 10 },
    ],
    depth: 2,
    offset: 10,
    parent_offset: 3,
    text_offset: 0,
};
```
使用 Path 表示偏移 11 对应的位置
```rust
const PATH: Path = Path {
    path: vec![
        Step { node: root, index: 1, offset: 5 },
        Step { node: blockquote, index: 0, offset: 6 },
        Step { node: paragrpah, index: 2, offset: 11 },
    ],
    depth: 2,
    offset: 11,
    parent_offset: 4,
    text_offset: 0,
};
```

**Offset 转换为 Path**
重点是构造 path 数组。
从根节点开始
1. 确定偏移在当前节点的第几个子节点中，确定 index
1. 根据 index 计算出 offset

注意：文本节点不进入，直接计算 text offset
## Slice

## Changing 

### Replace

### Update