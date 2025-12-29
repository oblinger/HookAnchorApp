# MCP Bridge: 1Password as OAuth Broker

## The Setup

- Enterprise has 1Password deployed to all employee desktops
- Enterprise configures: "These agent classes can run for these user groups"
- 1Password client knows which OAuth tokens each user needs
- 1Password can pop up auth screens when tokens need refresh/creation

## The OAuth Consent Question

When 1Password pops up OAuth, user sees:
> "Allow **X** to access your **Y** data?"

**Who is X? Who is Y?**

## Scenario Analysis

### Scenario A: Claude Code on laptop wants to read user's Notion

| Component | Where it runs | Identity |
|-----------|---------------|----------|
| Agent runtime | User's laptop | (local process) |
| AI Model | Anthropic cloud | Anthropic |
| MCP Server | User's laptop | (local process) |
| Resource | Notion cloud | Notion |

**OAuth parties:**
- Y (Resource Server) = Notion ✓ clear
- X (Client) = ???
  - Not Anthropic (model doesn't touch Notion directly)
  - Not "user's laptop" (not a registered app)
  - The MCP server? (just a local proxy)

**Current hack**: User creates Notion "integration", copies API key, pastes into config. No OAuth at all.

### Scenario B: ChatGPT with Notion plugin

| Component | Where it runs | Identity |
|-----------|---------------|----------|
| Agent runtime | OpenAI cloud | OpenAI |
| AI Model | OpenAI cloud | OpenAI |
| Plugin/Tool | OpenAI cloud | OpenAI |
| Resource | Notion cloud | Notion |

**OAuth parties:**
- Y = Notion ✓
- X = OpenAI ✓ (registered OAuth app with Notion)

**This works!** But user is consenting to "OpenAI can access my Notion" - broad, permanent, uncomfortable.

### Scenario C: Enterprise self-hosted agent accessing Google Drive

| Component | Where it runs | Identity |
|-----------|---------------|----------|
| Agent runtime | Company AWS | Acme Corp |
| AI Model | Anthropic API | Anthropic |
| MCP Server | Company AWS | Acme Corp |
| Resource | Google Workspace | Google |

**OAuth parties:**
- Y = Google ✓
- X = Acme Corp (registers as OAuth app with Google)

**Works for enterprise**, but every company must register OAuth apps with every service.

## The Pattern

OAuth works when there's a **single, identifiable, registered entity** making the request.

| Scenario | Has clear OAuth Client? | Works? |
|----------|------------------------|--------|
| Cloud-hosted agent (OpenAI/plugin model) | Yes - the platform | ✓ but over-broad |
| Enterprise self-hosted | Yes - the company | ✓ but setup burden |
| User's local machine | **No** | ✗ |

## 1Password Bridge Proposal

**1Password becomes the registered OAuth Client.**

```
User → 1Password → [any agent runtime] → Resource
         ↑
    Registered OAuth app with Notion, Google, etc.
```

### OAuth Flow with Bridge

1. User configures agent that needs Notion access
2. 1Password detects: "This agent needs Notion read scope"
3. 1Password pops OAuth: "Allow **1Password** to access your **Notion**?"
4. User approves (trusts 1Password)
5. 1Password holds the token
6. When agent runs, it requests token from 1Password
7. 1Password issues **scoped, time-limited, audited** sub-token to agent

### What User Sees

> "Allow **1Password Agent Broker** to access your **Notion** workspace?"
>
> Scopes requested:
> - Read pages ✓
> - Read databases ✓
>
> This will enable authorized agents to access Notion on your behalf.

### The Key Shift

| Without Bridge | With Bridge |
|----------------|-------------|
| User consents to unknown/many OAuth clients | User consents once to 1Password |
| Each agent needs own OAuth registration | Agents request from 1Password |
| No visibility into agent access | 1Password logs all agent access |
| Revoke = find every integration | Revoke = one place |

## Open Questions

1. **Will resource providers (Notion, Google) allow this?**
   - 1Password is essentially acting as OAuth intermediary
   - Some providers may prohibit token sharing in ToS
   - Need partnership or new OAuth extension

2. **How does agent prove identity to 1Password?**
   - Local agent: process attestation? signed binary?
   - Cloud agent: API key? mTLS cert?
   - This is the "sub-delegation" problem

3. **Enterprise admin control?**
   - Admin configures: "Only these agents can access Google Drive"
   - Admin configures: "Only read scope, never write"
   - 1Password enforces policy at token-issuance time

4. **Individual vs Enterprise**
   - Enterprise: Admin sets policy, users follow
   - Individual: User is own admin, more self-service UX needed
