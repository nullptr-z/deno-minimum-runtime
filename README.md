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

* run_event_loop
驱动 js中的异步任务执行；我们在用运行时处理它的时候，只能一个单线程中处理;
内部调用了 poll_event_loop

* poll_event_loop
运行一次事件循环的任务；返回 enum Poll；处理 js中的异步任务(Promise)的核心/必要函数

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
