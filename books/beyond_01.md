# What Color is Your Function?[^1]

# 你的函数什么颜色
[TOC]

I don’t know about you, but nothing gets me going in the morning quite like a good old fashioned programming language rant. It stirs the blood to see someone skewer one of those `“blub”` languages the plebians use, muddling through their day with it between furtive visits to StackOverflow.

我不知道你是怎么想的，但没有什么能比得上一个老式的编程语言咆哮更能让我在早上振作起来。看到有人对那些码农使用的 "blub【令人大哭】 "语言之一进行抨击，并在访问StackOverflow的间隙用它来糊弄自己的一天，这让我热血沸腾。

(Meanwhile, you and I, only use the most enlightened of languages. Chisel-sharp tools designed for the manicured hands of expert craftspersons such as ourselves.)

(这也就意味着，你和我，只使用最棒的语言。 专为像我们这样的攻城狮设计的凿子般锋利的工具。)

Of course, as the author of said screed, I run a risk. The language I mock could be one you like! Without realizing it, I could have let the rabble into my blog, pitchforks and torches at the ready, and my fool-hardy pamphlet could draw their ire!

当然，作为上述尖叫的作者，我也有风险。我嘲讽的语言可能是你喜欢的语言! 在不知不觉中，我可能已经让乌合之众进入我的博客，干柴和烈火都准备好了，而我的愚蠢的小册子可能会引起他们的愤怒!

To protect myself from the heat of those flames, and to avoid offending your possibly delicate sensibilities, instead, I’ll rant about a language I just made up. A strawman whose sole purpose is to be set aflame.

为了保护我自己不受这些愤青的影响 也为了避免冒犯你们可能存在的敏感神经 我将对我刚刚编造的一种语言进行咆哮。作为一个靶子一样的稻草人，其存在的唯一目的就是被点燃。

I know, this seems pointless right? Trust me, by the end, we’ll see whose face (or faces!) have been painted on his straw noggin.

我知道，这看起来毫无意义，对吧？相信我，到最后，我们会看到谁的脸（或几张脸！）被做成了那个稻草人的头。

## A new language
## 一门新的语言

Learning an entire new (crappy) language just for a blog post is a tall order, so let’s say it’s mostly similar to one you and I already know. We’ll say it has syntax sorta like JS. Curly braces and semicolons. if, while, etc. The `lingua franca` of the programming grotto.

仅仅为了一篇博客文章而学习一门全新的(蹩脚的)语言是一个高难度的任务，这个和我们已知的大多数编程语言很相似。我们说它的语法有点像JS。包含有大括号和分号.if，while等。这个就是远古时代的编程语言的通用语。

I’m picking JS not because that’s what this post is about. It’s just that it’s the language you, statistical representation of the average reader, are most likely to be able grok. Voilà:

我用JS举例，并不是这篇文章的内容是讲JS的。只是因为根据数据统计，最有可能被大家熟知的编程语言是它，就这样吧！比如下例：

```js
function thisIsAFunction() {
  return "It's awesome";
}
```
Because our strawman is a modern (shitty) language, we also have first-class functions. So you can make something like like:

对于我们这些稻草人来说，JS是一种现代(`低劣`)语言，它拥有一流的函数。所以我们可以写出这样的东西：
```js
// Return a list containing all of the elements in collection
// that match predicate.
function filter(collection, predicate) {
  var result = [];
  for (var i = 0; i < collection.length; i++) {
    if (predicate(collection[i])) result.push(collection[i]);
  }
  return result;
}
```

This is one of those higher-order functions, and, like the name implies, they are classy as all get out and super useful. You’re probably used to them for mucking around with collections, but once you internalize the concept, you start using them damn near everywhere.

这是那些高阶函数之一，并且顾名思义，它们都是一流的，因为它们全部都非常有用。 您可能已经习惯了他们混搭收藏，但是一旦概念化后，你就会开始把它们用到几乎所有的地方。

Maybe in your testing framework:

也许在你的测试框架中是这样写的：
```js
describe("An apple", function() {
  it("ain't no orange", function() {
    expect("Apple").not.toBe("Orange");
  });
});
```
Or when you need to parse some data:

或者，当您需要解析一些数据时：

