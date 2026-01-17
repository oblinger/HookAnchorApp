

## TESTS
- L1 - 01-22 THR 6p - 4-level coding challenge
- L2 - Live Code - Shared Python Env - LeetCode
- L3 - LLM prompting in Colab - practical LLM usage
- L4 - Research Brainstorm
- Process reportedly takes 3-4 weeks total from start to offer
- Testing Done on https://codesignal.com 


## 2026-01-16  Email from Anthropic

Hi Daniel,

Thank you for your application to Anthropic Fellows!

We thought your application was quite promising, and we invite you to the next stage of the application process. This stage includes two steps:

1. Completing an automated technical screen through the CodeSignal platform.
2. We will send a form to each of your references. **Please check that their information is correct [here](https://constellation.fillout.com/t/8RAKxUgP1Wus?id=recYvD7vS3otZwYCZ).** If you asked us to confirm with you before contacting a reference, you can grant confirmation at the link (check the box(s) and submit the form). We collect references early in the application process because they are one of the main ways we assess applications. The most useful references come from people who've collaborated with you on technical work and can speak concretely to your strengths and weaknesses. References from the ML research community are preferred if available.

You should receive an invite from CodeSignal shortly; if you don't see it in your inbox today, please let us know. **Please note that the link expires 1 week after the invite. The CodeSignal email will contain the exact expiry time.**

It may be helpful to take [this practice assessment](https://app.codesignal.com/practice-question/description/progressiveFilesystemUnitTests?context=otherTypes) on CodeSignal to familiarize yourself with the UI before attempting the test.

### Test Format

You'll be asked to implement a simple coding project from a spec, broken into four levels. **The test time limit is 90 minutes. You do not need to complete all levels to advance, but finishing early is a bonus for your application.** The test is intended to test your ability to quickly write correct code. You will not be evaluated on code quality, readability, or maintainability, and execution speed only matters when explicitly mentioned.

**You can read the tests, and run the tests** early and often. In case of ambiguity, use the provided test suite as the final word on the requirements. Don't worry about edge cases if there are no tests checking them.

We require that you take the test in Python, as that is what Anthropic primarily uses.

### Test Contents

You should be familiar with writing classes and methods, using lists, dictionaries, and sets, and using the standard Python functions for sorting, hashing, and binary search. It's beneficial to be familiar with the standard library modules bisect and collections. The provided tests uses the unittest library.

You will **not** require any external frameworks or libraries, or knowledge of machine learning or mathematics. For preparation, we recommend practicing standard LeetCode-style problems. To maintain fairness to all candidates, we can't provide further details on the contents of the assessment.

### Additional Rules

While AI-assisted coding is increasingly powerful, for now we believe that the ability to write code without AI assistance correlates with doing well in Fellows.

The take-home project later in the process will give us a sense of how effectively you can leverage AI tools, but for this round we want to see your performance without any AI assistance.

You may consult language documentation and we also allow "AI overview" search results in Google, since these are enabled by default. **You are not allowed to use any other form of AI coding assistance, including Claude Code, Cursor, Copilot, etc.**

Please write code only within the CodeSignal platform, and **do not use an external editor - any pasting of external code will be flagged.** We leverage both manual and automated checks to verify this. Violation of these rules will result in us not moving forward with your candidacy.

After reviewing your technical screen and references, we will update you on your application status. The next stages in the application process include additional technical interviews.

Let me know if you have any questions!

Best,  
Joe  
Anthropic Fellows Applications Team


## 2026-01-15  Notes About Format (from AI)

Anthropic AI Safety Fellowship - What to Expect

  The Coding Test Structure

  Round 1: CodeSignal Assessment
  - 4-level coding challenge (progressive difficulty)
  - Pure Python - no ML libraries needed
  - OOP-focused: build systems like banking, database, file system
  - Example: In-memory database with SET/GET/DELETE → filtered scans → TTL → compression
  - Target score: 480/600 to advance

  Round 2: Live Coding (55 min)
  - Google Meet, shared Python environment
  - LeetCode medium-level, but more practical than algorithmic
  - Focus on speed and correctness over Big O analysis
  - Modular, clean code matters - requirements evolve during the test

  Round 3: LLM Prompting (55 min)
  - Engineering with LLMs in Colab
  - Prompt engineering, debugging, practical Claude/LLM usage

  Round 4: Research Brainstorm (15 min)
  - With an Anthropic researcher
  - AI safety discussion, your ideas, alignment thinking

  Key Insights

  - No AI assistance allowed in live interviews
  - Python only - be very comfortable with it
  - Use a large monitor - time pressure is real
  - They care about AI safety thinking as much as coding
  - Read their https://www.anthropic.com/core-views-on-ai-safety and interpretability research

  What They're Looking For

  "They care deeply about alignment, safety, and ethics. If you can't talk thoughtfully about AI safety, you're probably not getting the offer."

  Sources:
  - https://interviewing.io/anthropic-interview-questions
  - https://igotanoffer.com/en/advice/anthropic-interview-process
  - https://alignment.anthropic.com/2024/anthropic-fellows-program/
  - https://medium.com/@anqi.silvia/my-2025-anthropic-software-engineer-interview-experience-9fc15cd81a99

✻ Baked for 54s

❯ they say I need to take the coding test. By the twenty eighth. Does this mean I need to do the first test by then? And then they'll ask me about the next test. Or do these
  tests happen in quick succession?

⏺ Based on the interview info, the CodeSignal assessment is the first gate. That's likely what they mean by "coding test by the 28th."

  The typical flow:

  1. CodeSignal (online, async) ← This is your Jan 28 deadline
    - You take it on your own time before the deadline
    - ~4 level progressive coding challenge
    - Need ~480/600 to advance
  2. Then they review and schedule next rounds (days/weeks later)
    - Live coding interview (55 min)
    - LLM prompting interview (55 min)
    - Research brainstorm (15 min)

  The process reportedly takes 3-4 weeks total from start to offer, so the live interviews don't happen immediately after CodeSignal.

  Bottom line: Complete the CodeSignal by the 28th. If you pass, they'll reach out to schedule the next rounds - you'll have time to prepare further.

  Want me to help you prep for the CodeSignal format? The OOP/system-building style (database, file system) is specific and worth practicing.
