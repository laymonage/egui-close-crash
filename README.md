# eframe 0.36.2: closing the window crashes on macOS

A minimal eframe app aborts with an uncaught `NSRangeException` when its window
is closed with the red close button. Quitting with ⌘Q exits normally.

## Environment

- eframe 0.36.2 (default features), winit 0.30.13 (pinned in `Cargo.lock`)
- Also reproduces on `main` at [`efcf497`](https://github.com/emilk/egui/commit/efcf49788df313b7ff701995f211aa543d2c9ac5),
  with `egui` and `eframe` patched to the `main` branch in `[patch.crates-io]`
- macOS 27.0 (26A428), Mac mini (Mac16,10, Apple M4). This Mac has no Touch Bar.
- rustc 1.99.0

## Steps

1. `cargo run`
2. Close the window with the red close button.

Expected: the process exits with status 0.

Actual: the process aborts (exit status 134) and macOS shows the "quit
unexpectedly" dialog:

```
*** Terminating app due to uncaught exception 'NSRangeException', reason: 'Cannot remove an observer <_NSTouchBarFinderObservation 0x75451b4d40> for the key path "nextResponder" from <WinitView 0x7544f50900> because it is not registered as an observer.'
*** First throw call stack:
(
	0   CoreFoundation    __exceptionPreprocess + 176
	1   libobjc.A.dylib   objc_exception_throw + 88
	2   Foundation        -[NSObject(NSKeyValueObserverRegistration) _removeObserver:forProperty:] + 636
	3   Foundation        -[NSObject(NSKeyValueObserverRegistration) removeObserver:forKeyPath:] + 136
	4   Foundation        -[NSObject(NSKeyValueObserverRegistration) removeObserver:forKeyPath:context:] + 196
	5   AppKit            -[_NSTouchBarFinderObservation invalidate] + 264
	6   AppKit            ___NSTouchBarFinderSetNeedsUpdateOnMain_block_invoke_2 + 636
	7   AppKit            NSDisplayCycleObserverInvoke + 152
	8   AppKit            NSDisplayCycleFlush + 656
	9   QuartzCore        CA::Transaction::run_commit_handlers(CATransactionPhase) + 120
	10  QuartzCore        CA::Transaction::commit() + 324
	11  QuartzCore        CA::Transaction::flush() + 84
	12  QuartzCore        CA::Transaction::flush_as_runloop_observer(bool) + 140
	13  AppKit            stepTransactionFlush + 588
	14  UpdateCycle       UC::DriverCore::continueProcessing() + 84
	15  CoreFoundation    __CFRUNLOOP_IS_CALLING_OUT_TO_AN_OBSERVER_CALLBACK_FUNCTION__ + 36
	16  CoreFoundation    __CFRunLoopDoObservers + 648
	17  CoreFoundation    _CFRunLoopRunSpecificWithOptions + 488
	18  HIToolbox         RunCurrentEventLoopInMode + 320
	19  HIToolbox         ReceiveNextEventCommon + 492
	20  HIToolbox         _BlockUntilNextEventMatchingListInMode + 48
	21  AppKit            _DPSBlockUntilNextEventMatchingListInMode + 292
	22  AppKit            _DPSNextEvent + 580
	23  AppKit            -[NSApplication(NSEventRouting) _nextEventMatchingEventMask:untilDate:inMode:dequeue:] + 780
	24  AppKit            -[NSApplication(NSEventRouting) nextEventMatchingMask:untilDate:inMode:dequeue:] + 292
	25  AppKit            -[NSApplication run] + 396
	... winit run_on_demand → eframe run_and_return → eframe::run_native
)
```

The exception is raised from the run loop after the window's `WinitView` has
been torn down, while eframe is still running the event loop.

## Observations

- ⌘Q quits cleanly. Only the close button crashes.
- It crashes with both the wgpu and glow renderers.
- An app on eframe 0.31.1 with the same winit 0.30.13 closes cleanly on the
  same machine, so the change is on the eframe side.
- `persist_window: false` does not help here. Without the `persistence` feature
  it has no effect.
- With `run_and_return: false`, no exception was printed, but in one test the
  process didn't exit after the close button was clicked.
- Workaround: calling `std::process::exit(0)` in `App::on_exit` avoids the
  crash, because it runs before the window is torn down.

## Related

- <https://github.com/storytold/craft-libs/issues/3>: the same crash in apps on eframe 0.36.2
- <https://github.com/emilk/egui/discussions/7857>: a similar close crash on eframe 0.33.3, fixed there with `persist_window: false`
- <https://github.com/zed-industries/zed/issues/64819>: the same AppKit exception in GPUI
