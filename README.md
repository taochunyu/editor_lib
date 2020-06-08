# 渲染协议

## 原子结构

**矩形**

主要属性：宽高
次要属性：样式

**文本**

主要属性：文本内容
次要属性：文本样式

## 原子结构表达

名称：

## 基础布局
absolute
relative

## 原子操作

**插入**

append id payload

insert_before parent_id id payload

**删除**

remove id

**更新**

update id payload

**移动**

move from_id (append or insert_before) to_id 

## 需求点

并行，多图层

合并，减少更新次数

简单，实现优先，最重要

## 思考点
