import logging
import os
import time
from typing import Dict, List

from openrouter import OpenRouter

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%H:%M:%S",
)
log = logging.getLogger(__name__)


def get_client():
    """Get OpenRouter client"""
    log.info("Initializing OpenRouter client")
    return OpenRouter(api_key=os.getenv("OPENROUTER_API_KEY"))


def call_llm(client: OpenRouter, model: str, prompt: str, name: str) -> str:
    """Call an LLM and return response"""
    log.info(f"Calling {name} ({model})...")
    start_time = time.time()
    response = client.chat.send(
        model=model, messages=[{"role": "user", "content": prompt}]
    )
    elapsed = time.time() - start_time
    content = response.choices[0].message.content
    log.info(f"Response from {name}: {len(content)} chars in {elapsed:.1f}s")
    return content


def phase_1_exploration(client: OpenRouter, problem: str) -> List[Dict]:
    """Multiple models propose different approaches"""
    log.info("=" * 60)
    log.info("PHASE 1: EXPLORATION - Each model proposes an approach")
    log.info("=" * 60)

    explorers = [
        {
            "name": "Claude Sonnet (Minimalist)",
            "model": "anthropic/claude-sonnet-4.5",
        },
        {
            "name": "GPT-5 (Feature-Rich)",
            "model": "openai/gpt-5-codex",
        },
    ]

    prompt = f"""Given this problem:

{problem}

Propose ONE specific design approach. Be opinionated and make clear trade-offs.

Include:
1. **Core Concept** (2-3 sentences max)
2. **Key Trade-offs** (What are you sacrificing for simplicity?)
3. **What's Explicitly Excluded** (and why)
4. **Concrete Example** (show actual usage)
5. **Implementation Sketch** (high-level, 5-10 bullet points)

Make your proposal concrete and different from what others might suggest.
Focus on the PROBLEM, not on showing off features.
"""

    proposals = []
    for explorer in explorers:
        response = call_llm(client, explorer["model"], prompt, explorer["name"])
        proposals.append(
            {
                "name": explorer["name"],
                "model": explorer["model"],
                "content": response,
            }
        )
        print(f"\n{'=' * 60}")
        print(f"{explorer['name']} Proposal")
        print(f"{'=' * 60}")
        print(response)

    return proposals


def phase_2_critique(client: OpenRouter, problem: str, proposals: List[Dict]) -> str:
    """One model critiques all proposals"""
    log.info("=" * 60)
    log.info("PHASE 2: CRITIQUE - Analyzing proposals")
    log.info("=" * 60)

    proposals_text = "\n\n".join(f"### {p['name']}\n{p['content']}" for p in proposals)

    prompt = f"""Problem we're solving:
{problem}

Different design proposals have been made:

{proposals_text}

Critically analyze these proposals:

1. **Strengths**: What does each proposal do well?
2. **Weaknesses**: What problems does each have?
3. **Conflicts**: Where do they disagree? Which disagreements matter?
4. **Gaps**: What's missing from ALL proposals?
5. **Recommendations**: Which ideas should definitely be included? Which should be dropped?

Be specific and constructive. Focus on helping reach a SIMPLE, WORKING solution.
"""

    critique = call_llm(
        client, "deepseek/deepseek-v3.2-speciale", prompt, "DeepSeek Critic"
    )

    print(f"\n{'=' * 60}")
    print("CRITIQUE")
    print(f"{'=' * 60}")
    print(critique)

    return critique


