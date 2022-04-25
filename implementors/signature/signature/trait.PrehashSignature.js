(function() {var implementors = {};
implementors["ecdsa"] = [{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"signature/signature/trait.PrehashSignature.html\" title=\"trait signature::signature::PrehashSignature\">PrehashSignature</a> for <a class=\"struct\" href=\"ecdsa/der/struct.Signature.html\" title=\"struct ecdsa::der::Signature\">Signature</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"ecdsa/trait.PrimeCurve.html\" title=\"trait ecdsa::PrimeCurve\">PrimeCurve</a> + <a class=\"trait\" href=\"ecdsa/hazmat/trait.DigestPrimitive.html\" title=\"trait ecdsa::hazmat::DigestPrimitive\">DigestPrimitive</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"ecdsa/der/type.MaxSize.html\" title=\"type ecdsa::der::MaxSize\">MaxSize</a>&lt;C&gt;: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"type\" href=\"elliptic_curve/type.FieldSize.html\" title=\"type elliptic_curve::FieldSize\">FieldSize</a>&lt;C&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/arith/trait.Add.html#associatedtype.Output\" title=\"type core::ops::arith::Add::Output\">Output</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"type\" href=\"ecdsa/der/type.MaxOverhead.html\" title=\"type ecdsa::der::MaxOverhead\">MaxOverhead</a>&gt; + <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a>&gt;,&nbsp;</span>","synthetic":false,"types":["ecdsa::der::Signature"]},{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"signature/signature/trait.PrehashSignature.html\" title=\"trait signature::signature::PrehashSignature\">PrehashSignature</a> for <a class=\"struct\" href=\"ecdsa/struct.Signature.html\" title=\"struct ecdsa::Signature\">Signature</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"ecdsa/hazmat/trait.DigestPrimitive.html\" title=\"trait ecdsa::hazmat::DigestPrimitive\">DigestPrimitive</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"type\" href=\"elliptic_curve/type.FieldSize.html\" title=\"type elliptic_curve::FieldSize\">FieldSize</a>&lt;C&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/arith/trait.Add.html#associatedtype.Output\" title=\"type core::ops::arith::Add::Output\">Output</a>: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a>&gt;,&nbsp;</span>","synthetic":false,"types":["ecdsa::Signature"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()