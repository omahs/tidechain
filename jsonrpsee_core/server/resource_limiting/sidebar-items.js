initSidebarItems({"struct":[["ResourceGuard","RAII style “lock” for claimed resources, will automatically release them once dropped."],["Resources","User defined resources available to be used by calls on the JSON-RPC server. Each of the 8 possible resource kinds, for instance “cpu”, “io”, “nanobots”, store a maximum `capacity` and a default. A value of `0` means no limits for the given resource."]],"type":[["ResourceTable","Fixed size table, mapping a resource to a (unitless) value indicating the amount of the resource that is available to RPC calls."],["ResourceVec","Variable size table, mapping a resource to a (unitless) value indicating the amount of the resource that is available to RPC calls."]]});