#!/bin/bash
# Nova Multi-Agent Coordinator
#
# Orchestrates the agent pipeline with ACCOUNTABILITY.
# Every agent is tracked. Every failure is recorded.
# Trust is earned. Penalties are enforced.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Directories
INBOX_DIR="$SCRIPT_DIR/inbox"
OUTBOX_DIR="$SCRIPT_DIR/outbox"
AGENTS_DIR="$SCRIPT_DIR/agents"
STATE_DIR="$SCRIPT_DIR/shared/state"
LOGS_DIR="$SCRIPT_DIR/logs/$(date +%Y-%m-%d)"

mkdir -p "$LOGS_DIR" "$STATE_DIR"

# State files
SCOREBOARD="$STATE_DIR/scoreboard.json"

log() {
    local level="$1"
    local msg="$2"
    local timestamp=$(date +"%H:%M:%S")
    echo -e "${CYAN}[$timestamp]${NC} ${level}: $msg"
    echo "[$timestamp] $level: $msg" >> "$LOGS_DIR/coordinator.log"
}

# Show the mission
show_mission() {
    echo -e "\n${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}                    THE NOVA MISSION${NC}"
    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

    echo -e "${BOLD}Nova will be the last programming language anyone needs to learn.${NC}\n"

    # Show progress
    if [[ -f "$SCOREBOARD" ]]; then
        local phase=$(jq -r '.mission_progress.phase' "$SCOREBOARD")
        local milestone=$(jq -r '.mission_progress.milestone' "$SCOREBOARD")
        local completed=$(jq -r '.mission_progress.completed | length' "$SCOREBOARD")

        echo -e "  ${YELLOW}Phase:${NC}     $phase"
        echo -e "  ${YELLOW}Milestone:${NC} $milestone"
        echo -e "  ${YELLOW}Completed:${NC} $completed components"
    fi

    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}\n"
}

# Show scoreboard
show_scoreboard() {
    echo -e "\n${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}                    AGENT SCOREBOARD${NC}"
    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

    if [[ ! -f "$SCOREBOARD" ]]; then
        echo "  No scoreboard data yet."
        return
    fi

    printf "  ${BOLD}%-12s │ Level │ Score │ Tasks │ Streak │ Status${NC}\n" "Agent"
    echo "  ─────────────┼───────┼───────┼───────┼────────┼────────"

    for agent in architect implementer reviewer tester security perf docs refactor release; do
        local level=$(jq -r ".agents.$agent.trust_level" "$SCOREBOARD")
        local score=$(jq -r ".agents.$agent.trust_score" "$SCOREBOARD")
        local tasks=$(jq -r ".agents.$agent.tasks_completed" "$SCOREBOARD")
        local streak=$(jq -r ".agents.$agent.current_streak" "$SCOREBOARD")
        local status=$(jq -r ".agents.$agent.status" "$SCOREBOARD")

        # Color based on level
        local level_color="${GREEN}"
        [[ "$level" == "0" ]] && level_color="${RED}"
        [[ "$level" == "1" ]] && level_color="${YELLOW}"

        # Status indicator
        local status_icon="○"
        [[ "$status" == "working" ]] && status_icon="${YELLOW}●${NC}"
        [[ "$status" == "failed" ]] && status_icon="${RED}✗${NC}"

        printf "  %-12s │ ${level_color}%5s${NC} │ %5s │ %5s │ %6s │ %s\n" \
            "$agent" "$level" "$score" "$tasks" "$streak" "$status_icon $status"
    done

    echo -e "\n  ${BOLD}Trust Levels:${NC} 0=PROBATION  1=JUNIOR  2=TRUSTED  3=SENIOR"
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}\n"
}