def phase_3_synthesis(
    client: OpenRouter, problem: str, proposals: List[Dict], critique: str
) -> str:
    """Synthesize final design from all inputs"""
    log.info("=" * 60)
    log.info("PHASE 3: SYNTHESIS - Creating final design")
    log.info("=" * 60)

    proposals_summary = "\n".join(
        f"- {p['name']}: {p['content'][:150].replace(chr(10), ' ')}..."
        for p in proposals
    )

    prompt = f"""Problem:
{problem}

Proposals made:
{proposals_summary}

Critique received:
{critique}

Create a FINAL DESIGN that:
1. Takes the best ideas from the proposals
2. Addresses the critique's concerns
3. Makes explicit trade-offs
4. IS ACTUALLY SIMPLE (complexity = failure for this problem)

Output a complete design document including:
- Overview (what it is, what it isn't)
- Core concepts
- Configuration format (with concrete example)
- Usage examples

Be concrete and actionable. This should be something someone can start building.
"""

    final_design = call_llm(
        client, "anthropic/claude-sonnet-4.5", prompt, "Final Synthesis"
    )

    print(f"\n{'=' * 60}")
    print("FINAL DESIGN")
    print(f"{'=' * 60}")
    print(final_design)

    return final_design


def save_session(problem: str, proposals: List[Dict], critique: str, final: str):
    """Save entire session to markdown"""
    log.info("Saving session to design_board_session.md")

    with open("design_board_session.md", "w") as f:
        f.write("# Design Board Session\n\n")

        f.write("## Problem Statement\n\n")
        f.write(problem)
        f.write("\n\n---\n\n")

        f.write("## Phase 1: Exploration\n\n")
        for p in proposals:
            f.write(f"### {p['name']}\n\n")
            f.write(p["content"])
            f.write("\n\n---\n\n")

        f.write("## Phase 2: Critique\n\n")
        f.write(critique)
        f.write("\n\n---\n\n")

        f.write("## Phase 3: Final Design\n\n")
        f.write(final)

    with open("design_final.md", "w") as f:
        f.write(final)

    log.info("Session saved to design_board_session.md")
    log.info("Final design saved to design_final.md")


def main():
    problem = """
# Problem: Dead-Simple Alternative to Ansible

## Current Situation
I manage ~5-10 personal Linux servers (Ubuntu + Alpine mix).

## Pain Points with Ansible
- Too complex for simple personal use
- YAML hell: playbooks, roles, inventory, galaxy
- Steep learning curve for basic tasks
- Overkill for what I need

## What I Actually Need

### Regular Tasks
1. **System Updates**: Run `apt update && apt upgrade` or `apk update && apk upgrade` weekly
2. **Dotfile Deployment**: Deploy my shell configs (fish, tmux, starship) to new/existing machines
3. **Package Management**: Ensure certain packages are installed
4. **Config Updates**: When I update my dotfiles, push changes to servers

### Constraints
- Single user (me)
- Written in Rust using (mininija, argh, thiserror, basic-toml)
- Mixed OS environments (Ubuntu, Alpine, maybe macOS later)
- Must work over SSH (no agents on targets)
- Configuration should be one file and inventory in another.
- Don't want to learn a complex DSL

Current configuration layout:

```repo/
  ├── manifests
  │   ├── dev-env.toml
  │   └── os-settings.toml
  ├── nodes
  │   ├── dev.toml
  │   ├── home-lab.toml
  │   └── prod.toml
  └── templates
      ├── fish
      │   └── config.fish.j2
      ├── starship
      │   └── starship.toml
      └── tmux
          └── tmux.conf.j2
```

### Non-Requirements
- Don't need: multi-user/team features, compliance reporting, complex orchestration
- Don't need: Windows support, cloud provider integrations
- Don't need: dynamic inventory, service discovery

## Success Criteria
1. Can deploy run harddots apply -f manifests/home-lab.toml and it installs the tools and renders the templates.
"""

    log.info("=" * 60)
    log.info("Design Board Starting")
    log.info("=" * 60)

    overall_start = time.time()
    client = get_client()

    # Phase 1: Get different proposals
    proposals = phase_1_exploration(client, problem)

    # Phase 2: Critique them
    critique = phase_2_critique(client, problem, proposals)

    # Phase 3: Synthesize final design
    final_design = phase_3_synthesis(client, problem, proposals, critique)

    # Save everything
    save_session(problem, proposals, critique, final_design)

    overall_elapsed = time.time() - overall_start
    log.info("=" * 60)
    log.info(f"Design Board completed in {overall_elapsed:.1f}s")
    log.info("=" * 60)


if __name__ == "__main__":
    main()
