# Branch Consolidation Manifest — `faroukpandor.github.io`

**Date:** 2026-10-19
**Method:** every branch compared to `main` by file path *and blob SHA*. Read-only; nothing merged.
**Corrects:** the claim that 138 files were stranded across 75 branches. That was wrong. See §1.

---

## 1. The correction

An earlier finding in `CONNECTOR-ROLE-MAP.md` (and Linear FAR-39) said the repository had "138 files stranded across 75 unmerged branches", a real corpus of ~245 documents with `main` exposing 44%. **That was an error.**

The 138 figure came from GitHub's compare API `files[]` count, which reports files **changed** between `main` and a branch — including edits to files that already exist. I read it as files **absent** from `main`. It is not.

Measured properly, by comparing blob SHAs:

| Measure | Count |
|---|---|
| Branches (excluding `main`) | 75 |
| (branch, file) entries changed vs `main` | 138 |
| Branch file versions **byte-identical to `main`** | **93** |
| Branch file versions **different from `main`** | 39 |
| File paths **absent from `main` entirely** | **6** |
| **Unique paths needing any attention at all** | **16** |

`main` holds 133 tracked files. Branches add 6. The corpus is ~139 files, not ~245. `main` exposes 96% of it, not 44%.

Of the 39 differing versions, the majority are simply stale copies of generated front-end files on branches older than the last `main` commit:

- `index.html` — 15 branches
- `assets/css/site.css` — 13 branches
- `portfolio-map.html`, `professional-record.html` — 1 each

Those are almost certainly branches cut before later `main` commits, not unique work. They need no merge; the branch should be deleted once its real payload (if any) is extracted.

## 2. What actually needs review — the 16

### 2.1 Files that exist ONLY on a branch (6)

These are the only genuinely absent content. Copy these paths into `main`:

| Path | Branch |
|---|---|
| `docs/AFRICA-100-PASS-EXECUTION-REGISTER.md` | `africa-opportunity-engine-100x-v1` |
| `docs/AU-55-MEMBER-STATE-BASELINE.md` | `africa-opportunity-engine-100x-v1` |
| `docs/SADC-16-MEMBER-BASELINE.md` | `africa-opportunity-engine-100x-v1` |
| `docs/INCOME-FIRST-10-OFFERS.md` | `docs/income-first-10-offers-2026-10-18` |
| `docs/opportunities/DIGITAL-INFRASTRUCTURE-AND-ASSET-MANAGEMENT.md` | `strategy/digital-infrastructure-opportunity-hub-2026-10` |
| `docs/COLLECTIVE-CUSTOM-CHATBOT-SERVICE-MAP-2026-10-16.md` | `strategy/collective-idea-assimilation-map-2026-10-16` |

Note two of these are recent and substantive: `INCOME-FIRST-10-OFFERS` (2026-10-18) and the digital-infrastructure asset-management opportunity note. `COLLECTIVE-CUSTOM-CHATBOT-SERVICE-MAP` predates Vercel PR #76 by three days and is referenced by that governance work, so it is worth recovering for provenance even if superseded.

### 2.2 Documents where the branch version differs from `main` (8 paths, 9 versions)

Diff each against `main` and keep whichever is newer, then delete the branch:

| Path | Branch holding the differing version |
|---|---|
| `docs/AFRICA-RESEARCH-10X-CONTINUATION-PROTOCOL.md` | `africa-opportunity-engine-10x-v1` |
| `docs/CAPABILITY-EVIDENCE-OFFER-MATRIX.md` | `strategy/capability-offer-architecture-v6` |
| `docs/CAPABILITY-EVIDENCE-OFFER-MATRIX.md` | `strategy/v8-commercial-front-doors` |
| `docs/COMMERCIAL-EVIDENCE-OPPORTUNITY-REGISTER.md` | `commercial-evidence-register-v1` |
| `docs/COMMERCIAL-MVP-OFFERS-AND-DELIVERY-SYSTEM.md` | `commercial/v9-mvp-offers` |
| `docs/PORTFOLIO-ASSET-LEDGER.md` | `content/v5-portfolio-asset-ledger` |
| `docs/REPOSITORY-RELATIONSHIP-REGISTRY.md` | `competitor-amish-ways-opportunity-plan` |
| `docs/REPOSITORY-RELATIONSHIP-REGISTRY.md` | `docs/portfolio-sync-registry-links-2026-10-17` |
| `docs/REPOSITORY-RELATIONSHIP-REGISTRY.md` | `strategy/collective-idea-assimilation-map-2026-10-16` |

`REPOSITORY-RELATIONSHIP-REGISTRY.md` is the interesting one: three branches carry three different versions of the repository registry — the single document that should be authoritative. That is worth resolving properly, because a registry with three candidate versions is not a registry.

## 3. Revised consolidation procedure

Much smaller than originally proposed. No batch merges, no 75 pull requests.

1. `git checkout main`
2. Copy the 6 files from §2.1. Commit.
3. Diff the 9 entries in §2.2 against `main`; take the newer version of each. Commit.
4. Resolve `REPOSITORY-RELATIONSHIP-REGISTRY.md` deliberately — read all three, keep one.
5. Delete all 75 branches. Nothing else on them differs from `main`.
6. From now on: one short-lived branch per change, merged immediately. The branch-per-document-batch habit is what produced this.

**Estimated work:** under two hours, most of it step 4. Not the multi-day consolidation implied by "138 files".

## 4. What changed in the conclusion

The original finding was used to reorder the priority list, ahead of the Vercel commercial-use exposure and the Supabase backup gap. With the corrected numbers, branch hygiene drops to **exactly what it is**: tidy-up work. The revenue-blocking items — the audit bookings, the hosting-plan exposure, the missing mailbox — are unaffected by this correction and remain the priorities.

This is recorded rather than quietly deleted because the error is instructive: a tool's field name (`files[]`) described *changed*, and I read it as *missing*. The lesson is to reconcile API counts against a second measurement before using a number to reframe a plan. The blob-SHA comparison here is that second measurement.
