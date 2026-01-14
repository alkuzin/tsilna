# Tsilna

**Tsilna** is a memory-safe autonomous navigation framework designed for the usage of both constrained `embedded systems` (no-std compatible) and general-purpose operating systems.

---

## 💡Overview

Built with the **Rust**, Tsilna provides a modular architecture that ensures safety-critical reliability without sacrificing developer velocity.

## 🛠️ Features

The framework is built as a facade, allowing you to include only the components necessary for your specific target:

* **`protocol`**: Enables binary protocol implementations and networking utilities. Includes RFC-compliant checksum and IDTP data framing.

* **`math`**: Provides math primitives essential for autonomous navigation.

## 📜 License

Copyright (C) 2026-present tsilna project and contributors.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
