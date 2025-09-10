# OpenAI Integration Logic Analysis - Sportsvisio

## Overview

The Sportsvisio platform uses OpenAI's GPT models primarily for **automated sports game analysis and coaching insights**. The AI integration is sport-specific, with different prompts and logic for Basketball, Basketball 3x3, and Volleyball games.

## Core AI Service Architecture

### OpenAI Service Configuration
**Location**: `src/shared/open-ai.service.ts`

**Model Used**: `o4-mini` (OpenAI's o1-preview/o1-mini model)
**Default Settings**:
- Temperature: 1 (maximum creativity)
- Max completion tokens: 8192
- Top P: 1 (full probability distribution)
- Reasoning effort: "high"
- Frequency/presence penalty: 0

The service provides a single method `sendCompletion()` that wraps OpenAI's chat completions API with consistent defaults.

## AI Usage Patterns

### 1. Game Summary Generation (Fan-Facing)

**Purpose**: Create engaging 200-300 word summaries of completed games for fans, email notifications, and social sharing.

**Trigger**: Automatically generated when games reach `FINAL_APPROVED` status via background queue processing.

**Data Inputs**:
- Complete box score statistics (points, rebounds, assists, etc.)
- Play-by-play action logs with timestamps
- Team standings and win/loss records
- Team schedules and opponent information
- Player performance metrics

#### Basketball Game Summaries

**System Prompt Logic**:
```
You are the **Basketball Game Summarizer**. Write summaries **200–300 words** long. 
Focus primarily on **player stats** (efficiency, defensive contributions, assists, rebounds, points, overall impact) and **team stats**. 
Use shorthand for statistics (e.g., `PTS`, `REB`, `AST`) and keep an **enthusiastic, respectful, unbiased** tone. 
Use **gender-neutral terms** unless gender is certain.

You may also receive **optional JSON data** for:
- Play-by-play / action logs (e.g., scoring events, key plays, timing)  
- Team standings  
- Team schedules  

Use this optional data **only to supplement** the main player and team stats...
```

**Input Data Structure Example**:
```json
{
  "statsSummary": {
    "teams": [
      {
        "Team": "Lakers",
        "Id": "team-uuid-1",
        "PTS": 98, "FGM": 37, "FGA": 85, "2PM": 24, "2PA": 58,
        "3PM": 13, "3PA": 27, "FTM": 11, "FTA": 15,
        "ORB": 12, "DRB": 33, "TRB": 45, "AST": 23, "TO": 14, "STL": 9, "BLK": 4, "PF": 18
      },
      {
        "Team": "Warriors",
        "Id": "team-uuid-2", 
        "PTS": 102, "FGM": 39, "FGA": 82, "2PM": 22, "2PA": 48,
        "3PM": 17, "3PA": 34, "FTM": 7, "FTA": 10,
        "ORB": 8, "DRB": 38, "TRB": 46, "AST": 28, "TO": 12, "STL": 7, "BLK": 6, "PF": 16
      }
    ],
    "players": [
      {
        "Player": "LeBron James", "Jersey": "6", "Team": "Lakers",
        "PTS": 28, "FGM": 11, "FGA": 22, "2PM": 7, "2PA": 14,
        "3PM": 4, "3PA": 8, "FTM": 2, "FTA": 3,
        "ORB": 2, "DRB": 8, "TRB": 10, "AST": 9, "TO": 4, "STL": 2, "BLK": 1, "PF": 3
      },
      {
        "Player": "Stephen Curry", "Jersey": "30", "Team": "Warriors", 
        "PTS": 32, "FGM": 12, "FGA": 25, "2PM": 4, "2PA": 8,
        "3PM": 8, "3PA": 17, "FTM": 0, "FTA": 0,
        "ORB": 1, "DRB": 5, "TRB": 6, "AST": 7, "TO": 3, "STL": 1, "BLK": 0, "PF": 2
      }
      // ... additional players
    ]
  },
  "teamsSummary": [
    {
      "team": { "name": "Lakers" },
      "schedule": [
        {
          "startTime": "12/15/2024, 7:00:00 PM",
          "opponents": ["Celtics"]
        },
        {
          "startTime": "12/18/2024, 8:30:00 PM", 
          "opponents": ["Warriors"]
        }
      ],
      "standings": { "wins": 18, "losses": 12 }
    }
  ],
  "playByPlay": [
    {
      "type": "field-goal-made",
      "startTime": 2897,
      "teamPlayerName": "LeBron James",
      "qualifiers": ["2-point"]
    },
    {
      "type": "field-goal-made", 
      "startTime": 2845,
      "teamPlayerName": "Stephen Curry",
      "qualifiers": ["3-point"]
    },
    {
      "type": "steal",
      "startTime": 2780,
      "teamPlayerName": "Anthony Davis",
      "qualifiers": []
    }
    // ... additional play-by-play events
  ]
}
```

**Business Logic**:
1. Gathers comprehensive game statistics from database
2. Fetches team schedules and current standings for context
3. Retrieves play-by-play actions, filtering out timing events
4. Combines all data into structured JSON for AI processing
5. Stores generated summary in `ScheduledGameInsights` table

**Basketball Game Summary Output Example**:
```
Warriors Overcome Lakers in Thrilling 102-98 Victory

Stephen Curry led the Warriors to victory with a dominant 32 PTS performance, shooting an impressive 8/17 from beyond the arc. Curry's 7 AST facilitated the Warriors' balanced offensive attack, which generated 28 total assists. The Warriors' superior three-point shooting (50%, 17/34) proved decisive against the Lakers' interior-focused approach.

LeBron James answered with 28 PTS, 10 REB, and 9 AST, nearly achieving a triple-double in the loss. The Lakers controlled the paint with 12 ORB compared to Golden State's 8, but couldn't overcome their three-point shooting struggles (48%, 13/27). Anthony Davis contributed 24 PTS and 12 REB, providing consistent interior presence.

The Warriors' ball movement created open looks throughout the game, evidenced by their superior AST/TO ratio (28/12 vs Lakers' 23/14). Golden State's defensive rebounding (38 DRB) limited Lakers' second-chance opportunities despite LA's offensive glass advantage. 

Key momentum shifts occurred in the third quarter when Curry connected on three consecutive three-pointers, extending Golden State's lead from 2 to 11 points. The Lakers responded with an 8-0 run late in the fourth, but couldn't complete the comeback.

Both teams showed playoff-caliber intensity with this division rivalry matchup having significant standings implications. The Warriors improve to 20-10 while the Lakers fall to 18-13, creating separation in the competitive Western Conference standings.
```

**Volleyball Game Summary Output Example**:
```
Rodriguez Powers Team to 3-Set Victory with Dominant Offensive Display

Maria Rodriguez led all players with 29 total points, recording 22 kills on 52 attacks for a .192 hitting percentage in the three-set victory. Rodriguez's aggressive offensive approach was complemented by 5 serve aces, putting constant pressure on the opposing reception. The team's balanced attack featured strong contributions across multiple statistical categories.

Sarah Johnson provided excellent support with 18 kills and an impressive .222 hitting percentage, demonstrating superior efficiency in her attacks. Johnson's 12 assists facilitated the team's offensive flow, while contributing 14 digs on the defensive end. Her consistent serve reception (18/21, .857) provided stable platform for the team's offense.

The victorious squad dominated at the net with superior blocking, recording 6 total blocks compared to their opponents' defensive efforts. Strong defensive play included 30 combined digs between the top performers, showcasing excellent court coverage and defensive awareness throughout all three sets.

Service games proved crucial with both teams recording multiple aces, but Rodriguez's 5 service winners created significant scoring opportunities. The team's reception efficiency allowed for consistent offensive execution, limiting the opponents' ability to disrupt their system.

This victory demonstrates the team's depth and balanced statistical production across kills, blocks, and defensive categories. The players' combined 47 total points illustrate the offensive firepower that has made them competitive throughout the season. Strong fundamentals in serve reception and defensive coverage continue to be hallmarks of their successful system.
```

#### Volleyball Game Summaries

**System Prompt Logic**:
```
As the Volleyball Game Summarizer, your summaries should be within 200-300 words, 
focusing on player efficiency, defensive plays, assists, serves, and overall team 
performance. Maintain an enthusiastic, factual, and unbiased tone...
```

**Includes Comprehensive Volleyball Glossary**:
- kills, killsAssist, attack, attackErrors
- hitPercentage calculation: (Kills − Errors) ÷ Total Attacks
- serveAces, serveErrors, blocks, blockAssist
- dig, ballHandlingErrors, reception statistics

**Volleyball Input Data Example**:
```json
[
  {
    "id": "player-summary-uuid-1",
    "player": { "id": "player-uuid-1", "displayName": "Sarah Johnson" },
    "summary": {
      "kills": 18, "killsAssist": 12, "attack": 45, "attackErrors": 8,
      "hitPercentage": 0.222, "serveAces": 3, "serveErrors": 2, "totalServe": 15,
      "blocks": 4, "blockAssist": 6, "blockErrors": 1, "dig": 14,
      "ballHandlingErrors": 2, "totalPoints": 25, "setsPlayed": 3,
      "reception": 18, "receptionError": 3, "totalReceptions": 21
    }
  },
  {
    "id": "player-summary-uuid-2", 
    "player": { "id": "player-uuid-2", "displayName": "Maria Rodriguez" },
    "summary": {
      "kills": 22, "killsAssist": 8, "attack": 52, "attackErrors": 12,
      "hitPercentage": 0.192, "serveAces": 5, "serveErrors": 4, "totalServe": 18,
      "blocks": 2, "blockAssist": 4, "blockErrors": 0, "dig": 16,
      "ballHandlingErrors": 1, "totalPoints": 29, "setsPlayed": 3,
      "reception": 22, "receptionError": 2, "totalReceptions": 24
    }
  }
  // ... additional players
]
```

**Volleyball-Specific Logic**:
- Only processes non-empty game summaries (filters out players with no stats)
- Focuses on volleyball-specific metrics like hit percentage and serve efficiency
- Includes detailed explanations of volleyball terminology in prompt

#### Basketball 3x3 Summaries

**Simplified Approach**:
- Uses same statistical data structure as regular basketball
- Shorter, focused prompt acknowledging the faster-paced 3x3 format
- Emphasizes efficiency and all-around player impact

### 2. Coaching Insights Generation

**Purpose**: Provide analytical feedback and strategic recommendations for coaches based on game performance.

**Availability**: Currently only implemented for Basketball (not Volleyball or Basketball 3x3)

**System Prompt Logic**:
```
Assume the role of a seasoned head coach. Provide analytical insights and suggestions based on the boxscore data
provided by the user, focusing on enhancing the team's performance. Emphasize clear, actionable feedback.
Highlight three positives and three areas for improvement from the boxscore data, offering detailed strategy
suggestions for improvement...

CRITICAL: You must respond with valid JSON only, in exactly this structure:
[{"teamId": "string", "positive": "string", "negative": "string"}]
```

**Response Processing**:
1. Expects structured JSON response with team-specific insights
2. Validates JSON structure and maps team IDs correctly
3. Falls back to raw text if JSON parsing fails
4. Stores insights in `coachInsights` field of `ScheduledGameInsights` table

**Coaching Insights Output Example**:
```json
[
  {
    "teamId": "team-uuid-1",
    "positive": "### Strong Offensive Execution\n\nYour team demonstrated excellent ball movement with 28 assists, showcasing unselfish play and good court vision. The 3-point shooting was particularly effective at 50% (17/34), indicating good shot selection and preparation.\n\n### Defensive Rebounding Dominance\n\nControlling the defensive glass with 38 rebounds limited second-chance opportunities for the opponent. This defensive rebounding created fast-break opportunities that led to easy scoring chances.\n\n### Free Throw Efficiency\n\nShooting 70% from the line (7/10) in crucial moments shows mental toughness and proper fundamentals under pressure.",
    "negative": "### Turnovers Need Attention\n\nWith 12 turnovers, focus on protecting the basketball through better decision-making. Implement more half-court sets to reduce rushed possessions and emphasize passing fundamentals in practice.\n\n### Offensive Rebounding Deficiency\n\nOnly 8 offensive rebounds suggests missed opportunities for second-chance points. Work on boxing out techniques and crashing the offensive glass more aggressively, especially from guard positions.\n\n### Foul Trouble Management\n\n16 personal fouls put key players at risk. Emphasize defensive positioning and discipline to avoid unnecessary contact fouls, particularly in the paint and on drives to the basket."
  },
  {
    "teamId": "team-uuid-2", 
    "positive": "### Balanced Scoring Attack\n\nMultiple players contributed significantly with no single player carrying the entire offensive load. This balance makes the team harder to defend and creates more sustainable offensive production.\n\n### Transition Defense\n\nLimited opponent fast-break opportunities through good hustle getting back on defense. This prevented easy scoring chances and forced the opponent into half-court execution.\n\n### Paint Presence\n\n6 blocks demonstrate good rim protection and defensive awareness. The interior defense altered shots and created additional possessions through deflections.",
    "negative": "### Three-Point Defense\n\nAllowing 13 made three-pointers suggests breakdowns in perimeter defense. Focus on closeouts and communication on defensive rotations, particularly on pick-and-roll coverage.\n\n### Assist-to-Turnover Ratio\n\nWith 23 assists but 14 turnovers, ball security needs improvement. Implement more structured offensive sets and emphasize making the simple pass rather than forcing difficult ones.\n\n### Free Throw Attempts\n\nOnly 15 free throw attempts indicates insufficient aggression getting to the rim. Encourage more drives to the basket and contact plays to get to the line more frequently."
  }
]
```

## Queue-Based Processing

**Background Processing**: AI generation is handled asynchronously via Bull queues to prevent blocking game status updates.

**Queue Jobs**:
- `sendAiSummaryGeneration`: Generates fan-facing game summaries
- `sendCoachingSummaryGeneration`: Creates coaching insights
- `sendGameStatusNotif`: Triggers notifications with optional AI summary

**Processing Logic**:
```typescript
// Fan notifications include AI summary
if (sendNotifications) await this.notifyGameUpdate(game, trophiesDiff);

// Separate coaching insights generation
if (this.enableCoachAiSummary && !insights?.coachInsights) 
    this.scheduledGameProducerService.sendCoachingSummaryGeneration({ gameId: game.id });

// Standalone AI summary if no notifications sent
if (this.enableAiSummary && !insights?.aiSummary && !sendNotifications) 
    this.scheduledGameProducerService.sendAiSummaryGeneration({ gameId: game.id });
```

## Data Transformation Logic

### Basketball Statistics Transformation

The system transforms raw game data into AI-friendly structures:

**Team Statistics Aggregation**:
```typescript
teamObj.PTS += summary.totalPoints;
teamObj.FGM += summary.fieldGoals.made;
teamObj.FGA += summary.fieldGoals.attempts;
teamObj["2PM"] += summary.twoPoints.made;
teamObj["3PM"] += summary.threePoints?.made || 0;
teamObj.FTM += summary.freeThrows.made;
teamObj.ORB += summary.offensiveRebounds;
teamObj.AST += summary.assists;
// ... etc
```

**Player Performance Mapping**:
- Maps roster information to statistical summaries
- Handles phantom players and display names
- Preserves team associations and jersey numbers
- Calculates composite statistics like total rebounds

### Context Enhancement

**Schedule Context**:
- Retrieves all games in the same division/event
- Provides opponent names and game timing
- Shows team's position in season context

**Standings Integration**:
- Queries precomputed `TeamEventStatSummary` records
- Provides wins/losses for contextual analysis
- Helps AI understand game significance

## Error Handling and Reliability

**Graceful Degradation**:
- Continues processing even if AI calls fail
- Logs errors but doesn't block game status updates
- Falls back to storing raw responses if JSON parsing fails

**Validation Layers**:
- Validates JSON structure for coaching insights
- Handles malformed AI responses gracefully
- Provides fallback content for failed generations

**Performance Considerations**:
- Caches statistical calculations
- Filters unnecessary play-by-play data to reduce token usage
- Uses background queues to prevent blocking operations

## Integration Points

**Database Storage**:
- `ScheduledGameInsights.aiSummary`: Fan-facing game summaries
- `ScheduledGameInsights.coachInsights`: Structured coaching feedback
- Linked to games via foreign key relationship

**Notification System**:
- AI summaries included in email notifications
- Integrated with SendGrid template system
- Supports both immediate and queued delivery

**API Endpoints**:
- `/annotations/stats/export/vision-team/llm-summary`: Direct AI summary generation
- RESTful access to generated insights via game endpoints
- Supports on-demand regeneration of summaries

## Business Logic Flow

1. **Game Completion**: When game status changes to `FINAL_APPROVED`
2. **Statistical Calculation**: Box scores and player summaries generated
3. **Queue Job Creation**: Background tasks created for AI generation
4. **Data Gathering**: Statistics, context, and play-by-play compiled
5. **AI Processing**: Structured prompts sent to OpenAI with game data
6. **Response Processing**: AI responses validated and stored
7. **Notification Integration**: Summaries included in fan communications
8. **Coach Dashboard**: Insights available for coaching staff review

This AI integration provides automated, contextual sports analysis that enhances both fan engagement and coaching effectiveness while maintaining reliability through robust error handling and queue-based processing.