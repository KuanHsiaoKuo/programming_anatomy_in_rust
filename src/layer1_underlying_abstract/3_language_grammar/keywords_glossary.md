# 词法关键字扫盲

## 1. 关键字

### 严格关键字

```rust
as / break / const / continue / crate / if / else / struct / enum / true / false / fn / for / in / let / loop / impl / mod / match / move mut / pub / ref / return / self / Self / static / super /trait / type / unsafe /use / where / while / async /await/dyn/main
```

### 弱关键字

```rust
abstract / become / box / do / final / macro / override / priv / typeof / unsized / virtual / yield / try
```

### 保留字

- 2018 Edition:union,'static
- 2015 Edition:dyn

> 被保留的关键字不代表将来一定会使用

## 2. 标识符

## 3. 注释

> //!, /*!, //!!, /*!!, /**...*/, //, ////, /***...*/

## 4. 空白: \n、\t、tab

> 任何形式的空白字符在RuSt中只用于分隔标记，没有语义意义。

## 5. 词条

1. 语言项(item)
2. 块(block)
3. 语句（Stmt)
4. 表达式（Expr)
5. 模式（Pattern)
6. 关键字（Keyword)
7. 标识符（Ident)
8. 字面量(Literal)
9. 生命周期(Lifetime)
10. 可见性(Vis)
11. 标点符号（Punctuation)
12. 分隔符（delimiter)
13. 词条树(Token Tree)
14. 属性（Attribute)

## 路径: ::, ::<>



## 参考资源

1. [词法结构](https://time.geekbang.org/course/detail/100060601-286522)
