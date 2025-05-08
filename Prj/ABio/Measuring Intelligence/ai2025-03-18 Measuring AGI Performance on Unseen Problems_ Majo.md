<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" class="logo" width="120"/>

# Measuring AGI Performance on Unseen Problems: Major Evaluation Frameworks

Artificial General Intelligence (AGI) represents a milestone where machines can replicate the full spectrum of human intellectual abilities across diverse domains. A key challenge in AGI research is determining how to measure an AI system's ability to handle unseen problems, which demonstrates true general intelligence rather than task-specific performance. This report examines the most significant research frameworks designed to evaluate AGI performance on novel tasks.

## Leading AGI Evaluation Frameworks

### ARC Challenge (Abstract and Reasoning Corpus)

The Automated Reasoning Challenge, or ARC, introduced by François Chollet in 2019, stands as one of the most widely recognized benchmarks for assessing progress toward AGI. This framework specifically targets an AI system's ability to generalize its knowledge to new, previously unseen tasks[^1][^2].

ARC consists of a series of complex, puzzle-like problems that require an AI to make predictions or generate answers from limited data. What makes ARC particularly valuable as an AGI assessment tool is its focus on abstraction and generalization rather than pattern recognition or rote learning[^1]. The test evaluates whether a system can infer and apply rules without prior exposure to similar problems, thus testing true intelligence rather than memorization capabilities[^1].

As of 2023, the best-performing AI systems scored 55.5% on the ARC-AGI test, which shows significant improvement but still falls short of the approximately 85% threshold considered to represent human-level intelligence[^2]. This benchmark has gained substantial credibility in the AI research community as a meaningful measure of progress toward AGI[^1][^2].

### GAIA Benchmark (Generalized AI Agent)

The GAIA benchmark represents a more recent and comprehensive approach to evaluating intelligent agents across multiple dimensions. Unlike task-specific benchmarks such as GLUE (for NLP) or ImageNet (for computer vision), GAIA is specifically designed to measure generalized intelligence across various domains, making it particularly relevant for AGI assessment[^3].

GAIA consists of 466 curated questions spanning different complexity levels and assesses AI agents along four key dimensions:

1. **Task execution**: The ability to complete predefined tasks with minimal errors and without direct human intervention
2. **Adaptability**: How well an agent responds to unforeseen circumstances
3. **Collaboration**: Evaluation of multi-agent coordination and human-agent teaming capabilities
4. **Generalization**: Testing whether an agent can apply learned knowledge to novel, unseen scenarios[^3]

What distinguishes GAIA from other benchmarks is its focus on tasks that humans find conceptually simple but require AI systems to demonstrate fundamental skills such as multi-modal reasoning, web browsing, information retrieval, and tool usage[^3]. This approach aligns well with evaluating an AI system's ability to handle unseen problems in real-world contexts.

## Additional AGI Evaluation Approaches

### Levels of AGI Framework

Developed by Morris et al., this framework introduces a classification system for AGI capabilities based on two core dimensions: performance and generality. This approach is notable for its emphasis that generality must be paired with performance measures to be meaningful[^4].

In this framework:

- **Performance** refers to the depth of an AI system's capabilities, measured against human-level performance for given tasks
- **Generality** refers to the breadth of capabilities, or the range of tasks for which an AI system reaches a target performance threshold[^4]

The framework establishes percentile-based performance levels relative to adult humans who possess relevant skills, creating a standardized method for evaluating progress toward AGI across different systems and approaches[^4].

### Extended Turing Test

The Extended Turing Test builds upon Alan Turing's original concept but expands it to assess general intelligence across multiple domains. While the original test focused primarily on conversational abilities that might fool human judges, the extended version incorporates broader cognitive tasks to evaluate whether machine intelligence can match human capabilities across various domains[^1].

This approach assesses not just linguistic competence but also reasoning, problem-solving, and adaptive learning when faced with novel situations. However, detailed specifications of this framework were not extensively covered in the search results[^1].

### The Coffee Test

