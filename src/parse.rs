use pulldown_cmark::{html, Options, Parser};

pub fn parse() -> String {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION,
    );

    // let markdown_input = "Hello, world!";
    let markdown_input = r#"
# 見出し1

## 見出し2

### 見出し3

#### 見出し4

markdownサンプル文章です。ここは地の文です。

markdownでは、箇条書きは*や-などの記号を文頭に置くことで記述します。箇条書きの階層は行頭スペース4つを足します。

- これはひとつめの箇条書き
- ふたつめの箇条書き
    - 一つ階層が深い箇条書き
- みっつめの箇条書き

### コード

3つのバッククォート記号でくくることで、コード例を示します

```
[ozuma@vpscon ~]$ cp a
cp: missing destination file operand after `a'
Try `cp --help' for more information.
```

markdown形式については、Wikipediaなども参照ください
[](http://ja.wikipedia.org/wiki/Markdown)
        "#;

    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("{}", html_output);
    return html_output;
}
