# MCP Open Source Authentication Strategy

# PARTS
## Players

| Player | Description |
|--------|-------------|
| **AI Platform** | LLM providers (Anthropic, OpenAI, Google) |
| **Tool Vendor** | MCP server/tool developers |
| **Integrator** | Companies building AI agent solutions |
| **Enterprise** | Organizations deploying AI agents |
| **End User** | Humans whose credentials agents act on behalf of |
| **Identity Provider** | OAuth/SSO providers (Google, Microsoft, Okta) |

## Key Value Propositions

| Stakeholder | Primary Value |
|-------------|---------------|
| AI Platform | Standard auth layer reduces liability, enables enterprise adoption |
| Tool Vendor | Single auth integration vs N identity providers |
| Integrator | Faster time-to-market, reduced security burden |
| Enterprise | Audit trail, credential governance, revocation control |
| End User | Consent visibility, credential delegation without exposure |
| 1Password | Market positioning as neutral trust broker in AI ecosystem |


## Gatekeepers (Who Decides Standards)

| Gatekeeper | Leverage Point |
|------------|----------------|
| **IETF/OAuth WG** | Technical standards body - need their blessing for legitimacy |
| **OpenID Foundation** | Owns OAuth extensions - could host MCP-Auth spec |
| **Google/Microsoft** | Control majority of IdP market - adoption requires their buy-in |
| **Anthropic/OpenAI** | Define MCP/agent protocols - can mandate auth requirements |
| **Enterprise Security** | CISOs won't deploy without standards compliance |



# ANALYSIS

## Where Do MCP Agents Actually Run?

| Execution Model | Who Runs Agent | Example |
|-----------------|----------------|---------|
| **User's machine** | User directly | Claude Code on laptop, Cursor |
| **AI Platform cloud** | OpenAI/Anthropic | ChatGPT plugins, Claude.ai |
| **SaaS provider** | Tool vendor | Notion AI, Slack AI |
| **Enterprise cloud** | Company IT | Self-hosted agent infrastructure |
| **Third-party agent host** | Agent-as-a-service | Zapier, Make, custom integrators |

**Key insight**: MCP agents run in *many* places. This is the problem.

## OAuth Party Analysis

In OAuth, two parties negotiate:
- **Resource Server** - holds the data (e.g., Notion, Google Drive)
- **Client** - wants access to the data (needs to be registered app)

### The Identity Crisis

| Scenario | OAuth Client Would Be | Problem |
|----------|----------------------|---------|
| Claude Code on laptop | User's machine? | Not a registered app |
| ChatGPT calling Notion | OpenAI | User didn't consent to OpenAI having their Notion |
| Enterprise agent | Company's agent infra | Every company registers as OAuth client? |
| Third-party agent | That third party | User doesn't know/trust them |

### Current Workarounds (All Bad)

1. **API Keys** - User copies key, agent uses it. No scoping, no revocation, leaked constantly
2. **Service Accounts** - Enterprise-only, doesn't help individuals
3. **Personal Access Tokens** - Same problems as API keys
4. **Agent registers as OAuth app** - Every agent dev registers with every service? Doesn't scale

## The Missing Entity

OAuth assumes: `User → App → Resource`

Agents need: `User → Agent Runtime → AI Model → Tool → Resource`

**Who is the OAuth "Client" here?** None of these cleanly map:
- The AI model provider? (User didn't consent to give Anthropic their data)
- The agent runtime? (Could be user's laptop)
- The tool/MCP server? (Intermediary, not the requestor)

**This is the gap.** OAuth has no concept of "delegated autonomous access on behalf of user, executed by untrusted intermediary."

# STRATEGY
## The OAuth Problem

OAuth **by design** prevents automation:
- User must see both parties (data holder + requestor)
- User must manually approve the sharing
- No programmatic "click" allowed - that's the security model

This works for apps. Breaks for agents acting autonomously.

## Two-Phase Strategy

### Phase 1: Bridge (Work within OAuth)
Build value **on top of** existing OAuth:
- 1Password becomes the "consent registry" - user pre-approves agent scopes
- When agent needs access, 1Password holds tokens user already granted
- Agent requests from 1Password, not from IdP directly
- Audit/revoke layer sits in 1Password

**Key insight**: User does OAuth dance once with 1Password. 1Password brokers to agents.

### Phase 2: New Standard (MCP-Auth)
Once adoption proves value, propose open standard that:
- Formalizes agent delegation model
- Gets buy-in from gatekeepers
- Becomes the "OAuth for agents"

## Adoption Path

```
1Password value today     →  Proves model works  →  Propose standard
(OAuth bridge layer)         (adoption/traction)     (gatekeepers adopt)
```

**Target first**: Anthropic (owns MCP), then enterprise CISOs (budget holders).