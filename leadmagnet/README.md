# Lead Magnet — 7-Day Validation Test

This is the exact "Single Most Valuable Next Action" from your strategy doc:
a free, static, zero-backend landing page that trades one digital asset for a
WhatsApp opt-in. Deploy it and start driving traffic today — no build step,
no framework, no database.

## 1. Customize it (5 minutes)

Open `index.html` and edit:

| What | Where |
|---|---|
| Page `<title>` and meta description | `<head>` |
| Headline, subtitle, bullet points | `#asset-title`, `#asset-subtitle`, `#asset-bullets` |
| Your WhatsApp number | the `href="https://wa.me/26700000000?..."` link — replace `26700000000` with your number in international format, no `+` or spaces (e.g. Botswana `267` + number) |
| The pre-filled message | the `text=` part of that same URL |

That's it — one file, no dependencies, nothing to install.

## 2. Deploy it free on Cloudflare Pages

You already have Cloudflare Pages wired up for the main site in this repo.
Create a **second, separate Pages project** pointing at this same GitHub repo,
but with a different root directory, so both sites deploy independently from
one repo:

1. Cloudflare dashboard → Workers & Pages → Create → Pages → Connect to Git
2. Select this repository again
3. Build settings:
   - **Framework preset**: `None`
   - **Build command**: *(leave empty — there's nothing to build)*
   - **Build output directory**: `leadmagnet`
4. Deploy. You'll get a second free URL, e.g. `your-leadmagnet.pages.dev`.

(If you'd rather have a totally separate repo for this so it doesn't get
confused with the dev-tools project, that's fine too — just copy the
`leadmagnet/` folder contents into a new repo's root.)

## 3. Drive traffic and measure (the actual test)

Per the strategy: use **only your 3 most active Facebook pages**, and tag each
one with a different `?src=` so you know which page converts:

```
https://your-leadmagnet.pages.dev/?src=page1
https://your-leadmagnet.pages.dev/?src=page2
https://your-leadmagnet.pages.dev/?src=page3
```

The page automatically appends `[source: page1]` etc. to the pre-filled
WhatsApp message — so when someone messages you, you'll immediately know
which Facebook page the lead came from. Zero analytics setup required.

**Optional (also free):** add Cloudflare Web Analytics to see raw page-view
counts per link, so you can calculate a real conversion rate (WhatsApp clicks
÷ page views). Cloudflare dashboard → your Pages project → Analytics & Logs →
Web Analytics → "Add site" → paste the one `<script>` snippet it gives you
into `index.html`'s `<head>` (there's a marked spot for it already).

## 4. What to report back after 7 days

- Page views per source (`page1` / `page2` / `page3`), if you added Web Analytics
- Number of WhatsApp messages actually received, and from which source
- Quality of those leads (are they real prospects, or just curiosity clicks?)

That data — not more building — is what should decide whether you invest in
the MMVE-OS subscription product or the B2B directory next.

## Alternative: capture emails instead of WhatsApp

If you'd rather build an email list than a WhatsApp thread, swap the CTA
button for a form using **Formspree** (free tier: 50 submissions/month, no
backend code):

1. Create a free account at [formspree.io](https://formspree.io) and create a form — you'll get a URL like `https://formspree.io/f/xxxxxxxx`.
2. Replace the `<a id="whatsapp-cta">...</a>` button with:
   ```html
   <form action="https://formspree.io/f/xxxxxxxx" method="POST">
     <input type="email" name="email" placeholder="you@example.com" required />
     <button type="submit" class="cta">Send me the Guide →</button>
   </form>
   ```
3. Submissions land directly in your email inbox — no server needed.
