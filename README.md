# Tracking Deno memory leaks

This repository contains a minimal example that shows whether a particular Deno runtime version has memory leaks or not.

No leaks are especially desirable in an environment where the same server process handles many FaaS calls.

Steps to reproduce:

Build, [sign](https://github.com/mlafeldt/dotfiles/blob/main/bin/codesign-for-instruments), and run the binary with the following command:

```
cargo build && codesign-for-instruments target/debug/deno-memleak && target/debug/deno-memleak
```

Open Instruments.app on macOS, select the "Leaks" template, attach it to the running deno-memleak process, and start recording (Cmd+R).

The result will look like this if there are any leaks:

![](Instruments.png)

Fixed leaks:

- ~~[Memory leak when transpiling modules during bootstrap](https://github.com/denoland/deno/issues/24380)~~ Fixed in Deno 1.45.0
- ~~CFunctionInfo/CTypeInfo leak in OpCtx: <https://github.com/denoland/deno_core/pull/714> / <https://github.com/denoland/deno/pull/24169>~~ Fixed in Deno 1.46.0

Known open leaks:

none 💪
