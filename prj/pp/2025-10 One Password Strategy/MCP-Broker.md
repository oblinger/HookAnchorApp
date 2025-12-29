
# TL; DR.

I think One Password can create a land grab for agentic authorization that it would be poised to dominate.

**Gatekeepers**:  The decision-makers for agentic authority will be those executing those agents:  Cursor, OpenAI, Anthropic, ...
**Mindset**:  Blanket adoption of MCP shows willingness to forgo lock-in opportunities in the service of velocity.
**Problems**: 
-- Their consumer-focused, wide-ranging customer base needs to grant credentials across a broad range of apps.   Massive catch-22 adoption problem.
-- Users want to grant authority at the level of an AGENT, not at the level of a platform running them.
**Solution**: 
-- An open-source identity-authority standard that bridges from today, using OAUTH tokens.
-- Then transitions to a new open-source agentic service account model.


**THIS LAND GRAB IS GOOD FOR ONE PASSWORD**
-- Gatekeepers are willing to accept open standards that don't afford them lock-in in the service of velocity.  It makes little sense for them to select standards that provide lock-in for others.
-- Velocity and catch-22 adoption requires a bridging solution based on OAUTH, which requires repeated user authorization.
-- One Password's consumer desktop presence uniquely positions it to provide low-friction automation during the bridging period, allowing it to win the adoption war.
-- Long-term users, enterprise, and vendors all want identity management that is decoupled from agents and agentic executors.  One password is an ideal candidate for such independent management.


# Open Agent Authority Standard

One password can create a new land grab for agentic identity-authority management in which is is poised to dominate the fight.


## 1Password Wins

### 1. Anti-Lock-In Coalition
Anthropic's MCP achieved fast adoption by offering zero lock-in. Current players' desire for velocity rewards openness over walled gardens.

| Player           | Fears                                                 | What 1Password offers                            |
| ---------------- | ----------------------------------------------------- | ------------------------------------------------ |
| Anthropic/OpenAI | Single company Google/Microsoft owning agent identity | Neutral standard which 1password doesn't control |
| Enterprise       | Vendor lock-in                                        | Portable authority, single audit point           |
| Google/Microsoft | Each other                                            | Neither wins = both can play                     |
1Password's position: shepherd the standard, benefit from adoption, compete fairly on implementation.


### 2. Desktop Leverage (Bridge Phase)
During the bridging phase Agentic executors will need OAuth tokens refreshed but they aren't on the user's machine. 
1Password is. This makes 1Password essential infrastructure for all executors during OAuth-to-native transition.


### 3. Long-Term Value: Identity Server Land Grab
The standard defines the *protocol* for authority delegation - not the identity records themselves.

- Identity servers own user relationships and credentials
- Switching identity servers has real cost (re-establish all credentials)
- Enterprise will pick ONE identity server for all employees → sticky

**Competitive landscape**:

| Player | Strength | Gap |
|--------|----------|-----|
| Google | Chrome, Workspace | Only their ecosystem |
| Microsoft | Azure AD | Only their ecosystem |
| Okta | Enterprise identity | No desktop, no consumer |
| 1Password | Desktop, cross-platform vault | Smaller enterprise footprint |

1Password's opportunity: be the identity server for everything *outside* Google/Microsoft ecosystems. That surface area is large and growing.


---

## The Core Problems Address by the standard

| Problem | Who has it | Need |
|---------|-----------|------|
| **Service Account** | Enterprise | Central policy, audit, automation across all employees |
| **Authority Distribution** | Any principal | Delegate identity/authority slices to agents flexibly |

These problems don't go away. The standard solves them.

---

## The Standard

### Two Roles

| Role | Function | Examples |
|------|----------|----------|
| **Identity Server** | Holds user credentials, issues authority grants | 1Password, Google, Microsoft, Okta |
| **Agent Executor** | Runs agents, requests authority from identity servers | Anthropic, OpenAI, enterprise infra |

### What's Standardized (Open)
- Protocol for executors to request authority from identity servers
- Format for authority grants (scopes, constraints, expiry)
- How grants are verified by resources
- Revocation mechanism

### What's NOT Shared (Proprietary)
- User identity records (owned by identity server)
- Credential storage (owned by identity server)
- User relationship (owned by identity server)

**Key principle**: Interoperability without commoditization. Executors can talk to any identity server. Identity servers keep their users.

---

## Two-Level Permissions

| Level | Controls | Example |
|-------|----------|---------|
| **Resource Access** | What data can agents touch at all | "Notion read, Google Drive read/write" |
| **Agent Authority** | Which agents get which slice | "CalendarSync: Calendar read, Notion read" |

Resource access is the ceiling. Agent authority carves out narrower grants.

### Example: Individual User

```
User: alice@acme.com
Identity Server: 1Password

Resource Access:
  - Notion (read)
  - Calendar (read)

Agent Authority:
  - Agent: CalendarSync → Calendar (read), Notion (read)
  - Category: Productivity → Calendar (read)
  - Category: External-Sharing → [blocked]
```

### Example: Enterprise Admin

```
Org: acme.com
Identity Server: 1Password Enterprise

Resource Access: [admin consent]
  - Google Workspace (full)
  - Notion (full)
  - Salesforce (read)

Agent Policies:
  - Category: Productivity → all users → Workspace, Notion
  - Category: Analytics → data-team → Salesforce (read)
  - Agent: ExternalMarketingBot → [blocked org-wide]
```

---

## Governance

**Organization**: New body under existing umbrella (OpenID Foundation or similar)

**Responsibilities**:
- Define and evolve the standard
- Certify compliant identity servers and executors
- Maintain reference implementation

**Membership requirement**: Identity servers must implement the standard protocol. They keep their identity records proprietary.

---

## Why Each Player Says Yes

| Player | Value |
|--------|-------|
| 1Password | Shepherd standard, win identity server market share |
| Anthropic/OpenAI | Auth friction solved, no Google/MS dependency |
| Enterprise | Single audit point, portable if needed, policy control |
| Resources (Notion, etc.) | See specific agent, not broad broker access |
| Users | Declare authority once, works across executors |
