Implement a task issue (work item or research task).

1. Find the `Task` issue:

   1. If the user provided a link to a GitHub issue, read it:
      - It should be a `Task` issue. If not: STOP and tell the user.
      - Determine if this is a regular task or a research task by checking for the `research` label.
      - Find the parent `Feature` issue (the 'root' issue) from the task's sub-issue relationship.

   2. If the user did not provide a link to a GitHub issue:
      - Ask them to provide the Task issue URL.

2. For research tasks (labeled with `research`):
   1. Follow the research methodology described in the issue.
   2. Document findings in the "Findings" section of the issue.
   3. Analyze options and trade-offs, updating the "Options & Trade-offs" section.
   4. Provide a clear recommendation with rationale in the "Recommendation" section.
   5. If the research reveals blockers or changes requirements:
      - Update the parent issue (Spec or Design) with the findings.
      - If this affects other tasks, note dependencies in comments.
   6. Complete the research:
      - Update the issue with all findings and recommendations.
      - Close the research task issue.
      - Inform the user that research is complete and ready for integration.

3. For regular tasks (implementation work):

   1. Set up the git branch:
      - Determine the branch name: `<feature-issue-number>-<feature-name>/<task-issue-number>-<task-name>` (use kebab-case for names)
      - Check if this task depends on other tasks that haven't been merged to `main`:
        - If yes: branch off the dependent task's branch
        - If no: branch off `main`
      - Create and switch to the new branch
   2. Review the acceptance criteria and technical notes.

   3. Understand the codebase context:
      - Search for relevant existing code and patterns.
      - Review related files mentioned in technical notes.
      - Check dependencies on other tasks.

   4. Implement the solution:
      - Follow existing code conventions and patterns.
      - Write clean, well-structured code.
      - Include appropriate error handling.
      - Add tests if the project has testing infrastructure.
      - Make incremental commits as you progress through the implementation:
        - Commit logical units of work (e.g., after adding a function, fixing a bug, adding tests)
        - Use descriptive commit messages that reference the task issue
        - Push commits regularly to backup your work

   5. Validate against acceptance criteria:
      - Test each criterion from the acceptance criteria list.
      - Document any deviations or limitations.
      - Run relevant build/test commands to ensure no regressions.

   6. Complete the task:
      - Ensure all changes are committed and pushed.
      - Read the [PR template](/.github/pull_request_template.md)
      - Create a pull request following that template:
        - If this task branched off `main`: base the PR against `main`
        - If this task branched off another task's branch: base the PR against that task's branch (creating a PR stack)
        - Title the PR with the task name and reference the task issue
        - Fill out the PR template completely:
          - Fill in relevant issue/PR links at the top of the PR body.
          - Copy acceptance criteria from the task issue and mark completed items
          - Document changes made and testing performed
          - Note any dependencies on other PRs
      - If this task enables other blocked tasks, notify about unblocked dependencies.

4. Handle complications:
   1. If acceptance criteria are unclear or incomplete:
      - Ask the user for clarification on specific points.
      - Update the task issue with clarified criteria.
   2. If implementation reveals scope creep or missing requirements:
      - Document the additional work needed.
      - Ask the user whether to expand the current task or create new tasks.
   3. If technical blockers arise:
      - Document the blocker clearly in the task issue.
      - If it's a research question: create a research sub-task.
      - If it's a dependency: identify what needs to be completed first.
      - PAUSE implementation and inform the user of the blocker.

5. Final steps:
   - Update the parent `Feature` issue if this task represents significant progress.
   - If this was the last task for the feature, inform the user that the feature may be ready for review.

That's it.