```js
tokens.match(Token.LEFT_BRACKET, function(token) {
  // Parse a list literal...
  tokens.consume(Token.RIGHT_BRACKET);
});
```
So you go to town and write all sorts of awesome reusable libraries and applications passing around functions, calling functions, returning functions. Functapalooza.

所以你就去写各种厉害的可重用的库和应用程序，这些函数和应用程序会传递函数，调用函数，返回函数。Functapalooza.

## What color is your function?
## 你的函数什么颜色

Except wait. Here’s where our language gets screwy. It has this one peculiar feature:

除了等待,这就是我们的语言变得很糟糕的地方。它有一个奇特的功能:

### 1. Every function has a color.
### 1. 每一个函数都是有颜色的。
Each function—anonymous callback or regular named one—is either red or blue. Since my blog’s code highlighter can’t handle actual color, we’ll say the syntax is like:

每个函数（匿名回调或常规命名的函数）不是红色就是蓝色。由于我的博客的代码高亮器不能处理实际的颜色，我们就把语法展示成这样：

```java
blue•function doSomethingAzure() {
  // This is a blue function...
}

red•function doSomethingCarnelian() {
  // This is a red function...
}
```

There are no colorless functions in the language. Want to make a function? Gotta pick a color. Them’s the rules. And, actually, there are a couple more rules you have to follow too:

语言中没有无色函数。想做一个函数？必须选择一种颜色。这就是规则。而且，实际上，还有一些规则你也要遵守。

### 2. The way you call a function depends on its color.
### 2. 你调用函数的方式取决于它的颜色。

Imagine a “blue call” syntax and a “red call” syntax. Something like:

想想一下一个是`blue call`语法，一个是`red call`语法。像这样：

```java
doSomethingAzure(...)•blue;
doSomethingCarnelian()•red;
```

When calling a function, you need to use the call that corresponds to its color. If you get it wrong—call a red function with •blue after the parentheses or vice versa—it does something bad. Dredge up some long-forgotten nightmare from your childhood like a clown with snakes for arms hiding under your bed. That jumps out of your monitor and sucks out your vitreous humour.

当调用一个函数时，你需要使用与其颜色相对应的调用。如果你弄错了--调用一个红色的函数时，在括号后面却标成了`•blue`，反之亦然。现在再现一个你童年中一些被遗忘已久的噩梦，就像一个以蛇为手臂的小丑藏在你的床下的噩梦。它会从你的显示器里跳出来，吸干你的幽默。

Annoying rule, right? Oh, and one more:

这是一个讨厌的规则，对吧？哦，还有一个：

### 3. You can only call a red function from within another red function.
### 3. 你只能从红色函数里面调用红色函数。【换言之，同颜色函数才可以被调用。】

You can call a blue function from with a red one. This is kosher:

你可以在一个红色函数里面调用一个蓝色函数。这是很正常的：

```java
red•function doSomethingCarnelian() {
  doSomethingAzure()•blue;
}
```
But you can’t go the other way. If you try to do this:

但是你不能做相反的操作。 如果您尝试这样做：
```java
blue•function doSomethingAzure() {
  doSomethingCarnelian()•red;
}
```
Well, you’re gonna get a visit from old Spidermouth the Night Clown.

噢，就像你接待一位叫做`Spidermouth`的夜场小丑的拜访。

>【意思就是小丑可以拜访你，但是你不能去拜访小丑。这里说的意思是：红色函数里面可以调用蓝色函数，蓝色函数里面不可以调用红色函数。】

This makes writing higher-order functions like our filter() example trickier. We have to pick a color for it and that affects the colors of the functions we’re allowed to pass to it. The obvious solution is to make filter() red. That way, it can take either red or blue functions and call them. But then we run into the next itchy spot in the hairshirt that is this language:

这使得编写像`filter（）`示例这样的高阶函数变得更加棘手。我们必须为其选择一种颜色，这会影响允许我们传递给它的功能的颜色。 显而易见的解决方案是将`filter（）`设为红色。这样，它可以接受红色或蓝色函数并调用它们。但是马上，我们就遇到了语言中有令我们不舒服的地方，比如：

