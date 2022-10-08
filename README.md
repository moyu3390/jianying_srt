# jianying_srt

## Introduction
剪映是字节推出的一款视频剪辑软件，它可以利用AI自动识别字幕，十分强大。但是剪映并没有提供字幕的导出功能，如果不做处理的话只能在剪映软件内部使用。好在剪映同时也生成了json格式的字幕文件，利用这个文件我们就可以将字幕转换为常见的srt格式。

## Usage
首先找到剪映默认的项目文件夹，一般在安装目录的同级，例如我将剪映装在`D:\Program Files\JianyingPro`，那么项目文件就在`D:\Program Files\JianyingPro\项目日期`，对应的项目文件夹下的`draft_content.json`就是字幕文件。
启动`jianying_srt.exe`，将该文件拖进去（或输入该文件的绝对路径），程序自动进行转换，按照提示指定生成的srt文件的输出路径即可。

## Example
项目提供的draft_content.json为测试文件。
```bash
jianying_srt.exe
输入json文件绝对路径或拖拽到此:
draft_content.json
转换完成. 生成srt文件到:
draft_content.srt
输出完成. 按任意键退出.

```

## Methods
程序使用Rust语言开发。使用了json和regex两个标准库。

## Results
程序运行完毕后会生成一个srt文件，可以将其再导入其他视频剪辑软件（Pr等）进行二次处理。

## Conclusion
之前胡萝卜周推荐过一个程序，也是剪映字幕转换的，还有字幕修改界面，但是后来剪映改版了，在字幕块周围加了一些html标签，导致识别出错。我没有找到这个程序对应的更新地址，索性也就不用了，自己写了一个，顺便学习Rust。
希望大家提供反馈和建议。