# Record task result
record_result() {
    local agent="$1"
    local task_id="$2"
    local result="$3"  # success, critical, major, minor
    local notes="$4"

    if [[ ! -f "$SCOREBOARD" ]]; then
        log "${RED}ERROR${NC}" "Scoreboard not found"
        return 1
    fi

    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

    # Calculate score change
    local score_change=10
    case "$result" in
        success)    score_change=10 ;;
        minor)      score_change=-5 ;;
        major)      score_change=-25 ;;
        critical)   score_change=-100 ;;
    esac

    # Update scoreboard
    local current_score=$(jq -r ".agents.$agent.trust_score" "$SCOREBOARD")
    local new_score=$((current_score + score_change))

    local current_tasks=$(jq -r ".agents.$agent.tasks_completed" "$SCOREBOARD")
    local current_streak=$(jq -r ".agents.$agent.current_streak" "$SCOREBOARD")

    if [[ "$result" == "success" ]]; then
        current_tasks=$((current_tasks + 1))
        current_streak=$((current_streak + 1))
    else
        current_streak=0
        # Increment failure counter
        jq ".agents.$agent.${result}_failures += 1" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"
    fi

    # Calculate new trust level
    local new_level=0
    [[ $new_score -ge 50 ]] && new_level=1
    [[ $new_score -ge 200 ]] && new_level=2
    [[ $new_score -ge 500 ]] && new_level=3

    # Update scoreboard
    jq ".agents.$agent.trust_score = $new_score |
        .agents.$agent.trust_level = $new_level |
        .agents.$agent.tasks_completed = $current_tasks |
        .agents.$agent.current_streak = $current_streak |
        .agents.$agent.status = \"idle\" |
        .last_updated = \"$timestamp\"" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"

    # Log the result
    if [[ "$result" == "success" ]]; then
        log "${GREEN}SUCCESS${NC}" "$agent completed $task_id (score: $current_score → $new_score)"
    else
        log "${RED}FAILURE${NC}" "$agent FAILED $task_id with $result failure (score: $current_score → $new_score)"

        # Record failure
        jq ".failures += [{
            \"timestamp\": \"$timestamp\",
            \"agent\": \"$agent\",
            \"task\": \"$task_id\",
            \"severity\": \"$result\",
            \"notes\": \"$notes\"
        }]" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"

        # Check for demotion
        if [[ "$result" == "critical" ]]; then
            log "${RED}DEMOTION${NC}" "$agent demoted to PROBATION due to critical failure"
            jq ".agents.$agent.trust_level = 0 | .agents.$agent.flags += [\"probation\"]" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"
        fi
    fi
}

# Check if agent can work autonomously
check_autonomy() {
    local agent="$1"
    local task_type="$2"  # routine, complex, approval

    local level=$(jq -r ".agents.$agent.trust_level" "$SCOREBOARD")

    case "$task_type" in
        routine)
            [[ $level -ge 1 ]] && return 0 ;;
        complex)
            [[ $level -ge 2 ]] && return 0 ;;
        approval)
            [[ $level -ge 3 ]] && return 0 ;;
    esac

    return 1
}