Mentioned as one of several AGI evaluation frameworks, the Coffee Test presents a real-world challenge: the AI system must enter an unfamiliar house and successfully make coffee using the available equipment and supplies[^1]. This test elegantly combines several essential AGI capabilities:

1. Environmental perception and navigation
2. Object recognition and functional understanding
3. Sequential planning and execution
4. Adaptation to unfamiliar settings
5. Understanding human preferences and social contexts

By requiring the AI to perform a mundane human task in an unfamiliar environment, the Coffee Test evaluates general intelligence capabilities rather than domain-specific knowledge[^1].

## Challenges in AGI Evaluation Methodologies

### Definitional Ambiguity

A significant challenge in evaluating AGI performance is the lack of consensus on what constitutes "general intelligence." Despite numerous proposed benchmarks, the AI research community has not converged on scientifically grounded definitions of AGI, making evaluation frameworks inherently contentious[^5].

Several researchers have argued that focusing on AGI as a singular goal may actually impede scientific progress by creating an "Illusion of Consensus" where researchers assume they're working toward the same objective despite fundamentally different understandings of what AGI entails[^5].

### Performance vs. Generalization Trade-offs

Current evaluation approaches struggle with balancing performance depth against generalization breadth. While some frameworks like ARC emphasize the ability to solve novel problems, effectively measuring both dimensions simultaneously remains challenging[^1][^4].

The Levels of AGI framework explicitly addresses this issue by treating performance and generality as separate dimensions, but the inherent tension between specialized excellence and broad competence persists across evaluation methodologies[^4].

## Conclusion

The measurement of AGI performance on unseen problems remains an evolving field with several promising approaches but no definitive solution. The ARC Challenge and GAIA Benchmark currently represent the most structured and widely recognized frameworks, offering complementary perspectives on evaluating general intelligence capabilities.

Both frameworks specifically target an AI system's ability to handle novel, unseen problems, which is essential for distinguishing true general intelligence from narrow, task-specific AI. However, with the AI field rapidly evolving, these evaluation methodologies will likely continue to develop alongside the technologies they seek to measure.

For researchers and developers interested in AGI evaluation, combining multiple assessment frameworks may provide the most comprehensive picture of an AI system's capabilities across different dimensions of general intelligence.

<div style="text-align: center">⁂</div>

[^1]: https://www.linkedin.com/pulse/how-do-we-know-weve-achieved-agi-nuri-cankaya-ivfec

[^2]: https://autogpt.net/arc-agi-test-nears-resolution-but-experts-question-its-validity-in-agi-quest/

[^3]: https://workos.com/blog/gaia-benchmark-evaluating-intelligent-agents

[^4]: https://arxiv.org/pdf/2311.02462.pdf

[^5]: https://arxiv.org/html/2502.03689v1

[^6]: https://pmc.ncbi.nlm.nih.gov/articles/PMC11614890/

[^7]: https://cloud.google.com/discover/what-is-artificial-general-intelligence

[^8]: https://arxiv.org/abs/2411.14486

[^9]: https://www.rand.org/pubs/perspectives/PEA3691-4.html

[^10]: https://www.linkedin.com/pulse/agi-closer-than-we-think-insights-from-arc-agi-test-r-pillai-ukqoe

[^11]: https://www.linkedin.com/pulse/arc-agi-test-tough-nut-crack-llms-mena-habeel-ojo9f

[^12]: https://www.linkedin.com/pulse/arc-agi-benchmark-agi-asi-journey-superintelligence-amita-kapoor-7jpwc

[^13]: https://arcprize.org/arc

[^14]: https://www.promptingguide.ai/techniques/zeroshot

[^15]: https://aws.amazon.com/what-is/artificial-general-intelligence/

[^16]: https://arcprize.org/blog/r1-zero-r1-results-analysis

[^17]: https://arxiv.org/html/2502.06559v1

[^18]: https://www.reddit.com/r/mlscaling/comments/1ht4emi/anyone_else_suspect_arcagi_was_never_much_of_a/