### 4. Red functions are more painful to call.
### 4.红色函数调用起来更痛苦。
For now, I won’t precisely define “painful”, but just imagine that the programmer has to jump through some kind of annoying hoops every time they call a red function. Maybe it’s really verbose, or maybe you can’t do it inside certain kinds of statements. Maybe you can only call them on line numbers that are prime.

现在，我不会精确地定义“痛苦”，但只要想象一下，程序员每次调用红色函数时都要跳过一些恼人的障碍。也许它很冗长，或者您无法在某些类型的语句中使用它。也许你只能在质数的行号上调用它们。

What matters is that, if you decide to make a function red, everyone using your API will want to spit in your coffee and/or deposit some even less savory fluids in it.

后果就是，如果您决定将某个函数设为红色，那么每个使用您的这个API的人都会想在您的咖啡中吐痰，或者在咖啡中放置一些更不可口的液体。

The obvious solution then is to never use red functions. Just make everything blue and you’re back to the sane world where all functions have the same color, which is equivalent to them all having no color, which is equivalent to our language not being entirely stupid.

那么显而易见的解决方案就是永远不要使用红色函数。只要把所有的东西都变成蓝色，你就回到了正常的世界，所有的函数都有相同的颜色，这相当于它们都没有颜色，这相当于我们的语言并不完全是愚蠢的。

Alas, the sadistic language designers—and we all know all programming language designers are sadists, don’t we?—jabbed one final thorn in our side:

唉，虐待狂的语言设计者--我们都知道所有的编程语言设计者都是虐待狂，不是吗?--在我们身上刺了最后一根刺：


### 5. Some core library functions are red.
### 5. 一些核心库函数是红色的。


There are some functions built in to the platform, functions that we need to use, that we are unable to write ourselves, that only come in red. At this point, a reasonable person might think the language hates us.

平台内置的一些我们需要使用的功能（这些功能我们自己还无法编写），只能用红色来表示。在这一点上，一个有理智的人可能会认为这种语言讨厌我们。

***It’s functional programming’s fault!***

***这个是函数式编程的错误！***


You might be thinking that the problem here is we’re trying to use higher-order functions. If we just stop flouncing around in all of that functional frippery and write normal blue collar first-order functions like God intended, we’d spare ourselves all the heartache.

你可能会想，这里的问题是我们在尝试使用高阶函数。如果我们不要再在那些功能上的花哨的东西上打转，而是像上帝希望的那样，写出正常的蓝领一阶函数，我们就可以省去所有的心痛了。

If we only call blue functions, make our function blue. Otherwise, make it red. As long as we never make functions that accept functions, we don’t have to worry about trying to be “polymorphic over function color” (polychromatic?) or any nonsense like that.

如果我们只调用蓝色的函数，就把我们的函数做成蓝色。否则，就把它做成红色的。只要我们永远不做接受函数的函数，我们就不用担心要想 "多色于函数颜色"（多色？）之类的废话。

But, alas, higher order functions are just one example. This problem is pervasive any time we want to break our program down into separate functions that get reused.

但是，可惜的是，高阶函数只是一个例子。 每当我们想将程序分解为可重用的独立功能时，这个问题就无处不在。

For example, let’s say we have a nice little blob of code that, I don’t know, implements Dijkstra’s algorithm over a graph representing how much your social network are crushing on each other. (I spent way too long trying to decide what such a result would even represent. Transitive undesirability?)

举例说明：假设我们有一个很好的小代码块，假设它是在一个图上实现了Dijkstra的算法，表示你的社交网络相互挤压的程度。（我花了太长时间试图决定这样的结果会代表什么。不受欢迎的传递性【Transitive undesirability】？）

Later, you end up needing to use this same blob of code somewhere else. You do the natural thing and hoist it out into a separate function. You call it from the old place and your new code that uses it. But what color should it be? Obviously, you’ll make it blue if you can, but what if it uses one of those nasty red-only core library functions?

稍后，您将需要在其他地方使用相同的代码块。你很自然的，把这段代码提升到一个单独的功能中。你从新旧功能都可以调用它。那么它应该是什么颜色呢？很明显，如果可以的话，你会把它变成蓝色，但是如果它使用了那些讨厌的纯红色核心库函数呢？

