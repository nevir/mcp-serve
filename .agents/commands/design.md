Author a technical design for the given feature.

1. Find the `Spec` issue:

   1. If the user provided a link to a GitHub issue, read it:
      - It should be a `Spec` issue. If it has [NEEDS CLARIFICATION] markers or open research: warn the user that the spec is incomplete and ask if they want to proceed with design anyway (parallel work is allowed but may require spec updates).
      - If they confirm to proceed: the spec issue is our starting point.
      - Find the parent `Feature` issue (the 'root' issue) from the spec's sub-issue relationship.

   2. If the user did not provide a link to a GitHub issue:
      - Ask them to provide the Spec issue URL.

2. Read the [`Design` template](/.github/ISSUE_TEMPLATE/design.md)

3. Create a new GitHub issue following the `Design` template:
   - Use the requirements and user scenarios from the `Spec` issue to inform the technical design.
   - This design issue will be a sub-issue of the 'root' `Feature` issue (not the Spec).
     - The title of the issue must be prefixed by the parent feature issue, separated by `:`.
       - E.g. "Shell Completion: Design" for a "Design" sub-issue of "Shell Completion".
   - After creating the issue, add it as a sub-issue to the 'root' `Feature` issue.

4. Output a hyperlink to the new design issue
   - A numbered list of ALL points in the design that need clarification.

5. Work with the user to complete the design:
   1. Clarify ambiguous points. If there are clarification markers:
      - Output a numbered list of ALL points in the design that need clarification, categorized by type, and ask the user to help clarify them.
      - Based on the user's response, update the design accordingly.
      - For `[BLOCKS IMPLEMENTATION: question]` markers that the user cannot resolve:
        - Read the [`Research` template](/.github/ISSUE_TEMPLATE/research.md)
        - Create a new `Research` issue as a sub-issue to the `Design` issue
        - The `Research` issue should describe the blocking technical question that needs resolution
        - Update the `Design` issue to remove the marker and link to the research sub-issue.
      - For `[IMPLEMENTATION DECISION: question]` markers:
        - Move these to the Implementation Notes section with validation criteria
        - Remove the marker from the main content
      - For `[RESEARCH OPPORTUNITY: question]` markers:
        - Either create optional research tasks or document as future considerations
        - Remove markers after handling

   2. Perform research. If there are open research tasks:
      - If you support spawning sub-agents:
        - For each open research task: generate and spawn a sub-agent to follow `.agents/commands/work.md` for the research task.
      - If you do not support sub-agents:
        - Output a numbered list of ALL open research tasks
        - STOP HERE and ask the user which one you should investigate.
        - ONCE THE USER RESPONDS: Follow `.agents/commands/work.md` to work on the chosen research task.

   3. Handle identified risks. If the design identifies technical risks:
      - For each risk with active mitigation: create validation tasks to verify mitigation approach
      - For risks requiring ongoing monitoring: add implementation notes about what to watch for
      - Update the Design issue with specific mitigation validation criteria

   4. Handle spec conflicts. If the design work uncovers:
      - New user requirements or changes to existing requirements
      - Conflicts with the original spec scope or assumptions
      - Technical constraints that affect user-facing functionality

      Then:
      - Reopen the `Spec` issue and update it to reflect the new findings
      - If these conflicts are blockers to the technical design: PAUSE the design work and ask the user to resolve the spec issues first
      - Once spec is updated, resume design work with the updated requirements

6. Repeat (5) until the design is complete. A design is considered complete when:
   - No `[BLOCKS IMPLEMENTATION]` markers remain
   - All `[IMPLEMENTATION DECISION]` points have clear validation criteria
   - All technical decisions have clear rationale
   - Technical risks are identified with mitigation strategies
   - All research is complete
   - Architecture supports all requirements from the spec
   - Implementation notes document deferred decisions with context

7. Complete the design:
   - Update the 'root' `Feature` issue if necessary to reflect the technical approach.
   - Close the `Design` issue and inform the user that the design is complete and ready for implementation.
   - If the user disagrees or identifies additional work needed, reopen the `Design` issue and return to step 5.

That's it.
