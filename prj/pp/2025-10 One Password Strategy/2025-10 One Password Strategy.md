

# WHY
- Want to stay central 
- 



# TOP IDEAS

## Want Agentic-AI w/ same permissions as its users across ALL assets
- Users want 'service account' like unification across all their apps, all agents, all contexts
- Requires "catch-all" approach - 1PW UX for managed OAUTH keys
- Requires an extension of MCP to support authority.
	- Needs an enormous span across applications
	- Foundational model providers resist 
	- open source standard to delegate existing auth - new kind of service account.
- One-Two punch for Agentic Identity     & Open Source Standard for Agentic Identity (delegated OAUTH)

## LLM-Based auto login PW reset automation
- Managed Recipes for LLM driven auto testing & refinement of

## Use of LLMs in understanding and strategizing about auto login failures / PW rotation failures


## Provide ONLY safe customer experience for agentic / ML-based companies.
- Huge range of startups hope to provide to provide agentic/ML solutions to customers but face adoption hurdles as their name is not yet trusted.
- Provide drop in replacement libraries for vibe coding that secure computing and data behind 1-password controlled servers.
	Free tier accounts for non-1password customers of their to just use their controlled services.
- Leverages existing 1Password acceptance within the SMB market to drive clients' customer adoption within their SMB customers.




## Open Source Standard for Agentic Identity
- => Agentic providers calling out for Agentic identity -- need it to enable their agents
- => Large Corp access is not the challenge, its the long tail of SMB and Personal data
- => Delegated OAUTH is the single point solution, but can only occur as an open source / open standard
- => 1Password is the natural leader and champion for such a standard.
- => Pressure from customers and agent providers will push adoption of this new standard.
- => Technical hurdles will be light -- drop in replacement for std oauth -- hurdles will be political / legal.
- => Open source would be risk for 1password, but its a bigger risk for current oauth holders.
- Bridge industry (Oauth, OIDC, Passkey) standards with centralized identity management.
- Develop drop-in replacement libraries for standard auth technologies that transparently supports centralized identity management.
- Gain momentum with key SAAS companies that want to deliver MCP servers that can be driven (without user signup) to user-proxied agentic AI
- Invite FAANG identity management to join the open standard to drive universal adoption
- 1 Password's 

## Strategy 
- Use 1password dominance in SMB to entice key Vendors serving that market to integrate drop-in replacement libraries allowing 1-password mediated agentic access.
- Add GCP, AWS, and Azure solutions allowing drop-in replacement for vendor backends build on these platforms


- Third party 


### Details


  Core Identity Standards to Build On:

  1. OAuth 2.0 / OpenID Connect (OIDC)
  - Use for: Agent authentication & authorization
  - Why: Widely adopted, handles delegation ("this agent acts on behalf of user X")
  - Benefit: Existing libraries in every language, familiar to developers
  - Agent twist: Issue tokens to agents instead of humans

  2. Passkeys / WebAuthn / FIDO2
  - Use for: Cryptographic agent credentials
  - Why: Phishing-resistant, public/private key pairs
  - Agent twist: Each agent instance gets its own passkey
  - Benefit: Hardware-backed security (TPM/Secure Enclave)

  1. DIDs (Decentralized Identifiers) - W3C Standard
  - Use for: Persistent agent identity without central authority
  - Why: did:key:, did:web: give agents global unique IDs
  - Benefit: Agent can prove identity across services
  - Example: did:key:z6Mkf... identifies agent, verified via cryptography

  2. Verifiable Credentials (VCs) - W3C Standard
  - Use for: Agent capabilities/permissions
  - Why: Cryptographically signed claims ("this agent can access X")
  - Benefit: Portable across MCP servers, revocable

  Recommended Architecture:

  Layer 1: Agent Identity (DIDs)

  Agent ID: did:web:example.com:agents:assistant-v1
  Public Key: stored in DID document

  Layer 2: Authentication (Passkeys/WebAuthn)

  Agent proves ownership of DID via private key signature
  MCP server verifies via public key in DID document

  Layer 3: Authorization (OAuth 2.0 / OIDC)

  Agent gets OAuth token scoped to specific permissions
  Token includes: agent DID, user delegation, capabilities
  MCP server validates token, checks scopes

  Layer 4: Capabilities (Verifiable Credentials)

  Agent presents VCs proving:
  - "Can read files in /documents"
  - "Authorized by user@example.com"
  - "Issued by trusted policy server"

  Concrete Tech Stack:

  For OAuth/OIDC:
  - Keycloak (open source identity provider)
  - ORY Hydra (lightweight OAuth server)
  - AuthZ (authorization as a service)

  For DIDs:
  - did:web (simplest - DID hosted on web server)
  - did:key (self-contained cryptographic DID)
  - Libraries: did-jwt, veramo

  For Passkeys/WebAuthn:
  - @simplewebauthn (JavaScript library)
  - webauthn-rs (Rust)
  - py_webauthn (Python)

  For Verifiable Credentials:
  - vc-js (W3C reference implementation)
  - SpruceID toolkit
  - Walt.id SSI framework


# Glossary
IAM - Identity and Access Management (controlling who has access to what systems/data)
IGA - Identity Governance and Administration (managing user identities, access rights, and compliance)
XAM customers - Cross-Application Management customers (users managing access across multiple applications)
EPM - Enterprise Performance Management (business planning, budgeting, and financial consolidation systems)
A2A - Application-to-Application (automated communication/integration between software systems)
PPM - Project Portfolio Management (managing multiple projects and resources across an organization)
PAM - ]

# Ideas

1. Agentic AI Identity - 
2. PAM - Pluggable Authentication Modules
3. Risk Engine - 
4. Automation - 

Corporate Agentic AI-based automation
- Must mirror org's current Identity & authority mgt
- Allow BOTH users & their agents to share to operate with/ SAME authority.


==> Third-party MCP servers w/ customer credentials.

==> Open source standard for agentic identity.

