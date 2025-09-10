Break down a technical design into discrete, actionable task issues.

1. Find the `Design` issue:

   1. If the user provided a link to a GitHub issue, read it:
      - It should be a `Design` issue. If not: STOP and tell the user.
      - If it has [NEEDS CLARIFICATION] markers or open technical questions: warn the user that the design is incomplete and ask if they want to proceed with planning anyway (parallel work is allowed but may require design updates).
      - If they confirm to proceed: the design issue is our starting point.
      - Find the parent `Feature` issue (the 'root' issue) from the design's sub-issue relationship.

   2. If the user did not provide a link to a GitHub issue:
      - Ask them to provide the Design issue URL.

2. Read the [`Task` template](/.github/ISSUE_TEMPLATE/task.md)

3. Analyze the design's architecture and technical decisions:
   - Break down the technical components into discrete, actionable tasks
   - Review risks from design and create specific validation/testing tasks for mitigation strategies
   - Convert `[IMPLEMENTATION DECISION]` markers into tasks with validation criteria
   - Create "implementation validation" tasks for key technical decisions
   - Identify dependencies between tasks
   - Consider any technical decisions or architecture that impacts task structure
   - Ensure tasks are appropriately sized (not too large, not too granular)
   - For complex areas, consider creating "epic" tasks with sub-tasks underneath

4. Create task issues for each discrete work item:
   - Use the `Task` template for each issue
   - Title should be prefixed by the parent feature, separated by `:`
     - E.g. "User Authentication: Add password hashing library" for a task under "User Authentication"
   - Top-level tasks should be sub-issues of the root `Feature` issue (not the Design)
   - For "epic" tasks with multiple steps, create sub-tasks as sub-issues of the epic task
     - E.g. "User Authentication: API Implementation: Add login endpoint" as a sub-task of an "User Authentication: API Implementation" epic.
   - Include clear acceptance criteria that define "done"
   - Add technical notes for context and dependencies

5. Order tasks by dependencies:
   - Output a numbered list of all created task issues in dependency order
   - Note any tasks that can be worked in parallel
   - Identify any blocking dependencies on external work

6. Confirm the plan is complete:
   - All implementation plan steps are covered by tasks
   - Task scope is appropriate (not too big/small)
   - Dependencies are clear and achievable
   - Acceptance criteria are testable

That's it.
