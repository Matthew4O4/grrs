
<p align="center">
	👉 <a href="https://space.bilibili.com/373989874/channel/seriesdetail?sid=2227064">Java三小时入门</a> 👈
</p>
<p align="center">
     <a target="_blank" href='https://app.travis-ci.com/Matthew4O4/grrs'>
		<img src='https://img.shields.io/badge/license-Apache-blue' alt='star'/>
	</a>
	<a target="_blank" href=' https://github.com/Matthew4O4/grrs/star'>
		<img src="https://img.shields.io/github/stars/Matthew4O4?style=social" alt="github star"/>
	</a>
</p>
<br/>
<p align="center">
	<a href="">
	<img src="https://img.shields.io/badge/vx-matthewStory-orange"/></a>
</p>

-------------------------------------------------------------------------------

[**🌎English Documentation**](README-EN.md)

-------------------------------------------------------------------------------

## 📚简介
Grrs是一个小而不全的检索文档内容的工具”。

### 🎁Grrs名称的由来

Grrs = 教程就叫这个名字 + 我也懒得改。

### 🍺Grrs如何改变我们的 grep file 方式

可以从指定的文件中检索 存在目标值的行

例：

- 👴【以前】cat xxx.txt grep abc
- 👦【现在】grrs -- abc xxx.txt

Grrs的存在就是为了减少文档的搜索成本。

-------------------------------------------------------------------------------

## 🛠️包含组件
没有更多了

|  模块  | 介绍  |
|------|-----|
| gree | 主体  |


-------------------------------------------------------------------------------

## 📝文档

[📘Rust中文文档](https://rustwiki.org/)

[📙参考教程](http://llever.com/cli-wg-zh/tutorial/packaging.zh.html)

-------------------------------------------------------------------------------

## 🪙支持Grrs

### 💳捐赠

如果你钱没处花.

[Gitee上捐赠](https://github.com/Matthew4O4/grrs)


-------------------------------------------------------------------------------

## 📦安装

### 🍊下载响应二进制文件使用

配置环境
```sh
export PATH=/{{下载二进制文件grss的地址}}:/bin:PATH
```


> 🔔️注意
> 要选择与系统相同的版本

### 🚽编译安装

访问Grrs的Gitee主页：[https://github.com/Matthew4O4/grrs](https://github.com/Matthew4O4/grrs) 下载整个项目源码（main或dev分支都可）然后进入Grrs项目目录执行：

```sh
.grrs -- 目标值 文件名
```

-------------------------------------------------------------------------------

## 🏗️添砖加瓦

### 🎋分支说明

Grrs的源码分为两个分支，功能如下：

| 分支  | 作用                                                         |
|-----|--------------------------------------------------------------|
| main | 主分支，release版本使用的分支，不接收任何pr或修改 |
| dev | 开发分支，默认为下个版本的SNAPSHOT版本，接受修改或pr                |

### 🐞提供bug反馈或建议

提交问题反馈请说明正在使用的Rust版本呢。

- [Github issue](https://github.com/Matthew4O4/grrs/issues)


### 🧬贡献代码的步骤

1. 在Gitee或者Github上fork项目到自己的repo
2. 把fork过去的项目也就是你的项目clone到你的本地
3. 修改代码（记得一定要修改dev分支）
4. commit后push到自己的库（dev分支）
5. 登录Gitee或Github在你首页可以看到一个 pull request 按钮，点击它，填写一些说明信息，然后提交即可。
6. 等待维护者合并

### 📐PR遵照的原则

Grrs欢迎任何人为Grrs添砖加瓦，贡献代码，不过维护者是一个强迫症患者，为了照顾病人，需要提交的pr（pull request）符合一些规范，规范如下：
1. 注释完备，尤其每个新增的方法应按照Java文档规范标明方法说明、参数说明、返回值说明等信息，必要时请添加单元测试，如果愿意，也可以加上你的大名。
2. Grrs的缩进按照IDEA
3. 新加的方法不要使用第三方库的方法，Grrs遵循无依赖原则（除非在extra模块中加方法工具）。
4. 请pull request到`dev`分支。Grrs在5.x版本后使用了新的分支：`main`是主分支，表示已经发布中央库的版本，这个分支不允许pr，也不允许修改。
5. 我们如果关闭了你的issue或pr，请不要诧异，这是我们保持问题处理整洁的一种方式，你依旧可以继续讨论，当有讨论结果时我们会重新打开。
