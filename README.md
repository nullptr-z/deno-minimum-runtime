# deno-minimum-runtime

## Trait
PollFn  代表这是一个 async 函数,通常是 poll_fn 的返回值

## Structure

* JsRuntime
单线程的，禁用异常回溯机制

## Enum

* Poll：Ready、Pending;  Promise的状态是否为可用了, 对应 v8的 PromiseState，Pending、Fulfilled、Rejected

## Function

#### 加载&执行
* execute_script
执行js代码。如果 execute_script 执行的是异步函数，会返回一个promise，需要调用 resolve_value 拿到返回值

* mod_evaluate
执行 ModuleId 模块，对应 v8的 module.evaluate

* load_main_module 和 load_side_module
*entry point 和 utility module*
加载一个 js模块,实例化他，并生成一个 ModuleId,类似文件系统的描述符; 实例化使用了 v8的 module.instantiate_module

#### 异步相关

* poll_event_loop
运行一次事件循环的任务；返回 enum Poll；处理 js中的异步任务(Promise)的核心/必要函数

* run_event_loop
驱动 js中的异步任务执行；我们在用运行时处理它的时候，只能一个单线程中处理;
内部调用了 poll_event_loop

* poll_value
得到执行后，返回对应状态的值 match PromiseState： Pending、Fulfilled、Rejected
内部调用了 poll_event_loop

* resolve_value
处理js的 Promise 异步函数，等待 Promise 执行返回；
内部调用 poll_value

#### 偏底层函数
* poll_fn
这是一个sync方法，但是他返回值是一个异步方法PollFn

* add_near_heap_limit_callback
v8的 heap 达到限界时，会调用这个函数，通过回调我们可以给一个新的 limit


## v8
通常情况下，我们使用 v8 引擎时，需要创建一个 Isolate 对象，然后在该对象上创建一个 HandleScope 对象，最后再创建一个 Context 对象，这样才能在 C++ 中执行 JavaScript 代码。

#### Isolate 沙箱

Isolate 是 V8 引擎的实例，它提供了一个完全隔离的 JavaScript 沙箱环境，其中的每一个环境都是相互独立的。

一个 Isolate 中可以运行多个 JavaScript 线程，这些线程共享同一个内存空间，因此需要一些特殊的手段来保证多线程间的数据安全和同步。通常情况下，多个线程之间是不会共享 JavaScript 对象的，也就是说，它们都有自己的对象实例

在创建 Isolate 时，可以指定它所需的内存大小，这取决于应用程序的需求。如果应用程序需要更多的内存，可以通过增加 Isolate 大小或在运行时动态分配内存来满足需求。


#### HandleScope 句柄·管理的域

当我们需要在 C++ 中创建 JavaScript 对象时，需要将其包装为一个句柄（Handle）对象

HandleScope 可以看作是一个作用域，在该作用域内创建的所有 JavaScript对象的句柄（Handles）都将被自动管理，避免内存泄漏或野指针等问题，HandleScope 被销毁时，其内部创建的所有对象句柄也将被自动销毁。

HandleScope 创建时，因为 V8本身是多线程的，我们需要将当前线程的 Isolate 对象作为参数传入，必须在Isolate上下文中创建 HandleScope, 而且不同 Isolate 之间的 HandleScope 不能相互使用。

一个 Isolate 中可以有多个 HandleScope，每个 HandleScope 可以管理一组句柄（handles）。HandleScope 只能使用各自 scope域的 handle。同样为了避免竞争


#### Context（上下文）

V8中的一个重要概念，用于表示JavaScript代码的执行环境。Context包含了JavaScript代码在运行时所需的所有信息，如全局对象、变量、函数等。每个Context都拥有自己独立的全局对象，在不同的Context中，同名的变量、函数等可能拥有不同的值或实现。

使用Context，可以在同一个应用程序中同时运行多个独立的JavaScript代码片段，每个片段都拥有自己的Context。同时，通过切换Context，可以实现在同一线程中执行不同的JavaScript代码，并保证它们不会互相影响。Context是线程安全的，可以在不同的线程中创建和使用。
