#!/bin/bash
# Nova Agent Runner
#
# Invokes Claude CLI with specific agent persona.
# Work happens in private space, approved code pushes to public repo.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
PRIVATE_WORKSPACE="$HOME/.nova-agents"  # Private workspace
PUBLIC_REPO="$REPO_ROOT"                 # Public repo

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Ensure private workspace exists
mkdir -p "$PRIVATE_WORKSPACE"/{inbox,outbox,logs,context}

AGENT="$1"
TASK="$2"

if [[ -z "$AGENT" ]]; then
    echo "Usage: $0 <agent> [task]"
    echo ""
    echo "Agents:"
    echo "  architect    - Design components, write specs"
    echo "  implementer  - Write code from specs"
    echo "  reviewer     - Review code quality"
    echo "  tester       - Write and run tests"
    echo "  security     - Security audit, adversarial tests"
    echo "  perf         - Performance analysis"
    echo "  docs         - Documentation"
    echo "  release      - Prepare releases"
    echo "  refactor     - Improve code structure"
    echo ""
    echo "Example:"
    echo "  $0 architect 'Design the type inference engine'"
    echo "  $0 reviewer --continue"
    echo "  $0 implementer --from-inbox"
    exit 1
fi

# Build the prompt for Claude
build_prompt() {
    local agent="$1"
    local task="$2"

    # Agent-specific system prompt
    local agent_prompt="$SCRIPT_DIR/agents/$agent.md"
    if [[ ! -f "$agent_prompt" ]]; then
        echo -e "${RED}ERROR: Agent definition not found: $agent${NC}"
        exit 1
    fi

    # Build context
    cat << EOF
You are the $agent agent for the Nova programming language project.

## Your Role
$(cat "$agent_prompt")

## Current Task
$task

## Project Context
- Repository: $PUBLIC_REPO
- Private Workspace: $PRIVATE_WORKSPACE
- Your output goes to: $PRIVATE_WORKSPACE/outbox/$agent/

## Important Rules
1. Write all work files to the PRIVATE workspace first
2. Only approved, reviewed code goes to the PUBLIC repo
3. Always write a result.json with your status and next steps
4. Reference existing code in bootstrap/src/
5. Follow existing patterns (see token.rs, span.rs for examples)

## Output Format
After completing your task, create:
1. Your artifacts (code, docs, etc.)
2. A result.json with:
   {
     "agent": "$agent",
     "task": "<task description>",
     "status": "success|failed|needs_review",
     "artifacts": ["list of files created"],
     "next_agent": "<who should process next>",
     "notes": "<any important notes>"
   }
EOF
}

# Run the agent via Claude CLI
run_claude() {
    local agent="$1"
    local task="$2"
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local session_dir="$PRIVATE_WORKSPACE/sessions/${agent}_$timestamp"

    mkdir -p "$session_dir"

    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  NOVA AGENT: ${GREEN}$agent${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}Task:${NC} $task"
    echo -e "${YELLOW}Session:${NC} $session_dir"
    echo ""

    # Prepare the prompt
    local prompt_file="$session_dir/prompt.md"
    build_prompt "$agent" "$task" > "$prompt_file"

    # Create a command file for Claude
    local cmd_file="$session_dir/command.txt"
    cat > "$cmd_file" << EOF
Read the agent instructions and complete the task.
Work in the Nova repository at: $PUBLIC_REPO
Write results to: $PRIVATE_WORKSPACE/outbox/$agent/

Task: $task

When done, write your result.json to indicate completion and next steps.
EOF

    echo -e "${GREEN}Starting Claude CLI...${NC}"
    echo ""

    # Invoke Claude CLI
    # The agent runs in the public repo but writes intermediate files to private space
    cd "$PUBLIC_REPO"

    # Use claude with the prompt
    if command -v claude &> /dev/null; then
        claude --print "$(cat "$prompt_file")" \
               --system-prompt "You are the $agent agent. $(cat "$SCRIPT_DIR/agents/$agent.md" | head -20)" \
            || claude -p "$(cat "$cmd_file")"
    else
        echo -e "${YELLOW}Claude CLI not found. Printing prompt instead:${NC}"
        echo ""
        cat "$prompt_file"
        echo ""
        echo -e "${YELLOW}Run manually with: claude -p '$(cat "$cmd_file" | tr '\n' ' ')'${NC}"
    fi
}

# Check for pending work in inbox
check_inbox() {
    local agent="$1"
    local inbox="$PRIVATE_WORKSPACE/inbox/$agent"

    if [[ -d "$inbox" ]] && [[ -n "$(ls -A "$inbox" 2>/dev/null)" ]]; then
        echo -e "${YELLOW}Pending work in inbox:${NC}"
        ls -la "$inbox"
        return 0
    else
        echo "No pending work in inbox"
        return 1
    fi
}

# Main execution
case "$TASK" in
    --status)
        check_inbox "$AGENT"
        ;;
    --from-inbox)
        inbox="$PRIVATE_WORKSPACE/inbox/$AGENT"
        if [[ -d "$inbox" ]] && [[ -n "$(ls -A "$inbox" 2>/dev/null)" ]]; then
            task_file=$(ls -t "$inbox"/*.json 2>/dev/null | head -1)
            if [[ -f "$task_file" ]]; then
                task=$(cat "$task_file" | jq -r '.task // .description // "Process inbox item"')
                run_claude "$AGENT" "$task"
                mv "$task_file" "$PRIVATE_WORKSPACE/logs/processed_$(basename "$task_file")"
            fi
        else
            echo "No tasks in inbox"
        fi
        ;;
    --continue)
        # Continue from last session
        last_session=$(ls -td "$PRIVATE_WORKSPACE/sessions/${AGENT}_"* 2>/dev/null | head -1)
        if [[ -d "$last_session" ]]; then
            echo "Continuing from: $last_session"
            run_claude "$AGENT" "Continue from previous session"
        else
            echo "No previous session found"
        fi
        ;;
    *)
        if [[ -n "$TASK" ]]; then
            run_claude "$AGENT" "$TASK"
        else
            echo "No task specified. Use --from-inbox or provide a task."
        fi
        ;;
esac
