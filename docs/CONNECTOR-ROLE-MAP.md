# Connector Role Map — One Home Per Data Type

**Status:** Draft for owner decision. Nothing has been created, moved, renamed or deleted.
**Evidence cut-off:** 2026-10-19.
**Scope:** the ten Arena connectors currently enabled — Google Drive, Google Docs, Google Sheets, Google Calendar, Google Slides, Notion, Linear, Supabase, Vercel, Dropbox.
**Intended canonical home:** `faroukpandor/faroukpandor.github.io` → `docs/CONNECTOR-ROLE-MAP.md`.
**Staged in:** `faroukpandor/faroukpandor` (a fork of the deprecated `netlify-templates/victor-hugo`), branch `arena/e5da17aa-faroukpandor`. See Decision D7 — that staging location is itself part of the problem.

Evidence labels used: **VERIFIED** (read directly this pass), **DOCUMENTED** (publisher's own published terms), **SELF-REPORTED** (owner-authored record), **UNVERIFIED**, **REQUIRES CONFIRMATION**.

---

## 1. If you read nothing else

Three findings reorder the whole picture. None of them is about tidiness; all three are about money.

1. **The hosting plan under the revenue work forbids commercial use.** `mokoro-eos`, `agrisage` and `jong-finance` are deployed on Vercel **Hobby**, which is restricted to non-commercial personal use. Vercel's own fair-use guidelines define commercial use as including "advertising the sale of a product or service", "receiving payment to create, update, or host the site", and any deployment used for the financial gain of anyone involved in producing it — explicitly including a paid consultant writing the code. This is an eligibility rule, not a traffic threshold: it is broken by intent, not volume. **DOCUMENTED** ([fair use guidelines](https://vercel.com/docs/limits/fair-use-guidelines), [terms](https://vercel.com/legal/terms)). Your own stack already contains the permitted alternative — Cloudflare Pages, which you are already deploying to. Cost to comply: zero.
2. **The books are empty.** `FAROUK-BOOKS-LEDGER` (`1EkkN9IqMME4ZCnaKM3ZIwveAGR2wsmccMgmRKjbwVaE`) returns one row: the header. Date / Type / Ref / Client / Description / Amount / Method / Received-Paid — no entries. **VERIFIED**. Combined with your Principal Action Sheet item 4 ("invoice + log in FAROUK-BOOKS-LEDGER same day"), the most economical reading is that no revenue has yet passed through this system. **REQUIRES CONFIRMATION** — entries may live somewhere I cannot see.
3. **Nothing in the calendar is a customer.** Across 2026-10-19 → 2026-11-03 the calendar is dominated by recurring internal blocks — Project Coordination (weekdays 10:00–11:00), Strategic Thinking (Thu 09:00–11:00), Sprint Planning (Thu 12:00–14:00) — plus one supplier-chain call and one personal visit. There is no sales call, no audit booking, no client meeting. **VERIFIED**. That is consistent with the P0 money action (name three businesses, send three audit bookings) still being unsent. The bottleneck is not a missing system. It is that the system has no customers in it.

The organising conclusion: this ecosystem does not need more structure. It has 107 governance documents, 23 Drive folders, 40 visible repositories, 24 calendars, 10 databases and a register that already defines ownership vocabulary. What it lacks is **one agreed home per data type**, so that each connector has a job instead of being a place where a copy might also exist.

---

## 2. The rule

**One canonical home per data type. Everything else is transport or staging, and is labelled as such.**

A second copy is allowed only when it is read-only and disposable — a mirror for phone reading, a PDF export, a backup. A second copy that can be *edited* is not a copy; it is a fork, and forks drift. That is already visible in your own estate: pesticide records exist in two Notion data sources with the same underlying record.

---

## 3. Canonical home per data type

| # | Data type | Canonical home (the only editable one) | Transport / staging | Status & evidence |
|---|---|---|---|---|
| 1 | Governance, strategy, decision records | GitHub `faroukpandor.github.io` → `docs/` (107 documents) | — | **VERIFIED.** Register marks it `ACTIVE / CANONICAL` |
| 2 | Enterprise IP and internal operating system | GitHub `MOKORO-ENTERPRISE-OS` (private) | Drive `0.DOCUMENTS` as a read-only mirror | **VERIFIED** via Vercel/Drive; repo unreadable to this session (403) |
| 3 | Repository, enterprise and ownership register | `PORTFOLIO-REPOSITORY-ENTERPRISE-REGISTER.md` | — | **VERIFIED.** Already authoritative; do not duplicate |
| 4 | Money: invoices, quotes, ledger | Google Sheets `FAROUK-BOOKS-LEDGER` | Drive `0.FINANCE_&_INVOICES` holds the artefacts (PDFs, order forms) | **VERIFIED.** Header-only, zero entries |
| 5 | Supplier and vendor evidence | Drive `0.VENDORS` | Promote to `docs/` once it informs an offer or a decision | **VERIFIED** folder exists |
| 6 | Agronomy and veterinary domain knowledge | Notion — **one** database per subject | — | **VERIFIED duplication** (see D4) |
| 7 | Dated commitments and work queue | Linear, team `FAR` | Principal Action Sheet stays the narrative | **VERIFIED.** 5 issues, all Todo, none assigned |
| 8 | Commitments with people and dates | Google Calendar | Source calendars (WCD, Facebook appointments, crop production) stay read-only feeds | **VERIFIED.** 24 calendars |
| 9 | Client and employer material | Client-controlled Drive — never mirrored | — | Register already carries `CLIENT-CONTROLLED` |
| 10 | Files and binary attachments | Google Drive | — | Dropbox is empty; see D8 |
| 11 | Build, preview and deploy | Vercel for previews of non-commercial work | Cloudflare Pages for public/commercial hosting; GitHub Pages for the portfolio | **VERIFIED.** See D1 |
| 12 | Database, auth, storage | Supabase — only where a revenue flow requires it | — | **VERIFIED** unhealthy; see D2 |

Two observations that follow directly from the table:

- **Google Sheets is not a registry.** `list_spreadsheets` returned `[]` on two calls, while `get_values` on the ledger ID succeeded. The connector can read a known spreadsheet but cannot enumerate them. A registry that relies on listing would silently report that you have no finance files. Address books and inventories belong in Markdown in the repo, not in Sheets.
- **Notion's job is narrow.** Its value here is the agronomy archive, which is real intellectual property — production guides, business plans, pesticide records. It is not a second documents folder, and it should not hold governance or money.

---

## 4. Decisions this forces

Each one is a decision, not a task. Costs are stated. Nothing here is executed.

**D1 — Vercel Hobby carries commercial work. `RED` for a paying client, `$0` to fix.**
`mokoro-eos` (PR #76, 2026-10-19), `agrisage` and `jong-finance` (live at `jong-finance.vercel.app`, loan management) all sit on Hobby. `agrisage` advertises a broiler-farming product; `jong-finance` is a lending tool. Both are commercial under the published definition. Options: (a) move anything monetised to Cloudflare Pages and keep Vercel for previews only — zero cost, already in your stack; (b) upgrade to Pro at $20/seat/month — only defensible once a paying engagement depends on Vercel-specific features; (c) do nothing — acceptable only while nothing is being sold, and wrong the day the first DGE audit invoice is issued. **Recommendation: (a), before the invoice, not after.**

**D2 — Supabase: one project or none. `AMBER`.**
Ten projects; nine inactive (nexus-print, supabase-sky-door, supabase-blue-ball, BWHANDYMAN, everything-city, MOKORO, supabase-vercel-erp, supabase-green-lamp, BUSINESS_DIRECTORY). The single active project, `Farouk Pandor Collective` (`absqkjimaooetbdbvxge`, eu-west-1), reports `db`, `auth`, `rest`, `storage`, `realtime` and `pg_bouncer` all **UNHEALTHY**; only the pooler is healthy. A schema query timed out at 544 seconds and was not retried. Free plan allows **two active projects**, and pauses any project after seven days without database activity — and, critically, **free projects have no backups** on the free plan. **DOCUMENTED.** Note the contradiction: an inactive project on the free plan is supposed to be *paused*, not unhealthy — so "UNHEALTHY" is not simply the inactivity rule, and its cause is unknown to me. Do not debug this speculatively. Decide what, if anything, in `Farouk Pandor Collective` is load-bearing; if nothing is, the correct answer is zero live databases, not one broken one. If something is, the missing backup is the emergency, not the outage.

**D3 — Linear is not the queue of record. `GREEN`.**
`FAR` holds five issues — FAR-21 Android/Capacitor wrapper, FAR-22 Tauri desktop, FAR-23 iOS, FAR-24 SECURITY.md and token rotation, FAR-25 paid-step budget — all Todo, unassigned, no project, all derived from `docs/26`. That is a backlog for a future product, not a record of current commitments. Meanwhile the commitments that actually bind you (three audit bookings, JV & Sons gate 1, RafTon/Zhauns/SupplySA sends, Cloudflare Access) live in a Drive document with no dates and no reminders. The fix is to load the Action Sheet's twelve items into `FAR` as dated issues, one project: `Revenue — Q4 2026`. This is the highest-leverage change in this document that costs nothing and creates nothing new.

**D4 — Duplicate pesticide databases in Notion. `AMBER`.**
The same record appears in two data sources: `b826ad89-5703-4b61-a2ef-9bb0bcc6b3d2` and `63ce62e6-e4d3-466e-b61b-e84b64a2a3f5` — SARBAN 4 E (chlorpyrifos 480 g/L), SAFA TARIM A.S., registration 1877, importer Scientific Agricultural Services Est, restricted, expired 2019-03-30. **VERIFIED.** Choose one database as canonical, keep the other read-only until the merge is confirmed, then archive it. Do not delete records until one set has been diffed against the other — this is regulated-product data and a wrong merge is a safety problem, not a tidiness problem.

**D5 — The ledger is on Los Angeles time. `GREEN`, money-adjacent.**
`FAROUK-BOOKS-LEDGER` has `time_zone: America/Los_Angeles`; your calendar is `Africa/Maputo`. Any date entered near midnight, or any auto-stamp, will land on the wrong day and potentially the wrong month — in the one file that records revenue. Set it to `Africa/Gaborone`. Two minutes, reversible.

**D6 — `agrisphere` vs `smartfarm`. `Owner decision`, already open as Action Sheet item 11.**
Both repositories were pushed 2026-10-16. Two candidate homes for the same agricultural capability means neither is canonical. Pick one, mark the other `CONSOLIDATE` using the vocabulary your register already defines, and stop pushing to both.

**D7 — The PERSONAL-BRAND docs are in the wrong repository. `GREEN`.**
`docs/PERSONAL-BRAND/` — README, CAPABILITY-MAP, ENTERPRISE-ROUTING-MATRIX, MASTER-PROFESSIONAL-RECORD, OWNERSHIP-STATUS-REGISTER, PORTFOLIO-ASSET-LEDGER — exists only in `faroukpandor/faroukpandor`, which is a **fork of the deprecated `netlify-templates/victor-hugo`** (description: "FAROUK PANDOR by Victor Hugo boilerplate…"). The canonical repo `faroukpandor.github.io` contains none of them. Your register says the canonical repo is `ACTIVE / CANONICAL` and this one is not mentioned at all. So the current strategic documentation of your professional layer is stranded in a deprecated boilerplate fork with a Hugo/Webpack build nothing uses. Move the six documents into `faroukpandor.github.io/docs/PERSONAL-BRAND/` — same relative path, a straight copy — and leave the fork as a stub or archive it.

**D8 — Dropbox is empty and should be declared so. `GREEN`.**
Root listing returns `[]`. It is not a storage hub and should not appear in any plan. Declaring it unused is cheaper than keeping it as an unexamined "maybe".

**D9 — There is no email connector. `Owner decision`.**
Of the ten enabled connectors, none is mail. The two live commercial threads in your own records — JV & Sons (Sections 1–12 complete, five WhatsApp messages sent 2026-10-09, awaiting reply, gate 1 is the identity test) and the vendor sends — both terminate in an inbox this system cannot see. Action Sheet item 10 already reads "Gmail reconnect in Arena". Until it is reconnected, no system in this document can tell you whether a deal is alive or dead. That is a bigger gap than any of the tidiness items above.

---

## 5. Free-tier leverage — and where each one actually bites

You asked to exploit the free tiers. The useful version of that is knowing which limit will stop you, not which features you get.

| Service | Free allowance (verified where cited) | The limit that will bite you |
|---|---|---|
| GitHub | Private repos, Pages, unlimited collaborators on public | Pages soft bandwidth 100 GB/month; 1 GB repo — not a near-term constraint |
| **Vercel Hobby** | 100 GB fast data transfer, 1M edge requests, 1M function invocations, 4 CPU-hrs, 200 projects, 100 deploys/day, 1 concurrent build, 1 hr log retention, 100 cron jobs but **one invocation per day** | **The commercial-use clause**, not any number. Also one concurrent build and 1-hour logs, which makes it poor for incident work |
| **Cloudflare Pages** | Generous static/edge allowance, commercial use permitted | Nothing that bites a static export. This is the arbitrage: the same product, permitted |
| **Supabase Free** | 2 active projects, 500 MB DB, 1 GB storage, 5 GB egress, 50k MAU | **Pauses after 7 days idle; no backups.** Never the only copy of anything |
| Google Drive | 15 GB shared with Gmail and Photos (standard free tier) — **DOCUMENTED, not fetched this pass** | 23 folders of PDFs and exports. The real ceiling is Drive, not any connector |
| Linear | Ample for five issues | Not capacity. The constraint is that commitments are not entered |
| Notion | Free personal workspace — **UNVERIFIED**, no figure asserted | Not the binding limit here; duplication is |
| Dropbox | Empty | Nothing. Declare unused |

The strategic point: **you are paying nothing, and the free tiers are not the constraint.** The constraint is that the hosting plan you use for revenue work is not licensed for revenue work, and the database you keep is not the one you need.

---

## 6. Ranked next actions

Ordered by money and risk, not by tidiness.

1. **Move anything monetised off Vercel Hobby to Cloudflare Pages.** Before the first DGE invoice, not after. Zero cost. (D1)
2. **Decide what Supabase is for.** If nothing in `Farouk Pandor Collective` is load-bearing, retire it; if something is, recovering it is urgent because there is no backup. (D2)
3. **Load the twelve Principal Action Sheet items into Linear `FAR`**, dated, one project. This is what stops the three audit bookings from being a paragraph. (D3)
4. **Reconnect Gmail.** Without it, both live commercial threads are invisible to the system. (D9)
5. **Collapse the Notion pesticide databases** to one, after a diff. (D4)
6. **Fix the ledger timezone** and log the next invoice in it the day it is issued. (D5)
7. **Move `docs/PERSONAL-BRAND/` into `faroukpandor.github.io`.** Same paths, straight copy. (D7)
8. **Close the `agrisphere` / `smartfarm` decision.** (D6)

Explicitly **not** recommended: creating a repository, a dashboard, a Notion database, a spreadsheet registry, a new Supabase project or a subscription. There are 107 documents already; the deficit is not documentation.

---

## 7. What I did not verify

Stated plainly so nothing here is read as more than it is.

- **The private repositories.** `MOKORO-ENTERPRISE-OS`, `AGRISAGE`, `EVERYTHINGCITY` and the rest are invisible to this session — the GitHub integration returns 403 and `gh repo list` silently omits private repos. The canonical register lists eleven repositories I could not open. Everything in this document about them comes from Vercel deployment metadata, Drive mirrors and your own register — **SELF-REPORTED** at best.
- **What is inside `Farouk Pandor Collective`.** The schema query timed out; no table, row count or data content is known. Whether the database holds anything worth recovering is **unknown**.
- **Notion in full.** Search truncates even at 25 results because of property payloads. I saw a sample of databases and pages, not the workspace.
- **Vercel in full.** `list_projects` truncates at both 50 and 10 because it embeds deployment metadata. The project inventory is partial. I did not surface any environment values and none appear in this document.
- **The ledger's emptiness** is verified as a read, not as a fact about your business — entries may exist elsewhere. Flagged as such in section 1.
- **Google Drive quota**, Notion free-plan limits and Linear free-plan limits were not fetched this pass.

No files were moved, no records edited, no deployments changed, no keys read into this document.
