(function() {var implementors = {};
implementors["wasmtime_environ"] = [{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.InstructionAddressMap.html\" title=\"struct wasmtime_environ::InstructionAddressMap\">InstructionAddressMap</a>","synthetic":true,"types":["wasmtime_environ::address_map::InstructionAddressMap"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.FilePos.html\" title=\"struct wasmtime_environ::FilePos\">FilePos</a>","synthetic":true,"types":["wasmtime_environ::address_map::FilePos"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.AddressMapSection.html\" title=\"struct wasmtime_environ::AddressMapSection\">AddressMapSection</a>","synthetic":true,"types":["wasmtime_environ::address_map::AddressMapSection"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.BuiltinFunctionIndex.html\" title=\"struct wasmtime_environ::BuiltinFunctionIndex\">BuiltinFunctionIndex</a>","synthetic":true,"types":["wasmtime_environ::builtin::BuiltinFunctionIndex"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.FunctionInfo.html\" title=\"struct wasmtime_environ::FunctionInfo\">FunctionInfo</a>","synthetic":true,"types":["wasmtime_environ::compilation::FunctionInfo"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.Trampoline.html\" title=\"struct wasmtime_environ::Trampoline\">Trampoline</a>","synthetic":true,"types":["wasmtime_environ::compilation::Trampoline"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.StackMapInformation.html\" title=\"struct wasmtime_environ::StackMapInformation\">StackMapInformation</a>","synthetic":true,"types":["wasmtime_environ::compilation::StackMapInformation"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.CompileError.html\" title=\"enum wasmtime_environ::CompileError\">CompileError</a>","synthetic":true,"types":["wasmtime_environ::compilation::CompileError"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.Setting.html\" title=\"struct wasmtime_environ::Setting\">Setting</a>","synthetic":true,"types":["wasmtime_environ::compilation::Setting"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.SettingKind.html\" title=\"enum wasmtime_environ::SettingKind\">SettingKind</a>","synthetic":true,"types":["wasmtime_environ::compilation::SettingKind"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.FlagValue.html\" title=\"enum wasmtime_environ::FlagValue\">FlagValue</a>","synthetic":true,"types":["wasmtime_environ::compilation::FlagValue"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.MemoryStyle.html\" title=\"enum wasmtime_environ::MemoryStyle\">MemoryStyle</a>","synthetic":true,"types":["wasmtime_environ::module::MemoryStyle"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.MemoryPlan.html\" title=\"struct wasmtime_environ::MemoryPlan\">MemoryPlan</a>","synthetic":true,"types":["wasmtime_environ::module::MemoryPlan"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.MemoryInitializer.html\" title=\"struct wasmtime_environ::MemoryInitializer\">MemoryInitializer</a>","synthetic":true,"types":["wasmtime_environ::module::MemoryInitializer"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.StaticMemoryInitializer.html\" title=\"struct wasmtime_environ::StaticMemoryInitializer\">StaticMemoryInitializer</a>","synthetic":true,"types":["wasmtime_environ::module::StaticMemoryInitializer"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.MemoryInitialization.html\" title=\"enum wasmtime_environ::MemoryInitialization\">MemoryInitialization</a>","synthetic":true,"types":["wasmtime_environ::module::MemoryInitialization"]},{"text":"impl&lt;'a&gt; !<a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.InitMemory.html\" title=\"enum wasmtime_environ::InitMemory\">InitMemory</a>&lt;'a&gt;","synthetic":true,"types":["wasmtime_environ::module::InitMemory"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.TableStyle.html\" title=\"enum wasmtime_environ::TableStyle\">TableStyle</a>","synthetic":true,"types":["wasmtime_environ::module::TableStyle"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TablePlan.html\" title=\"struct wasmtime_environ::TablePlan\">TablePlan</a>","synthetic":true,"types":["wasmtime_environ::module::TablePlan"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TableInitializer.html\" title=\"struct wasmtime_environ::TableInitializer\">TableInitializer</a>","synthetic":true,"types":["wasmtime_environ::module::TableInitializer"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.TableInitialization.html\" title=\"enum wasmtime_environ::TableInitialization\">TableInitialization</a>","synthetic":true,"types":["wasmtime_environ::module::TableInitialization"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.ModuleType.html\" title=\"enum wasmtime_environ::ModuleType\">ModuleType</a>","synthetic":true,"types":["wasmtime_environ::module::ModuleType"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.Module.html\" title=\"struct wasmtime_environ::Module\">Module</a>","synthetic":true,"types":["wasmtime_environ::module::Module"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.Initializer.html\" title=\"enum wasmtime_environ::Initializer\">Initializer</a>","synthetic":true,"types":["wasmtime_environ::module::Initializer"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.ModuleUpvar.html\" title=\"enum wasmtime_environ::ModuleUpvar\">ModuleUpvar</a>","synthetic":true,"types":["wasmtime_environ::module::ModuleUpvar"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TypeTables.html\" title=\"struct wasmtime_environ::TypeTables\">TypeTables</a>","synthetic":true,"types":["wasmtime_environ::module::TypeTables"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleSignature.html\" title=\"struct wasmtime_environ::ModuleSignature\">ModuleSignature</a>","synthetic":true,"types":["wasmtime_environ::module::ModuleSignature"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.InstanceSignature.html\" title=\"struct wasmtime_environ::InstanceSignature\">InstanceSignature</a>","synthetic":true,"types":["wasmtime_environ::module::InstanceSignature"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.FunctionType.html\" title=\"struct wasmtime_environ::FunctionType\">FunctionType</a>","synthetic":true,"types":["wasmtime_environ::module::FunctionType"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.AnyfuncIndex.html\" title=\"struct wasmtime_environ::AnyfuncIndex\">AnyfuncIndex</a>","synthetic":true,"types":["wasmtime_environ::module::AnyfuncIndex"]},{"text":"impl&lt;'data&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleEnvironment.html\" title=\"struct wasmtime_environ::ModuleEnvironment\">ModuleEnvironment</a>&lt;'data&gt;","synthetic":true,"types":["wasmtime_environ::module_environ::ModuleEnvironment"]},{"text":"impl&lt;'data&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.ModuleTranslation.html\" title=\"struct wasmtime_environ::ModuleTranslation\">ModuleTranslation</a>&lt;'data&gt;","synthetic":true,"types":["wasmtime_environ::module_environ::ModuleTranslation"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.FunctionBodyData.html\" title=\"struct wasmtime_environ::FunctionBodyData\">FunctionBodyData</a>&lt;'a&gt;","synthetic":true,"types":["wasmtime_environ::module_environ::FunctionBodyData"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.DebugInfoData.html\" title=\"struct wasmtime_environ::DebugInfoData\">DebugInfoData</a>&lt;'a&gt;","synthetic":true,"types":["wasmtime_environ::module_environ::DebugInfoData"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.NameSection.html\" title=\"struct wasmtime_environ::NameSection\">NameSection</a>&lt;'a&gt;","synthetic":true,"types":["wasmtime_environ::module_environ::NameSection"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.WasmFileInfo.html\" title=\"struct wasmtime_environ::WasmFileInfo\">WasmFileInfo</a>","synthetic":true,"types":["wasmtime_environ::module_environ::WasmFileInfo"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.FunctionMetadata.html\" title=\"struct wasmtime_environ::FunctionMetadata\">FunctionMetadata</a>","synthetic":true,"types":["wasmtime_environ::module_environ::FunctionMetadata"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.StackMap.html\" title=\"struct wasmtime_environ::StackMap\">StackMap</a>","synthetic":true,"types":["wasmtime_environ::stack_map::StackMap"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TrapEncodingBuilder.html\" title=\"struct wasmtime_environ::TrapEncodingBuilder\">TrapEncodingBuilder</a>","synthetic":true,"types":["wasmtime_environ::trap_encoding::TrapEncodingBuilder"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TrapInformation.html\" title=\"struct wasmtime_environ::TrapInformation\">TrapInformation</a>","synthetic":true,"types":["wasmtime_environ::trap_encoding::TrapInformation"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"wasmtime_environ/enum.TrapCode.html\" title=\"enum wasmtime_environ::TrapCode\">TrapCode</a>","synthetic":true,"types":["wasmtime_environ::trap_encoding::TrapCode"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.Tunables.html\" title=\"struct wasmtime_environ::Tunables\">Tunables</a>","synthetic":true,"types":["wasmtime_environ::tunables::Tunables"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.VMOffsets.html\" title=\"struct wasmtime_environ::VMOffsets\">VMOffsets</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["wasmtime_environ::vmoffsets::VMOffsets"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.HostPtr.html\" title=\"struct wasmtime_environ::HostPtr\">HostPtr</a>","synthetic":true,"types":["wasmtime_environ::vmoffsets::HostPtr"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.VMOffsetsFields.html\" title=\"struct wasmtime_environ::VMOffsetsFields\">VMOffsetsFields</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["wasmtime_environ::vmoffsets::VMOffsetsFields"]},{"text":"impl <a class=\"trait\" href=\"wasmtime_environ/__core/panic/trait.UnwindSafe.html\" title=\"trait wasmtime_environ::__core::panic::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"wasmtime_environ/struct.TargetSharedSignatureIndex.html\" title=\"struct wasmtime_environ::TargetSharedSignatureIndex\">TargetSharedSignatureIndex</a>","synthetic":true,"types":["wasmtime_environ::vmoffsets::TargetSharedSignatureIndex"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()