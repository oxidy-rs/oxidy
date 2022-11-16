# Changelog

# 0.5.0 (Alpha) |

- Refactor File & Folder Structure
- Refactor suggestion given by cargo clippy
- Full async/await support
- Improved Middleware for async/await
- Added Route & Middleware macro for better DX
- Added Json Response
- Moved from threadpool completely to Tokio Runtime
- Moved from Standard Tcp to Tokio Tcp
- Added Exception for Debug
- Added Code of Conduct
- Changed License

### Breaking

- Rewrite Entire Codebase (Everything)

# 0.4.0 (Alpha) | 2022-02-13

- Removed Unnecessary Codes & variables & Loops
- Replace own Threadpool with "threadpool" Crate
- Implemented fork for gain more performance
- Made Separate Module for Cpus
- Refactor from Loops to Iterators
- Rename fork file with listen
- Added "Futures" Crate
- Better Memory Manage
- Find Route Callback in Parallel (Improved Latency)

# 0.3.1 (Alpha) | 2022-02-06

- Added client IP to request header
- Map Function Consistent Parameter
- Improved Query Parser
- Added Benchmark

### Breaking

- Refactor Request Query from Variable to Function (To gain more performance)

# 0.2.1 (Alpha) | 2022-02-05

- Match Path Bug Fixed

# 0.2.0 (Alpha) | 2022-02-05

- Update Cargo Info
- Remove Regex with Match
- Improved Performance. (Replacing Regex boot performance by huge margin)

# 0.1.0 (Alpha) | 2022-02-05

- Initial Publish
