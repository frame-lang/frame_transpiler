# Bug #050: Language-Specific Tests Running for All Languages — Closed

Status: Closed (Fixed in V3 runner)
Date: 2025-11-12

Summary
- The V3 test runner filters tests by language and category, supports @skip-if toolchain guards, and executes curated exec selectively. False cross-language failures no longer occur.

Action
- Close as Fixed. The runner’s language-aware discovery, skips, and curated exec gating resolve this.