What if the new place you want to call it is blue? You’ll have to turn it red. Then you’ll have to turn the function that calls it red. Ugh. No matter what, you’ll have to think about color constantly. It will be the sand in your swimsuit on the beach vacation of development.

如果您要称呼它的新地方是蓝色怎么办？ 您必须将其变成红色。 然后，您必须将调用它的函数变成红色。 啊。 无论如何，您都必须不断考虑色彩。即便是你在海边度假也不得安宁。

## A colorful allegory
## 丰富多彩的寓言
Of course, I’m not really talking about color here, am I? It’s an allegory, a literary trick. The Sneetches isn’t about stars on bellies, it’s about race. By now, you may have an inkling of what color actually represents. If not, here’s the big reveal:

当然，我不是在这里真正谈论颜色，是吗？ 这是一种寓言，一种文学技巧。 `Sneetches`并不是讲关于肚子上的星星，而是再说种族问题。 到现在为止，您可能已经对实际代表的颜色有所了解。 如果没有的话，这是一个很大的启示：

***Red functions are asynchronous ones.*** 

***红色函数是异步函数。***
If you’re programming in JavaScript on Node.js, everytime you define a function that “returns” a value by invoking a callback, you just made a red function. Look back at that list of rules and see how my metaphor stacks up:

如果你在Node.js上用JavaScript编程，每当你定义一个函数,可以通过调用回调来 "返回 "值时，你只是做了一个红色函数。回过头来看看那个规则列表，看看我的比喻是如何形成的：

- 1.Synchronous functions return values, async ones do not and instead invoke callbacks.

- 1.同步函数返回值，异步函数不返回值，而是调用回调。

- 2.Synchronous functions give their result as a return value, async functions give it by invoking a callback you pass to it.

- 2.同步函数以返回值的形式给出结果，异步函数通过调用你传递给它的回调来给出结果。

- 3.You can’t call an async function from a synchronous one because you won’t be able to determine the result until the async one completes later.

- 3.您无法从同步功能中调用异步功能，因为要等到异步功能完成后才能确定结果。

- 4.Async functions don’t compose in expressions because of the callbacks, have different error-handling, and can’t be used with try/catch or inside a lot of other control flow statements.

- 4.异步函数由于回调而不在表达式中组合，错误处理不同，不能与try/catch或其他许多控制流语句一起使用。

- 5.Node’s whole shtick is that the core libs are all asynchronous. (Though they did dial that back and start adding ___Sync() versions of a lot of things.)

_ 5.Node的整个语法特点是核心库(lib)都是异步的。(尽管他们确实回调了，并开始添加很多东西的__Sync()版本)。

When people talk about “callback hell” they’re talking about how annoying it is to have red functions in their language. When they create 4089 libraries for doing asynchronous programming, they’re trying to cope at the library level with a problem that the language foisted onto them.

当人们谈论 "回调地狱 "时，他们谈论的是在他们的语言中出现红色函数是多么的烦人。当他们为做异步编程而创建4089库时，他们是想在库的层面上解决语言强加给他们的问题。

## I promise the future is better
## 我承诺未来会更好
People in the Node community have realized that callbacks are a pain for a long time, and have looked around for solutions. One technique that gets a bunch of people excited is promises, which you may also know by their rapper name “futures”.

Node社区里的人早就意识到回调是一件很痛苦的事情，于是四处寻找解决方案。有一种技术让一群人兴奋不已，那就是`promises`，你可能还知道他们的名字—— "futures"。

> 在这里我借用一些Rust的概念解释一下什么是Future。Rust中Future的定义如下，一个Future可以理解为一段供将来调度执行的代码。我们为什么需要异步呢，异步相比同步高效在哪里呢？就是在异步环境下，当前调用就绪时则执行，没有就绪时则不等待任务就绪，而是返回一个Future，等待将来任务就绪时再调度执行。当然，这里返回Future时关键的是要声明事件什么时候就绪，就绪后怎么唤醒这个任务到调度器去调度执行。

These are sort of a jacked up wrapper around a callback and an error handler. If you think of passing a callback and errorback to a function as a concept, a promise is basically a reification of that idea. It’s a first-class object that represents an asynchronous operation.

