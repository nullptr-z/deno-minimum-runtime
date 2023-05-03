# deno-minimum-runtime

## Structure

* JsRuntime
单线程的，禁用异常回溯机制

## Function

* add_near_heap_limit_callback
v8的 heap 达到限界时，会调用这个函数，通过回调我们可以给一个新的 limit

* run_event_loop
驱动 js中的异步任务执行；我们在用运行时处理它的时候，只能一个单线程中处理

* load_main_module 和 load_side_module
*entry point 和 utility module*
加载一个 js模块,并实例化他，对应这 v8的 module.instantiate_module；并生成一个 ModuleId,类似文件系统的描述符

* mod_evaluate
执行 ModuleId 模块，对应 v8的 module.evaluate