# Run agent with accountability
run_agent() {
    local agent="$1"
    local task_file="$2"

    # Check agent exists
    if [[ ! -f "$AGENTS_DIR/$agent.md" ]]; then
        log "${RED}ERROR${NC}" "Agent definition not found: $agent.md"
        return 1
    fi

    # Get trust level
    local level=$(jq -r ".agents.$agent.trust_level" "$SCOREBOARD" 2>/dev/null || echo "1")
    local level_name="JUNIOR"
    [[ "$level" == "0" ]] && level_name="${RED}PROBATION${NC}"
    [[ "$level" == "2" ]] && level_name="${GREEN}TRUSTED${NC}"
    [[ "$level" == "3" ]] && level_name="${BLUE}SENIOR${NC}"

    log "${BLUE}AGENT${NC}" "Starting $agent (Level: $level_name)"

    # Mark agent as working
    jq ".agents.$agent.status = \"working\"" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"

    # Build context with mission and accountability
    local context_file="$LOGS_DIR/${agent}_context_$(date +%H%M%S).md"

    cat > "$context_file" << EOF
# NOVA MISSION CONTEXT

$(cat "$SCRIPT_DIR/MISSION.md")

---

# ACCOUNTABILITY REQUIREMENTS

$(cat "$SCRIPT_DIR/ACCOUNTABILITY.md" | head -100)

---

# YOUR CURRENT STATUS

- **Agent**: $agent
- **Trust Level**: $level ($level_name)
- **Tasks Completed**: $(jq -r ".agents.$agent.tasks_completed" "$SCOREBOARD")
- **Current Streak**: $(jq -r ".agents.$agent.current_streak" "$SCOREBOARD")
- **Flags**: $(jq -r ".agents.$agent.flags | join(\", \")" "$SCOREBOARD")

**Remember**: Your trust score depends on the quality of your work.
- Success: +10 points
- Minor failure: -5 points
- Major failure: -25 points
- Critical failure: -100 points (immediate PROBATION)

---

# AGENT INSTRUCTIONS

$(cat "$AGENTS_DIR/$agent.md")

---

# CURRENT TASK

$(cat "$task_file" 2>/dev/null || echo "No specific task file provided")

---

# QUALITY GATE CHECKLIST

Before submitting your work, verify:

□ Does it compile/run without errors?
□ Do all tests pass?
□ Did I actually test it myself?
□ Am I proud of this work?
□ Would I bet my trust score on this?

**If any answer is NO, fix it before submitting.**
EOF

    echo -e "\n${GREEN}Agent $agent ready to work${NC}"
    echo -e "Context: $context_file"
    echo -e "Task: ${task_file:-'Interactive mode'}\n"

    # The actual Claude invocation would happen here
    # For now, we prepare the context
    echo "Ready to invoke: claude -p \"$(head -5 "$context_file" | tr '\n' ' ')...\""
}

# Route completed work with accountability check
route_to_next() {
    local from_agent="$1"
    local result_file="$2"
    local next_agent=""

    # Check result status
    local status=$(jq -r '.status' "$result_file" 2>/dev/null || echo "unknown")

    case "$from_agent" in
        architect)    next_agent="implementer" ;;
        implementer)  next_agent="reviewer" ;;
        reviewer)
            if [[ "$status" == "approved" ]]; then
                next_agent="tester"
                record_result "implementer" "$(jq -r '.task_id' "$result_file")" "success" "Approved by reviewer"
            else
                next_agent="implementer"
                record_result "implementer" "$(jq -r '.task_id' "$result_file")" "major" "Rejected by reviewer"
            fi
            ;;
        tester)
            if [[ "$status" == "passed" ]]; then
                next_agent="security"
                record_result "tester" "$(jq -r '.task_id' "$result_file")" "success" "Tests passed"
            else
                next_agent="implementer"
                record_result "implementer" "$(jq -r '.task_id' "$result_file")" "major" "Tests failed"
            fi
            ;;
        security)
            local risk=$(jq -r '.risk_level' "$result_file" 2>/dev/null || echo "low")
            if [[ "$risk" == "critical" || "$risk" == "high" ]]; then
                next_agent="implementer"
                record_result "implementer" "$(jq -r '.task_id' "$result_file")" "critical" "Security vulnerability"
            else
                next_agent="perf"
                record_result "security" "$(jq -r '.task_id' "$result_file")" "success" "Security approved"
            fi
            ;;
        perf)         next_agent="docs" ;;
        docs)         next_agent="release" ;;
        release)      next_agent="" ;;
    esac

    if [[ -n "$next_agent" ]]; then
        log "${YELLOW}ROUTE${NC}" "Routing from $from_agent to $next_agent"
        cp "$result_file" "$INBOX_DIR/$next_agent/"
    else
        log "${GREEN}COMPLETE${NC}" "Pipeline complete for task"

        # Update mission progress
        local task=$(jq -r '.task' "$result_file")
        jq ".mission_progress.completed += [\"$task\"]" "$SCOREBOARD" > "$SCOREBOARD.tmp" && mv "$SCOREBOARD.tmp" "$SCOREBOARD"
    fi
}

