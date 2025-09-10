Author a spec for the given feature.

1. Find or define the `Feature` issue:

   1. If the user provided a link to a GitHub issue, read it:
      - It should be a `Feature` issue. If not: STOP and tell the user.
      - If it is: the issue is our 'root' issue.

   2. If the user did not provide a link to a GitHub issue:
      - Folllow the instructions in [.agents/commands/feature.md](/.agents/commands/feature.md) to create a new `Feature` issue to be our 'root' issue.

2. Read the [`Spec` template](/.github/ISSUE_TEMPLATE/spec.md)

3. Create a new GitHub issue following the `Spec` template:
   - Use the description of our 'root' issue as a starting point to begin to fill in the template.
   - This spec issue will be a sub-issue of the 'root' `Feature` issue.
     - The title of the issue must be prefixed by the parent issue, separated by `:`.
       - E.g. "Shell Completion: Spec" for a "Spec" sub-issue of "Shell Completion".
   - After creating the issue, add it as a sub-issue to the 'root' issue.

4. Output a hyperlink to the new spec issue
   - A numbered list of ALL points in the spec that need clarification.

5. Work with the user to complete the spec:
   1. Clarify ambiguous points. If there are [NEEDS CLARIFICATION] markers:
      - Output a numbered list of ALL points in the spec that need clarification, and ask the user to help clarify them.
      - Based on the user's response, update the spec accordingly.
      - If the user indicates they are unsure about a particular point, or that it is an open question:
        - Read the [`Research` template](/.github/ISSUE_TEMPLATE/research.md)
        - Create a new `Research` issue as a sub-issue to the `Spec` issue
        - The `Research` issue should describe the open question or decision that needs to be resolved
        - Update the `Spec` issue to remove the [NEEDS CLARIFICATION] marker, and instead have it link to the research sub-issue.

   2. Perform research. If there are open research tasks:
      - If you support spawning sub-agents:
        - For each open research task: generate and spawn a sub-agent to follow `.agents/commands/work.md` for the research task.
      - If you do not support sub-agents:
        - Output a numbered list of ALL open research tasks
        - STOP HERE and ask the user which one you should investigate.
        - ONCE THE USER RESPONDS: Follow `.agents/commands/work.md` to work on the chosen research task.

6. Repeat (5) until the spec is complete. A spec is considered complete when:
   - No [NEEDS CLARIFICATION] markers remain
   - Edge cases have clearly defined outcomes
   - All research is complete
   - Requirements are testable and unambiguous
   - Success criteria are measurable
   - Scope is clearly bounded
   - Dependencies and assumptions identified

7. Confirm with the user that the spec is complete. Otherwise, repeat (5) until they are satisfied.
   - If the user confirms, close the `Spec` issue.
   - Update the 'root' `Feature` issue if necessary, so that it reflects the latest thinking.

That's it.
