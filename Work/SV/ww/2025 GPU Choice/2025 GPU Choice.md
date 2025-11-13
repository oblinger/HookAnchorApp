
- [T59](https://docs.google.com/document/d/1g135euS74c7FfBO6GDMRGyvi_0jXHBm4Pd4bqU9Pa94/edit?tab=t.0) 


# Roadmap

## SW Tactical Shifts
- Trimming the sails - S3/videos, EC2, images, 
- Media Convert Solution

## Microsoft Credits
- Media Convert Phase I, II, III
	Activate Credits in phase III.
- AI-GPU & Media Convert will eat credits

## AWS Rearchitecting
- AWS Spot instances (save up to 90%)
- ARM / Graviton

## A-Round
## Big III Pricing

### AI - Roadmap

# __
## Questions
- 
# Log


## 2025-10-31  GCP Pricing (See [[@google]]) 



## 2025-10-28  AWS Spot Instances


  How Volatile Are Spot Instances?

  Overall Interruption Rate:
  - Average across all regions/instance types: <5% (historically)
  - However, some instance types and regions can have >20% interruption rates
  - Recent trends (2023-2024) show increasing volatility - some regions saw interruption rates spike as much as 55% in us-east-1

  Interruption Rate Categories (per AWS Spot Instance Advisor):
  - <5% (most stable)
  - 5-10%
  - 10-15%
  - 15-20%
  - 20% (most volatile)

  Mean Runtime Before Interruption

  Average: ~4 hours before interruption

  But the range is extremely wide:
  - Some instances terminated after just 10 hours
  - Others ran for 257-351 days before shutdown
  - Highly dependent on instance type, region, and availability zone

  How Often Are They Unavailable?

  The Bad News:
  - AWS doesn't publish historical capacity availability data
  - Capacity shortages can happen at any time, even when prices are low
  - "No capacity available" errors are common during high-demand periods

  Key Insight:
  - Capacity shortages ≠ interruptions
  - You can fail to launch a spot instance (no capacity)
  - OR get one that runs for hours/days and then gets interrupted

  Practical Takeaways

  For Reliability:
  1. Diversify - Use multiple instance types across multiple AZs
  2. Check the Spot Instance Advisor (https://aws.amazon.com/ec2/spot/instance-advisor/) before choosing instance types
  3. Avoid instance types with >10% interruption rates for critical workloads
  4. Expect interruptions but with 2-minute warning to checkpoint/save state

  For Workloads:
  - ✅ Good for: Batch jobs, stateless apps, fault-tolerant systems
  - ❌ Bad for: Long-running databases, stateful apps without checkpointing

  Reality: Spot instances are increasingly volatile since 2023 but still viable if you architect for interruptions.

## 2025-10-24  Emily Gay

Hey Dan,

  

Here’s that GPU inventory without the markup I mentioned. We’re sharing this with about 40 AI startup leads before listings go public. Pricing is often 50% below most clouds and 70% less than AWS/big clouds.

  

These are live rates that update as startups reserve capacity — first come, first served.

  

Below are some of our lowest-priced clusters. These are the rates most teams start with. Other configs are available if you’re scaling or need specific specs.

  

⚫ **Blackwell (RTX 6000 / B200)**

• RTX 6000 Blackwell — about $1.63/hr per GPU ($13/hr per node) | 8 × RTX 6000 | Tier 3 DC (US/EU) | 1-node min (4 weeks)

• B200 — about $3.20/hr per GPU ($25.60/hr per node) | 8 × B200 | Tier 3 DC (EU) | 1-node min (4 weeks)

  

🟣 **NVIDIA H200**

• Base setup — $1.96/hr per GPU ($15.70/hr per node ≈ $11K/mo) | 8 × H200 | 2 × AMD 9654 | 2 TB RAM | Tier 3 DC (France) | 1-node min (4 weeks)

  

🟢 **NVIDIA H100**

• Starting around $1.40/hr per GPU ($11.20/hr per node ≈ $8.1K/mo) | 8 × H100 | 2 × Xeon Platinum 8468 | 2 TB RAM | Tier 3 DC (Amsterdam / NY) | 10-node min (4 weeks)

• Flexible option from $1.20/hr per GPU ($9.60/hr per node ≈ $7K/mo) | 1-node min (4 weeks) | Tier 3 DC (Houston, TX)

  

🟣 **NVIDIA 5090** — around $0.68/hr per GPU ($5.44/hr per node) | 8 × 5090 | 512 GB RAM | Tier 3 DC (LA / NY) | 1-node min (4 weeks)

  

🟢 **NVIDIA 4090** — around $0.40/hr per GPU ($3.20/hr per node) | 8 × 4090 | 128 GB RAM | Tier 3 DC (LA / Amsterdam) | 1-node min (4 weeks)

  

Happy to walk you through how much you’ll save/options before these clusters are spoken for.

Fair to assume you're running on AWS or another hyperscaler right now?

  

Best,

Emily Gay

Valdi.ai Compute

[View full inventory →](https://gpulist.valdi.ai/)