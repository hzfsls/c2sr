use crate::vos_typedef::*;
```c
typedef VOS_VOID *(*VOS_PRE_DLL_OPEN_HOOK_FUNC)(const VOS_CHAR *pscFileName, VOS_INT32 siFlag);
```

```rust
type VosPreDllOpenHookFunc = fn(&str, VosInt32) -> *mut VosVoid;



```