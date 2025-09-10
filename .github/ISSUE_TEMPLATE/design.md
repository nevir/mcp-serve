---
name: Design
about: A technical blueprint outlining "how" the feature will be engineered and implemented.
labels: ["design"]
---

<!--
`Design` issues are ALWAYS a sub-issue of the `Feature` issue they describe.
There is only one `Design` per `Feature`.

Principles:
✅ Focus on HOW to implement (technical decisions, architecture, APIs)
❌ Avoid changing WHAT or WHY (those belong in the Spec)
❌ Avoid task breakdowns (use `/plan` command for implementation planning)
🤔 Use categorized clarification markers:
   - [BLOCKS IMPLEMENTATION: question] for decisions that must be made before planning
   - [IMPLEMENTATION DECISION: question] for decisions that can be made during implementation
   - [RESEARCH OPPORTUNITY: question] for optional research that could inform the approach
🔗 Cross-reference: When referencing a technology, concept, feature, etc - LINK to it!
-->

[Spec](LINK TO THE SPEC ISSUE)

## Overview

<!--
Brief summary of what you're building technically. This should connect the user needs from the Spec to the technical solution.
Example: "Implement shell completion using Rust's clap_complete crate with a `mcp-serve completion <shell>` subcommand that outputs completion scripts for eval or static file generation."
-->

## Architecture

<!--
High-level technical components and how they fit together.
Include: key modules/crates, data flow, integration points
Example: "Extend mcp-serve with completion subcommand, integrate clap_complete for script generation, modify Homebrew formula to auto-install completions"
-->

## Technical Decisions

<!--
Key technical choices with brief rationale.
Examples:
- "Use clap_complete over custom completion - proven, maintained, integrates with existing clap usage"
- "Generate scripts dynamically vs static files - allows runtime customization of completions"
-->

## Open Technical Questions

<!--
Technical decisions or research that still need resolution.
Examples:
- "Should completion scripts be embedded at compile time or generated dynamically?"
- "How to handle custom subcommands from plugins in completion?"
-->

## Risks & Mitigation

<!--
Technical risks and how to address them.
Examples:
- "Risk: Completion might be slow for large configs. Mitigation: Cache completion data, benchmark performance"
- "Risk: Shell compatibility issues. Mitigation: Test across multiple shell versions, fallback gracefully"
-->

## Implementation Notes

<!--
Decisions that can be made during implementation, with validation criteria.
Examples:
- "Specific clap_complete API usage - validate performance is <100ms"
- "Shell detection method - test across target environments during implementation" 
-->
