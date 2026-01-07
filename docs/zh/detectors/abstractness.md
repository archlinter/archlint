# 抽象度违规 (Abstractness Violation)

**ID:** `abstractness_violation` | **严重程度:** 低 (默认)

基于 Robert C. Martin 的“主序列” (Main Sequence) 指标。它衡量稳定性 (I) 和抽象度 (A) 之间的平衡。模块应该是稳定且抽象的，或者是涉及具体实现且不稳定的。

## 为什么这是一种坏味道

稳定且具体的模块处于“痛苦地带” (Zone of Pain)（难以更改，但其他模块依赖它们）。不稳定且抽象的模块处于“无用地带” (Zone of Uselessness)（没有其他模块依赖它们，但它们却是抽象的）。

## 如何修复

调整模块的抽象度（例如，通过引入接口）或其稳定性（通过更改谁依赖它）。