# Check inbox for pending work
check_inbox() {
    local agent="$1"
    local inbox="$INBOX_DIR/$agent"

    if [[ -d "$inbox" ]]; then
        local pending=$(find "$inbox" -name "*.json" -o -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
        echo "$pending"
    else
        echo "0"
    fi
}

# Show status with accountability
show_status() {
    show_mission
    show_scoreboard

    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}                    PENDING WORK${NC}"
    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

    local total_pending=0
    for agent in architect implementer reviewer tester security perf docs refactor release; do
        local pending=$(check_inbox "$agent")
        total_pending=$((total_pending + pending))

        local status_color="${GREEN}"
        [[ "$pending" -gt 0 ]] && status_color="${YELLOW}"
        [[ "$pending" -gt 5 ]] && status_color="${RED}"

        printf "  %-15s │ Pending: ${status_color}%d${NC}\n" "$agent" "$pending"
    done

    echo -e "\n  ${BOLD}Total pending:${NC} $total_pending tasks"
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

    # Show recent failures if any
    if [[ -f "$SCOREBOARD" ]]; then
        local failures=$(jq -r '.failures | length' "$SCOREBOARD")
        if [[ "$failures" -gt 0 ]]; then
            echo -e "${RED}${BOLD}RECENT FAILURES:${NC}"
            jq -r '.failures[-3:] | .[] | "  [\(.severity)] \(.agent): \(.notes)"' "$SCOREBOARD"
            echo ""
        fi
    fi
}

# Main
main() {
    case "${1:-status}" in
        mission)
            show_mission
            ;;
        scoreboard)
            show_scoreboard
            ;;
        status)
            show_status
            ;;
        run)
            if [[ -z "$2" ]]; then
                echo "Usage: $0 run <agent> [task_file]"
                exit 1
            fi
            run_agent "$2" "${3:-}"
            ;;
        record)
            if [[ -z "$2" || -z "$3" || -z "$4" ]]; then
                echo "Usage: $0 record <agent> <task_id> <result> [notes]"
                echo "  result: success, minor, major, critical"
                exit 1
            fi
            record_result "$2" "$3" "$4" "${5:-}"
            ;;
        route)
            if [[ -z "$2" || -z "$3" ]]; then
                echo "Usage: $0 route <from_agent> <result_file>"
                exit 1
            fi
            route_to_next "$2" "$3"
            ;;
        watch)
            log "${BLUE}INFO${NC}" "Starting watch mode..."
            while true; do
                for agent in architect implementer reviewer tester security perf docs refactor release; do
                    local pending=$(check_inbox "$agent")
                    if [[ "$pending" -gt 0 ]]; then
                        log "${YELLOW}PENDING${NC}" "$agent has $pending tasks"
                        local task=$(find "$INBOX_DIR/$agent" -name "*.json" 2>/dev/null | head -1)
                        if [[ -n "$task" ]]; then
                            run_agent "$agent" "$task"
                        fi
                    fi
                done
                sleep 5
            done
            ;;
        *)
            echo -e "${BOLD}Nova Multi-Agent Coordinator${NC}"
            echo ""
            echo "Usage: $0 <command>"
            echo ""
            echo "Commands:"
            echo "  mission       Show the mission"
            echo "  scoreboard    Show agent scores and trust levels"
            echo "  status        Show full status (mission + scoreboard + pending)"
            echo "  run <agent>   Run specific agent"
            echo "  record        Record task result (success/failure)"
            echo "  route         Route result to next agent"
            echo "  watch         Watch mode (continuous)"
            ;;
    esac
}

main "$@"
