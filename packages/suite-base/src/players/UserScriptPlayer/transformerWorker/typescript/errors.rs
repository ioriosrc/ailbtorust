```rust
use crate::{diagnostic_severity::DiagnosticSeverity, sources::SOURCES};

const NO_FUNC_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const NON_FUNC_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const BAD_TYPE_RETURN_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const LIMITED_UNIONS_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const UNIONS_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const FUNCTION_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_TYPE_LITERALS_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_INTERSECTION_TYPES_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const PREFER_ARRAY_LITERALS: DiagnosticSeverity = DiagnosticSeverity::Error;
const CLASS_ERROR: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_TYPEOF: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_TUPLES: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_NESTED_ANY: DiagnosticSeverity = DiagnosticSeverity::Error;
const NO_MAPPED_TYPES: DiagnosticSeverity = DiagnosticSeverity::Error;
```