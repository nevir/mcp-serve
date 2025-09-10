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
Example: "Implement a new REST API endpoint for user profile updates, including data validation and persistence to the database."
-->

## Architecture

<!--
High-level technical components and how they fit together.
Include: key modules/crates, data flow, integration points
Example: "Add a new controller to handle `/api/v1/profile` requests, a service layer for business logic, and a repository for database interaction. Update the API gateway to route requests."
-->

## Technical Decisions

<!--
Key technical choices with brief rationale for the chosen approach.
Examples:
- "Use JWT for authentication to ensure stateless and secure API calls."
- "Use a PostgreSQL database for its robustness and support for JSON data types."
-->

## Open Technical Questions

<!--
Technical decisions or research that still need resolution.
Examples:
- "Should we use a document-based or relational schema for storing user preferences?"
- "What is the best strategy for caching user session data?"
-->

## Risks & Mitigation

<!--
Technical risks and how to address them.
Examples:
- "Risk: High latency on profile image uploads. Mitigation: Offload image processing to a background worker queue."
- "Risk: Database schema changes may require downtime. Mitigation: Implement a blue-green deployment strategy for database migrations."
-->

## Alternatives Considered

<!--
For each alternative, create a sub-section and document the pros, cons, and why it was not chosen.
-->

### Alternative: [Name]

<!--
Description of approach

**Pros:**
- Benefit 1
- Benefit 2

**Cons:**
- Drawback 1
- Drawback 2

**Rationale for discarding:** Why we ultimately didn't choose it
-->

### Alternative: [Name]

<!-- Repeat pattern above -->

## Implementation Notes

<!--
Decisions that can be made during implementation, with validation criteria.
Examples:
- "Specific validation library to use - validate that it supports custom validation rules."
- "Error handling strategy for external API calls - test for graceful failure and retries."
-->