这些都是回调和错误处理程序的升级包装。如果你把向函数传递回调和错误处理作为一个概念，那么`promise`基本上就是这个概念的具化。它是一个代表异步操作的一级对象。

I just jammed a bunch of fancy PL language in that paragraph so it probably sounds like a sweet deal, but it’s basically snake oil. Promises do make async code a little easier to write. They compose a bit better, so rule #4 isn’t quite so onerous.

我只是在这段程序中叫了一堆花哨的PL语言，所以它可能听起来像一个甜蜜的合约，它就像蛇油一样润滑。`promises`确实使异步代码更容易编写。 它们的合成效果更好一些，所以第4条规则并不那么繁琐。

But, honestly, it’s like the difference between being punched in the gut versus punched in the privates. Less painful, yes, but I don’t think anyone should really get thrilled about the value proposition.

但是，说实话，这就像被人打肚子和打私处的区别。是的，痛苦少了一些，但是我想所有人都会为了这个建议高兴的颤抖。

You still can’t use them with exception handling or other control flow statements. You still can’t call a function that returns a future from synchronous code. (Well, you can, but if you do, the person who later maintains your code will invent a time machine, travel back in time to the moment that you did this and stab you in the face with a #2 pencil.)

您仍然不能将它们与异常处理或其他控制流语句一起使用。 您仍然无法调用从同步代码返回`future`的函数。(嗯，你可以，但如果你这样做，以后维护你的代码的人会发明一个时间机器，回到你这样做的那一刻，然后用2号铅笔扎你的脸。)

You’ve still divided your entire world into asynchronous and synchronous halves and all of the misery that entails. So, even if your language features promises or futures, its face looks an awful lot like the one on my strawman.

你还是把你的整个世界分成了异步和同步的两半，以及由此带来的所有痛苦。所以，即使你的语言以承诺或`future`为特色，它的面孔看起来还是像一张可怕的稻草人的脸。

(Yes, that means even Dart, the language I work on. That’s why I’m so excited some of the team are experimenting with other concurrency models.)

（是的，这就是我正在使用的语言——Dart就是这样做的，。这就是为什么我对某些团队正在尝试其他并发模型感到非常兴奋的原因。）

> Dart下的`Future`类似于ES6下新增的`Promise`，也是为了解决异步回调带来的各种问题。

## I’m awaiting a solution
## 我正在等待问题解决
C# programmers are probably feeling pretty smug right now (a condition they’ve increasingly fallen prey to as Hejlsberg and company have piled sweet feature after sweet feature into the language). In C#, you can use the await keyword to invoke an asynchronous function.

C#程序员们现在可能正感到沾沾自喜（随着Hejlsberg和微软公司将一个又一个甜美的功能添加到语言中，他们越来越得意）。在C＃中，可以使用`await`关键字来调用异步函数。

This lets you make asynchronous calls just as easily as you can synchronous ones, with the tiny addition of a cute little keyword. You can nest await calls in expressions, use them in exception handling code, stuff them inside control flow. Go nuts. Make it rain await calls like a they’re dollars in the advance you got for your new rap album.

这让你可以像同步调用一样轻松地进行异步调用，只是增加了一个可爱的小关键字。你可以在表达式中嵌套等待调用，在异常处理代码中使用它们，在控制流中塞入它们。发疯吧。***就像为了购买你的说唱专辑的预付款像雨点一样落下。[为了翻译这句话我也是拼了，我也是go nuts了]***

Async-await is nice, which is why we’re adding it to Dart. It makes it a lot easier to write asynchronous code. You know a “but” is coming. It is. But… you still have divided the world in two. Those async functions are easier to write, but they’re still async functions.

异步等待(`Async-await` )非常好，这就是为什么我们将其添加到Dart中。 这使得编写异步代码变得容易得多。 您知道一个“但是”就要来了——但是…你还是把世界一分为二。这些异步函数更容易编写，但它们仍然是异步函数。

您仍然有两种颜色。 异步等待解决了烦人的规则4：它们使红色函数的调用不比蓝色函数差。 但是所有其他规则仍然存在：

- 1.Synchronous functions return values, async ones return Task<T> (or Future<T> in Dart) wrappers around the value.

- 1.同步函数返回值，异步函数返回值周围的`Task<T>`（或Dart中的`Future<T>`）。

- 2.Sync functions are just called, async ones need an await.

- 2.同步函数只是被调用，异步函数需要一个`await`。

- 3.If you call an async function you’ve got this wrapper object when you actually want the T. You can’t unwrap it unless you make your function async and await it. (But see below.)

- 3.如果你调用一个异步函数，当你真正想要`T`的时候，你已经得到了这个包装对象，你不能解开它，除非您让您的函数异步并等待它。(但请看下文)。

- 4.Aside from a liberal garnish of await, we did at least fix this.

- 4.除了“等待”的自由装饰外，我们至少解决了这个问题。

- 5.C#‘s core library is actually older than async so I guess they never had this problem.

- 5.C#的核心库其实比async还要老，所以我猜他们从来没有出现过这个问题。

It is better. I will take async-await over bare callbacks or futures any day of the week. But we’re lying to ourselves if we think all of our troubles are gone. As soon as you start trying to write higher-order functions, or reuse code, you’re right back to realizing color is still there, bleeding all over your codebase.

这样更好。我将在一周中的任何一天通过裸露的回调或`future`进行异步等待。但如果我们认为所有的麻烦都过去了，那就是在自欺欺人。一旦你开始尝试编写高阶函数，或者重用代码，你马上就会意识到颜色仍然存在，这样的问题在你的代码库里到处都是。

## What language isn’t colored?
## 哪种语言没有颜色？
So JS, Dart, C#, and Python have this problem. CoffeeScript and most other languages that compile to JS do too (which is why Dart inherited it). I think even ClojureScript has this issue even though they’ve tried really hard to push against it with their core.async stuff.

因此，JS，Dart，C＃和Python都有此问题。 CoffeeScript和大多数其他可编译为JS的语言也可以这样做（这就是Dart继承它的原因）。 我认为，即使ClojureScript也确实遇到了这个问题，尽管他们已经非常努力地使用core.async东西来反对它。

Wanna know one that doesn’t? Java. I know right? How often do you get to say, “Yeah, Java is the one that really does this right.”? But there you go. In their defense, they are actively trying to correct this oversight by moving to futures and async IO. It’s like a race to the bottom.

有没有语言不知这么做的吗？有，这个就是Java。你看我说啥来者？你有多少次会说 "是啊，Java是一个真正做得对的"？但也就这样了。在你为Java辩护的时候，Java正积极地试图通过转向`future`和异步IO来纠正这种疏忽。这就像一场军备竞赛。

C# also actually can avoid this problem too. They opted in to having color. Before they added async-await and all of the Task<T> stuff, you just used regular sync API calls. Three more languages that don’t have this problem: Go, Lua, and Ruby.

C#其实也可以避免这个问题。但是C#选择了有颜色。在他们加入`async-await`和所有`Task<T>`这些元素之前，只需要使用常规`sync api`调用。目前有三种语言没有这个问题：Go、Lua和Ruby。

Any guess what they have in common?

有没有猜到他们有什么共同点？

Threads. Or, more precisely: multiple independent callstacks that can be switched between. It isn’t strictly necessary for them to be operating system threads. Goroutines in Go, coroutines in Lua, and fibers in Ruby are perfectly adequate.

线程。或者更准确的说：多个独立的、可以相互切换的调用栈。严格来说，它们不一定是操作系统的线程，Go中的Goroutines、Lua中的coroutines和Ruby中的fiber都完全够用。

(That’s why C# has that little caveat. You can avoid the pain of async in C# by using threads.)

(这就是为什么C#会有这样的小警告。通过使用线程，您可以避免C中异步的痛苦。)

## Remembrance of operations past

## 回顾过去的操作

The fundamental problem is “How do you pick up where you left off when an operation completes”? You’ve built up some big callstack and then you call some IO operation. For performance, that operation uses the operating system’s underlying asynchronous API. You cannot wait for it to complete because it won’t. You have to return all the way back to your language’s event loop and give the OS some time to spin before it will be done.

最根本的问题是“当一个操作完成时，你如何从你停止的地方重新开始”？你已经建立了一些大的调用栈，然后你调用一些IO操作。为了性能，该操作使用了操作系统的底层异步API。你不能等它完成，因为它不会完成。您必须完全回到您程序的循环事件里面处理，并为操作系统腾出一些时间进行运转(`time.sleep()`)。然后才能够完成。

Once it is, you need to resume what you were doing. The usual way a language “remembers where it is” is the callstack. That tracks all of the functions that are currently being invoked and where the instruction pointer is in each one.

一旦完成，您就需要恢复当时在做的事情。语言 "记住它在哪里 "的通常方式是调用栈。它跟踪了当前正在调用的所有函数，以及每个函数的指令指针在哪里。

But to do async IO, you have to unwind discard the entire C callstack. Kind of a Catch-22. You can do super fast IO, you just can’t do anything with the result! Every language that has async IO in its bowels—or in the case of JS, the browser’s event loop—copes with this in some way.

但是要执行异步IO，必须释放并丢弃整个C调用堆栈。每种在内置有异步IO的语言（对于JS，即浏览器的事件循环）都以某种方式对此进行了处理。

Node with its ever-marching-to-the-right callbacks stuffs all of those callframes in closures. When you do:

Node将带有不断向右的回调，一帧一帧的填充在闭包中。当你这样做的时候：

```js
function makeSundae(callback) {
  scoopIceCream(function (iceCream) {
    warmUpCaramel(function (caramel) {
      callback(pourOnIceCream(iceCream, caramel));
    });
  });
}
```
Each of those function expressions closes over all of its surrounding context. That moves parameters like iceCream and caramel off the callstack and onto the heap. When the outer function returns and the callstack is trashed, it’s cool. That data is still floating around the heap.

每一个函数表达式都会关闭其周围的所有上下文。这样会将诸如iceCream和焦糖之类的参数从调用栈移出并移到堆上。当外部函数返回并且调用堆栈被废弃时，它很酷。 该数据仍在堆中浮动。

The problem is you have to manually reify every damn one of these steps. There’s actually a name for this transformation: continuation-passing style. It was invented by language hackers in the 70s as an intermediate representation to use in the guts of their compilers. It’s a really bizarro way to represent code that happens to make some compiler optimizations easier to do.

问题是，你必须手动地将每一个该死的步骤重新规范化。实际上，这种转换有一个名字：CPS（[Continuation-Passing-Style, 续体传递风格](https://cloud.tencent.com/developer/article/1557083)）。它是由70年代的语言黑客们发明的，作为一种中间表示法，用于他们编译器的内部。这是一种非常奇怪的代码表示方式，恰好可以让一些编译器的优化更容易进行。


No one ever for a second thought that a programmer would write actual code like that. And then Node came along and all of the sudden here we are pretending to be compiler back-ends. Where did we go wrong?

从来没有人想过，一个程序员会写出这样的实际代码。然后Node出现了，突然间我们在这里假装成编译器后端。我们到底错在哪里？

Note that promises and futures don’t actually buy you anything, either. If you’ve used them, you know you’re still hand-creating giant piles of function literals. You’re just passing them to .then() instead of to the asynchronous function itself.

请注意，`promise`和`future`实际上也不会给你买任何东西。如果你使用过它们，你知道你仍然在手工创建大量的函数文本。您只是将它们传递给.then（），而不是传递给异步函数本身。

## Awaiting a generated solution
## 等待生成的解决方案
Async-await does help. If you peel back your compiler’s skull and see what it’s doing when it hits an await call you’d see it actually doing the CPS-transform. That’s why you need to use await in C#: it’s a clue to the compiler to say, “break the function in half here”. Everything after the await gets hoisted into a new function that it synthesizes on your behalf.

异步等待确实有帮助。如果你剥开编译器的外皮，看看它在执行什么，当它到达等待调用，你会看到它实际上正在做CPS转换。这就是为什么您需要在C＃中使用await的原因：这是编译器说“在这里将函数分成两部分”的线索。await之后的所有内容都会被提升到一个新的函数中，由它代为合成。

This is why async-await didn’t need any runtime support in the .NET framework. The compiler compiles it away to a series of chained closures that it can already handle. (Interestingly, closures themselves also don’t need runtime support. They get compiled to anonymous classes. In C#, closures really are a poor man’s objects.)

这就是为什么async-await在.NET框架中不需要任何运行时支持的原因。 编译器将其编译为一系列已经可以处理的链式闭包。 （有趣的是，闭包本身也不需要运行时支持。它们会编译为匿名类。在C＃中，闭包实际上是一个可怜人的对象。）

You might be wondering when I’m going to bring up generators. Does your language have a yield keyword? Then it can do something very similar.

您可能想知道我何时要启动生成器(`generator`)。 您的语言是否有yield关键字？ 然后它可以做一些非常相似的事情。

(In fact, I believe generators and async-await are isomorphic. I’ve got a bit of code floating around in some dark corner of my hard disc that implements a generator-style game loop using only async-await.)

事实上，我相信生成器和async-await是同构的。在我的硬盘的某个黑暗角落里有一段代码，只用async-await实现了一个游戏循环，这段代码就是生成器(`generator`)。)

Where was I? Oh, right. So with callbacks, promises, async-await, and generators, you ultimately end up taking your asynchronous function and smearing it out into a bunch of closures that live over in the heap.

我说到哪儿了？哦，对了。所以，通过回调、承诺、异步等待和生成器，你最终会把你的异步函数修改成一堆活在堆里的闭包。

Your function passes the outermost one into the runtime. When the event loop or IO operation is done, it invokes that function and you pick up where you left off. But that means everything above you also has to return. You still have to unwind the whole stack.

您的函数将最外面的函数传递给运行时。 完成事件循环或IO操作后，它将调用该函数，然后从上次中断的地方继续。 但这意味着您之上的所有事物也必须返回。 您仍然必须把整个栈都释放。

This is where the “red functions can only be called by red functions” rule comes from. You have to closurify the entire callstack all the way back to main() or the event handler.

这是“红色函数只能由红色函数调用”规则的原因。 您必须完全关闭整个调用堆栈，返回`main()`或事件处理程序。

## Reified callstacks
## 改良的调用栈
But if you have threads (green- or OS-level), you don’t need to do that. You can just suspend the entire thread and hop straight back to the OS or event loop without having to return from all of those functions.

但如果你有线程（绿色或操作系统级），你就不需要这样做。你可以直接暂停整个线程，直接跳回OS或事件循环，而不必从所有这些函数中返回。

Go is the language that does this most beautifully in my opinion. As soon as you do any IO operation, it just parks that goroutine and resumes any other ones that aren’t blocked on IO.

在我看来，Go是这方面做得最漂亮的语言。只要你做任何IO操作，它就会把这个goroutine停掉，并恢复其他没有被IO阻塞的goroutine。

If you look at the IO operations in the standard library, they seem synchronous. In other words, they just do work and then return a result when they are done. But it’s not that they’re synchronous in the sense that it would mean in JavaScript. Other Go code can run while one of these operations is pending. It’s that Go has eliminated the distinction between synchronous and asynchronous code.

如果你看看标准库中的IO操作，它们似乎是同步的。换句话说，它们只是做工作，然后在完成后返回一个结果。但这并不是说它们是JavaScript中意义上的同步。这些操作之一处于挂起状态时，其他Go代码可以运行， Go消除了同步代码和异步代码之间的区别。

Concurrency in Go is a facet of how you choose to model your program, and not a color seared into each function in the standard library. This means all of the pain of the five rules I mentioned above is completely and totally eliminated.

Go中的并发性是您选择如何对程序建模的一个方面，而不是标准库中每个函数的颜色。这意味着我上面提到的五条规则的所有痛苦都被完全消除了。

So, the next time you start telling me about some new hot language and how awesome its concurrency story is because it has asynchronous APIs, now you’ll know why I start grinding my teeth. Because it means you’re right back to red functions and blue ones.

所以，下一次你开始跟我讲一些新的热门语言，以及它的并发故事有多棒，因为它有异步API，现在你就会知道为什么我开始磨牙了。因为这意味着你马上就会回到函数是红色还是蓝色的纠结中。



## 注释

[^1]:原文地址<https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/>

2021-04-01【April Fools' Day】

