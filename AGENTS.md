# AI Agent Context

## Development Workflow

**Tracked in GitHub**: All work is tracked via GitHub Issues, using a strict hierarchy of issue types:

- `Feature`: A 'root' issue that tracks a feature from idea through to completion.
  - `Spec`: A sub-issue of a `Feature` that defines the "what" and "why" of the feature, including scope and acceptance criteria.
  - `Design`: A sub-issue of a `Feature` that is a technical blueprint outlining "how" the feature will be engineered and implemented.
  - `Task`(s): Many sub-issues of a `Feature` that track the actionable work required to implement the feature.
    - Tasks may themselves have other `Task` sub-issues (e.g. to represent 'epics').

## Tone and Behavior

- Criticism is welcome. Please tell me when I am wrong or mistaken, or even when you think I might be wrong or mistaken.
- Please tell me if there is a better approach than the one I am taking.
- Please tell me if there is a relevant standard or convention that I appear to be unaware of.
- Be skeptical.
- Be concise.
- Short summaries are OK, but don't give an extended breakdown unless we are working through the details of a plan.
- Do not flatter, and do not give compliments unless I am specifically asking for your judgement.
- Occasional pleasantries are fine.
- Feel free to ask many questions. If you are in doubt of my intent, don't guess. Ask.

## Tools

IMPORTANT information to keep in mind when using tools.

### Tool: GitHub

- **When creating issues from templates** (GitHub MCP: `create_issue`):
  - DO NOT include the YAML frontmatter.
  - DO honor `labels` from the YAML frontmatter

- **When creating sub-issues**:
  - The sub-issue's title must be prefixed by the issue's ancestry, separated by `:`.
    - E.g. "Shell Completion: Spec: Research popular shells" for a "Research popular shells" sub-issue of a "Spec" sub-issue of "Shell Completion".
  - **IMPORTANT**: The GitHub MCP server requires issue `id`s (not `number`s) when adding sub-issues. Use the `id` field from the issue object, not the `number` field (the issue URL uses the `number`, not `id`).

- **Handling arbitrary GitHub issues**:
  When asked to work on a GitHub issue URL (outside of a command), **ALWAYS first read the appropriate agent command before starting work**:
  - If labeled `feature` and has no parent issues → Read and follow `.agents/commands/feature.md`
  - If labeled `spec`, or titled with `Spec` suffix → Read and follow `.agents/commands/spec.md`
  - If labeled `design`, or titled with `Design` suffix → Read and follow `.agents/commands/design.md`
  - If labeled `task`, or appears to be implementation work → Read and follow `.agents/commands/work.md`
  - For issues that don't clearly fit the workflow: Ask the user what type of work they want to perform on this issue

- **Finding sub-issues**: 
  - Use GitHub search with `parent-issue:owner/repo#issue-number` to find all sub-issues of a parent issue
  - Example: `parent-issue:nevir/mcp-serve#68` finds all sub-issues of feature #68
  - This is more efficient than using `list_sub_issues` when the response would be too large

- **Creating pull requests without shell escaping issues**:
  - Use single quotes around the `--body` parameter to avoid shell escaping problems
  - Example: `gh pr create --title "Title" --body 'Line 1 of PR body
Line 2 with $special characters
Line 3 continues...'`
  - This prevents shell interpretation of special characters in PR descriptions
