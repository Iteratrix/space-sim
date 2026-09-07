# 04 — Economics and sponsors

Status: complete first draft. Sections per BRIEF.md.

## 1. Findings

### 1.1 The uncomfortable truth: bulk material to Earth's surface

**The precious-metal case does not close, and every honest study since 1997 says so.**

The world platinum-group-metal market is tiny by mass. USGS (2025) gives 2024 mine
production of 170 t platinum and 190 t palladium; total PGM including rhodium, ruthenium
and iridium is roughly 400-670 t/yr depending on what is counted. At 2024 prices
(Pt ~$950/oz, Pd ~$980/oz, Rh ~$4,600/oz) the whole PGM market is on the order of
$30-40B/yr. Terrestrial reserves are ~80-100 million kg, mostly in the Bushveld and
Norilsk. Terrestrial "high-grade" ore runs 4-10 ppm PGM; iron meteorites run
100-187 ppm; a single 1-km M-type body contains more PGM than has ever been mined on
Earth (Nachtrieb & Smith 2026). That last fact is the problem, not the opportunity.

Three analyses, using three methods, converge:

- **Blair (2007, NSS/Colorado School of Mines)** sized returned mass against the market.
  A 100-m LL-chondrite sphere (1.4 Mt) contains ~43 t Pt, worth ~$690M at $500/oz.
  A 200-m body contains twice annual world Pt output. He proposes a practical ceiling of
  ~20% of annual market to avoid price disruption, which caps a mission at ~100 m of
  unprocessed rock, and then notes that launch technology limited retrievable bodies to
  ~20 m ($5.5M of Pt), concluding that "both options for mining asteroidal platinum lack
  economic justification" while "existing launch vehicle technology is more than adequate
  to ship enough concentrate to totally disrupt the world market."
- **Hein, Matheson & Fries (2020, Acta Astronautica)** extended Sonter's (1997) NPV
  model with reuse, learning curves, multiple spacecraft, and a supply-demand model.
  Their demand anchor is ($40,449/kg, 254,582 kg/yr). With price elasticity 0.5-0.6 and a
  parameter *b* for how much terrestrial producers cut output per kg of asteroid supply,
  "there is only a slim range of [elasticity] and *b* values for which asteroid mining is
  profitable, and an even slimmer range in which it breaks even before 30 years." At
  *b*=1 (Earth mines shut 1:1) the venture makes $1,044M/yr and breaks even in one year;
  at *b*≈0.85 profit is $0; at *b*=0 (Earth mines keep producing) it is ~$70M/yr with no
  breakeven inside 30 years. Their scenario tables assume Pt at $70,000/kg; at $30,000/kg
  (the 2024 price) only the 10-small-spacecraft conservative case breaks even within
  10 years. Their verdict: a platinum asteroid miner "would only be profitable in a
  relatively small and unlikely set of conditions."
- **Nachtrieb & Smith (2026, MIT Sloan system dynamics)** model the transition rather than
  the equilibrium, using AstroForge's claimed ~85% margins vs ~7% terrestrial. Result:
  total market profit rises ~8x during a "gold rush" phase, then collapses to less than
  half its initial value as price falls toward the asteroid marginal cost and margins
  compress to ~10%. "The window for outsized profits is finite." The first mover wins; the
  industry as a whole does not.

**Psyche's "$10,000 quadrillion" is iron mass times iron price with no supply response.**
World iron-ore production is ~2.5 Gt/yr at ~$100/t. Delivered to Earth's surface for
free, Psyche's iron is worth nothing beyond what displaces existing mines; delivered
at any positive cost it is worth less than nothing. The figure is a rhetorical device
from the mission's PI and should be treated as such.

**A main-belt outpost is doubly disqualified from Earth-return.** NEAs are the
competitors for any Earth-facing product, with lower delta-v and 1-3 year cycles; belt
bodies cost 2-5x the delta-v and 2-4 year one-way transits (see 01-map). Whatever the
belt outpost ships, it will not be to Earth's surface, and it will not be PGMs.

**The 2012-2019 postmortem.** Planetary Resources raised ~$50M (Schmidt, Cameron, a
Luxembourg government stake later written off); Deep Space Industries raised ~$3.5M plus
contracts. Both were acquired in 2018-19 (ConsenSys; Bradford Space) for reasons
unrelated to mining. The MIT Technology Review postmortem is explicit about the cause:
"a traditional VC time line is 10 years... in seven years they want to exit"
(Crawford); "our planning was decades long, and a VC fund's life cycle is one decade
long. They're incompatible" (Bonin, ex-CTO DSI); "there's no customer base for asteroid
mining in the next 12 to 15 years" (Marquez, ex-Planetary Resources). Neither company's
side products (hyperspectral sensors, water thrusters) generated revenue. The current
wave (AstroForge: $40M Series A, ~$3.5M spacecraft, DeepSpace-2 landing attempt 2026,
PGM; Karman+: $20M seed, water, 2027; TransAstra: space-domain-awareness revenue while
developing capture bags) has learned to fund the long horizon with short-horizon side
businesses. That is a sponsor behaviour the sim should model: the outpost is the
long-horizon bet, and the sponsor's patience is set by its short-horizon cash.

### 1.2 Where the real market is: propellant, water, bulk mass, in space

**Everything in space is priced against the cost of launching it from Earth to that
location.** ULA's Cislunar-1000 (Sowers 2016) set the reference price ladder:
LO2/LH2 propellant at $3,000/kg in LEO, $1,000/kg at GEO or L1, $500/kg on the lunar
surface, versus ~$1/kg on Earth's surface. Their pilot plant: 1,575 t water mined on the
Moon → 1,050 t propellant → 210 t delivered to LEO (a 5:1 gear ratio) → $630M/yr revenue.
Colvin, Crane & Lal (2020, IDA/Acta Astronautica) found asteroid-derived propellant could
be delivered to LEO at ~$2,000-3,000/kg, comparable to Falcon Heavy, against a demand of
6,000-10,000 t over 20 years (300-500 t/yr), and concluded water for on-orbit propellant
is "the only asteroid-derived resource that may become economical" in their period.
ESA's Space Resources Strategy (2019) cites a Luxembourg-commissioned assessment of
73-170 B€ market revenue from space resources over 2018-2045.

**Metzger (2023) is the key theoretical result.** For an industry where capital is
built on one body, transported to a second, teleoperated there, and the product shipped
off, competitiveness is governed by two numbers: the "gear ratio on cost" *G* for
delivering the capital, and the production mass ratio φ = (mass of product over the
capital's life) / (mass of capital). In-space propellant beats Earth-launched propellant
"no matter how low launch costs go" provided φ ≳ 35 and the transport architecture keeps
*G* low; tent-sublimation lunar ice has φ ≈ 400-550, strip mining is nearer the threshold
but crosses it after 4-11 years of learning. The intuition: falling launch cost cheapens
the *capital* as much as the *competing propellant*, so the ratio survives. The
corollary for the belt: a belt outpost is worth funding only if its φ (tonnes of
volatiles shipped per tonne of imported hardware and people) is very high and its
customer is far up the gravity well. Metzger also shows why a Mars-settlement sponsor
would pay a 40% premium for in-space propellant: capital landed on Mars a synodic
period earlier compounds at ~48% per synod, so buying propellant to free launch capacity
is a net gain.

**Launch demand is now price-elastic, so the market grows.** Metzger's review: the
elasticity threshold was estimated at ~$1,300-6,100/kg (2022 $); global-average launch
cost crossed it around 2000-2010; cumulative mass to orbit grew ~140 t/yr (1980-2005),
~280 t/yr (from 2005), ~550 t/yr (since 2019). His launch-cost model runs from
$2,000/kg (Falcon 9, 2022) toward a floor of $30/kg only if annual up-mass reaches
~436,000 t/yr; at 10% and 1% of that, the 30-year cost is $119/kg and $436/kg.

**What has to be true in the inner system for a belt outpost to be fundable:**

1. A sustained in-space propellant/water market of at least several hundred t/yr with
   depots at LEO/L1, so that "propellant at location X" has a posted price.
2. That market has outgrown or bypassed lunar polar ice (which is closer and has
   φ ≈ 400+), or needs what the Moon lacks: carbon, nitrogen, hydrogen in bulk,
   and low-delta-v access to Mars and the outer system. Lunar ice is the belt's
   competitor in cislunar space; the belt wins on chemistry (C, N, NH3, organics) and
   on geography (Mars-side and belt-side demand), not on water alone.
3. A Mars-side or large-station customer whose demand for shielding mass, buffer gas
   and consumables is priced against Earth launch plus 2-4 years of transit.
4. A sponsor whose horizon is 15-30 years, which (see 1.7) is a state, a consortium
   with a state anchor tenant, or a founder-controlled firm, not a VC fund.

Candidly: the belt outpost is *not* an economic proposition on its own in act 1. It is
a strategic option purchased by a sponsor who believes (2)-(3) will come true, kept on
a budget sized to that belief. The outpost's real product in act 1 is *demonstrated
throughput per imported tonne* (φ), because that is the number the sponsor uses to
decide whether to keep paying.

### 1.3 Cost structure: launch, crewed vs automated, timescale to return

**Launch.** Shuttle ~$54,000/kg; Falcon 9 customer price ~$2,700-3,000/kg to LEO,
SpaceX internal marginal cost ~$630/kg; Starship targets <$100/kg with high reuse,
$30/kg aspirational. Cislunar-1000 in 2016 quoted $4,000-10,000/kg, with GEO 4x and the
lunar surface 9x that. The belt sits beyond the Moon: capital delivered to a belt
body costs 10-30x LEO on a gear-ratio basis and arrives 2-4 years after order (01-map,
02-vessels give the actual numbers).

**People are the expensive part, and the numbers are brutal.**

- ISS: US spend ~$75B through 2013 ($43.7B program + $30.7B for 37 Shuttle flights);
  ~$3.1B/yr now ($1.3B operations and research, $1.8B crew and cargo transport) for an
  average of ~7 crew → ~$450M per crew-year. Soyuz seats were $70M; commercial crew more.
- US Antarctic Program: $356M/yr (FY2008); NSF Antarctic Facilities and Operations line
  $216M (FY2022). McMurdo 800-1,000 summer / 150-200 winter; South Pole 150 / 45;
  Palmer 44 / 20. Roughly $300k-1M per person-year, with a prime contractor (Leidos)
  and an NSF senior representative on the ice.
- A belt outpost person sits between: ISS-like transport cost amortised over a
  multi-year tour, Antarctic-like operating cost once there. At Starship-era prices a
  person-year in the belt plausibly costs $5-30M; at Falcon prices, ten times that.

**Why the sponsor prefers automation.** Blair (2007): "A 'typical' asteroid mining
mission would launch the mining equipment at the first orbital phasing opportunity and
expect delivery of materials at the next, 2-5 years later. The requirement for
automation stems from the cost and difficulty of sending a rescue mission when the
orbit is out of phase." Metzger's whole framework assumes capital is *teleoperated*.
Hein et al. find throughput per kg of hardware (2.3×10⁻⁴ kg/s/kg for water;
two orders higher needed for platinum) is the dominant driver, and hardware is the
thing the sponsor can cost. People have no φ; they are overhead with a return-trip
liability. The sponsor's ideal outpost is a mass driver, a solar farm, a processing
plant, and as few humans as reliability demands.

**But teleoperation from Earth does not work at 10-40 minutes one-way.** Mars rover
operations already run on supervised autonomy with 3-22 minute command latency; ESA
is testing robots supervised from nearby crewed spacecraft precisely because Earth
control is too slow. NASA's Deep Space Network is 40% oversubscribed today and
projected 50% by the 2030s, with a missed track costing ~$100k/hr in mission
operations; new 34-m antennas came in at $706M vs $419M estimated, five years late.
So the sponsor's cheapest path (automation) requires either humans on site or a
scarce, sponsor-controlled communications resource. This is the act-1 tension in
economic form: **the sponsor wants a machine; the machine needs a crew; the crew,
once there, has interests.**

**Timescale to return.** Hein's conservative water case breaks even in 5.9 years, but
at an assumed water price of $20,000/kg, roughly 7x the Cislunar-1000 LEO price; at
$3,000/kg the same hardware needs high reuse and multiple spacecraft, and breakeven
moves past 10 years. Add 2-4 years of transit each way and a belt outpost's first
revenue is 5-8 years after funding, NPV-positive at 15-30 years if ever. Compare
sponsor horizons: VC fund 7-10 years; corporate capital plan 3-5 years; national
budget 1 year with 5-10 year programs; chartered-company terms 20-25 years
(RAC 1799/1821/1844; VOC 21-year octrooi; BSAC 25 + 10 years). Only the last matches.

### 1.4 Trade within the belt: scarcity, currency, light-lag

**Complementarity by asteroid type is the trade engine.** S-types (inner belt, the
majority near 2.2-2.5 AU): silicates, Fe-Ni metal, essentially no volatiles. C-types
(outer belt, 2.7+ AU): 5-20 wt% water in hydrated minerals, ~2-5 wt% carbon, and
nitrogen at ~0.1-0.3 wt% in ammonia and organics (Bennu returned samples are
ammonia-rich; the Glavin et al. 2025 Nature Astronomy paper reports abundant NH3 and
N-rich soluble organics). M-types: Fe-Ni with 100-187 ppm PGM, plus Co, Ge, Ga. Ceres:
ammoniated clays, brines, ammonium salts at Occator. A group on an S-type has structure
and no atmosphere; a group on a C-type has air and water and poor metal. Neither is
self-sufficient. This is the belt's answer to "why would anyone trade" and it falls
directly out of 01-map's body catalogue.

**What is scarce.** In order of hardness to substitute:
1. *Nitrogen* as buffer gas: ~0.9 kg N2 per m³ of habitat at 0.78 bar partial pressure,
   plus biomass N, plus leakage. Not rare in C-types but locked in clays and organics;
   groups without a C-type must buy it.
2. *Semiconductors, optics, precision bearings, seals, catalysts, pharmaceuticals*
   (03-industry's hard tail). These are imported for decades; their supply is the
   sponsor's strongest lever and the first thing a cut-off group runs out of.
3. *Skilled people and genetic diversity.* Small populations lose skills stochastically;
   the VOC and HBC both recruited from narrow pools (Orkney; specific Dutch towns) for
   control, and paid for it in skill shortages.
4. *Delta-v*: propellant at the right place and time. The belt's own transport
   currency.
5. *Information*: software, designs, medical protocols. Zero mass, but licensed.

**Currency under light-lag.** Real-time clearing is impossible at minutes to hours of
lag and irrelevant when physical delivery takes years. The historical solutions were
built for exactly this: medieval bills of exchange ("idiosyncratic loan contracts
passed between members of a closed business network"), settled at periodic fairs
(Champagne, Lyon) that functioned as clearing houses netting offsetting debts; the
HBC's *Made Beaver* as a unit of account with company-posted standards of trade; the
RAC's statutory fur prices (sea otter 50 rubles, beaver 6, silver fox 15) and company
stores selling at St Petersburg cost plus 30%; Pullman's rent-deducted wages;
Longyearbyen's quota cards and "cork money." The pattern: **a commodity unit of
account set by whoever controls the store, plus ledger credit cleared at rendezvous.**
In act 1 the sponsor is the store and sets the prices; in act 2 the unit of account
must become something physical and verifiable at delivery (tonnes of water at a named
body, kWh, or kg of propellant at a depot); in act 3 clearing at rendezvous between
groups is the confederation's first institution.

### 1.5 Chartered companies and frontier outposts

**Hudson's Bay Company (1670-1870).** The London Governor and Committee sent one ship
each spring with instructions and trade goods; it returned with furs, post journals and
accounts. One information round trip per year, for two centuries. Carlos & Nicholas
(1990) showed the Committee understood the agency problem and managed it: formal
employment contracts and bonds; modest fixed wages (~£50/yr for a factor in 1690) with
performance gratuities; a strict prohibition on private trade; mandatory journals and
account books reviewed annually; recruitment from Orkney to create "a social structure
compatible with the company's aims." The 1821 Deed Poll made officers partners: Chief
Factors got just under 1% of profits per outfit, Chief Traders half that, and sat in an
annual council. Internal inspection (proto-audit) appears by 1871. The accounting year
("outfit") ran June 1-May 31, keyed to the ship. Control *worked* for the fur trade.

It soured on the settlement question. The Selkirk grant (1811) put farmers into fur
country; the Pemmican Proclamation triggered Seven Oaks (1816, Governor Semple and
20 men killed by Métis). Métis free trade defeated the monopoly in practice (Sayer
trial, 1849). Parliament's 1857 Select Committee questioned the charter. In 1869 the
Company sold Rupert's Land to Canada "without consulting local populations"; Riel and
120 men seized Upper Fort Garry, and the outpost became the province of Manitoba
(1870). **A trading post became a polity the moment its population stopped being
company employees.**

**VOC (1602-1799).** The Heeren XVII in the Republic; Governor-General and Council at
Batavia summarising all Asian correspondence into annual *generale missiven*.
Letters took 8-9 months each way. Private trade by servants was endemic; van Imhoff
(Governor-General 1742) complained "the company trade [has become] a sideline and
private trade the main work," and responded by legitimising private trade in
non-monopoly goods, raising officer pay ~20%, and paying captains by experience.
Wezel & Ruef (2017), from 500,000+ personnel records: desertion ran 2.1% of contracts;
punishment rates were 26% higher in the strict period (1700-41) than under van Imhoff
and 3x the post-1756 rate; *monitoring* reduced desertion when private trade was
subordinated to hierarchy, *social bonds* (shared origin, prior voyages) reduced it
when private trade was elevated above hierarchy. Choose one control regime; mixing
them fails. The company died of "corruption, war, smuggling, and mismanagement,"
was nationalised 1796-1800, and its territories became state colonies.

**East India Company.** Servants enriched themselves as "nabobs" while the Company
needed a Parliamentary bailout; the Regulating Act (1773) banned private trade and
bribes and made the Directors report revenue, civil and military affairs to
government; Pitt's India Act (1784) added a Board of Control; the Crown took over in
1858. Adam Smith's diagnosis: "negligence, profusion and malversation of their own
servants." **A sponsor that cannot control its agents is taken over by *its* sponsor.**

**Russian-American Company (1799-1867).** State-chartered monopoly, 20-year charters
(1799, 1821, 1844), Main Directorate in St Petersburg, subject to imperial approval.
Baranov ran Alaska for 19 years on shares, provisioning through Fort Ross (1812-41),
Alta California, Hawaii and Yankee ships (an 1812 deal with Astor to supply the
colonies and carry furs to Canton, killed by the War of 1812). In 1808 returns covered
"slightly less than half" of 2.3M rubles of expenses. The 1818 shake-up: naval
officers replaced merchant managers (serving 5-year terms), employees moved from
shares to regular salary in money and kind, new posts, a harder line on smugglers; the
1821 charter made naval command mandatory, and "most naval officers had no experience
in the fur trade, so the company suffered." Khlebnikov's 1821 inventory of New
Archangel is a ready-made act-1 ledger: 639-693 people (235 Russian men, 92 wives and
children, 76 Creole apprentices, 155 Aleut hunters and 83 dependents, 30 school
pupils); working capital 3.26M rubles; salaries 60-150 rubles/yr plus 36 lb flour a
month, a suit and shoes; company store at cost plus 30% (10% on goods bought en
route), "mark-ups [that] sometimes equal the transport costs"; Aleuts paid one-fifth
of pelt value; a school, a hospital, a church. Fur stocks collapsed from the 1820s; the
state's obligations (schools, church, native protection, colonisation) grew; Alaska
was sold in 1867 and the assets to a San Francisco firm in 1881. **A state-backed
monopoly with an unprofitable core product survives 68 years on sovereignty value,
then is sold.**

**British South Africa Company (1889-1923).** Legislative Council 1899: administrator,
5 company nominees, 4 elected. By 1920: 13 elected vs 6 company. A 1918 Privy Council
ruling gave unalienated land to the Crown, not the Company, gutting its dividend
stream. Charter extended 1915 to 1924; Responsible Government Association 1917; won
10 of 13 seats in 1920; referendum 27 Oct 1922, ~60% of 18,810 for self-government;
self-governing colony 1 Oct 1923; the settlers bought the land for £2M and the mineral
rights for another £2M in 1933. **An elected-seat ratchet converts a company outpost
into a polity in about 25 years.**

**Virginia Company (1606-24).** Investors expected gold; the colony shipped timber,
pitch, glass. 60 of 214 alive after the 1609-10 starving time. Tobacco from 1612 saved
the books; concessions followed (headrights, an elected assembly of 22 in 1619/21).
After the 1622 attack (350 of 1,240 killed) the Crown revoked the charter (1624).
**The first product is never the one in the prospectus; the sponsor concedes
self-government to keep colonists; the state takes over when the sponsor fails.**

**Company towns.** Pullman (1880-98): company owned every house, deducted rent from
wages, banned ownership and labour meetings, ran spies; cut wages ~25% in 1893-94 and
held rents; the federal commission called it "un-American"; the Illinois Supreme Court
ordered divestiture (1898); Chicago annexed it (1899). Longyearbyen: the 1925 Svalbard
Mining Code *obliged* the company to provide shelter, a community centre, library and
hospital, with store and alcohol profits "used for the general benefit of the workers"
(still true: NOK 2.7M of "cork money" distributed in 2012); Welfare Council 1948 →
Svalbard Council 1971 → party lists 1993 → company split into coal/municipal/tourism
arms 1989 → elected Community Council of 15 in 2002; the company school went to the
state in 1937/1976; Store Norske itself nationalised 1976 because Norway wanted the
town for sovereignty. Royal Greenland Trading Department: state monopoly 1776-1950,
governed Greenland 1774-1908 through inspectors; the 1782 Instruction deliberately
isolated Greenlanders and taxed sugar and coffee to prevent "detrimental lifestyle
changes"; trade and administration separated 1908-12; Home Rule 1979; KGH handed to
Greenland 1986. **Company towns end one of three ways: courts force divestiture, the
state absorbs them, or they vote themselves into municipalities.**

### 1.6 The principal-agent problem across light-lag

The belt inverts the historical regime. HBC had one round trip a year; the VOC 18
months; the RAC a 14.5-month voyage round Cape Horn. Information and enforcement moved
at the same speed. In the belt, information moves at 10-40 minutes one-way (2-4 weeks
of solar-conjunction blackout per synodic period), while enforcement (a ship, a
replacement manager, a cut in supplies actually biting) moves at 2-5 years. **The
sponsor can see everything and touch nothing.** Every historical mechanism maps, but
with that asymmetry:

| Historical mechanism | Belt equivalent | What changes |
|---|---|---|
| Journals and accounts on the annual ship | Telemetry, inventory, video | Continuous; the outpost controls the sensors |
| Bonds, deferred wages paid at home, forfeit on desertion | Pay held on Earth; return ticket as collateral | Strong while people intend to return; zero once they don't |
| Ban on private trade / legitimised private trade (1742) | Side-deals with other belt groups | Sponsor cannot detect barter it does not carry |
| Rotation of managers (naval 5-year terms) | Crew rotation | A rotation costs a 2-4 year ship; skipping one is the first austerity move |
| Recruit from one place (Orkney) | Recruit from one culture/company | The birthplace-as-culture-seed mechanic already in NOTES.md |
| Inspection tours (Khlebnikov, HBC 1871) | Sponsor-loyal inspector on rotation ship | Arrives 2 years after the events being inspected |
| Company store, posted prices, scrip | Resupply manifest and pricing | The sponsor's strongest lever until ISRU displaces the store |
| Charter renewal by the state | Contract/licence renewal | The sponsor has its own principal, who can also lose patience |

**How outposts drift**, from the cases above, in order of appearance:
1. Local trade becomes more valuable than sponsor trade (VOC private trade; Métis
   free trade; belt: volatiles swapped with a neighbour beat waiting for the ship).
2. The population stops being employees: dependants, creoles, apprentices, settlers
   (New Archangel was already 35% dependants and non-Russians in 1821).
3. An elected or council voice appears as a concession and ratchets (BSAC, Longyearbyen,
   Virginia).
4. The outpost builds its own supply chain (Fort Ross, Astor's ships) and the store's
   monopoly erodes; in the belt this is the self-sufficiency ladder of 03-industry.
5. The sponsor's own fortunes fail (VOC, Virginia, Planetary Resources) and the outpost
   is sold, nationalised, or abandoned. This is the act-2 trigger.

The VOC finding is the sharpest mechanic: monitoring works when private activity is
subordinate to hierarchy; social bonds work when private activity is dominant; a
sponsor that tries both gets the worst of each.

### 1.7 State vs corporate vs mixed sponsors

**Corporate.** Metrics: NPV, burn rate, milestones, φ demonstrated, share price or
fund IRR. Horizon 7-10 years (VC), 3-5 (corporate capital plan), longer only for
founder-controlled firms (Metzger quotes Musk's Mars city at $100B-10T, "a huge
public-private partnership"). Patience degrades with capital markets, not with the
outpost's performance. Exit by sale of the outpost to another sponsor is normal
(RAC → Alaska Commercial Co.; PR → ConsenSys). Corporate sponsors are cheap on
obligations and fast to abandon.

**State.** Metrics: presence and sovereignty (Svalbard, Antarctica), science output,
prestige, domestic jobs, safety incidents. Horizon: annual budgets inside 5-10 year
programs inside 4-5 year electoral cycles; renewals at charter-like intervals. States
are more patient on money and less patient on *embarrassment*: a death or a scandal
moves a state sponsor more than a cost overrun. States impose obligations (RAC:
schools, church, natives; 1925 Mining Code: hospital, library, welfare) and exogenous
shocks (Longyearbyen evacuated 1941; War of 1812 killed the Astor supply deal). State
sponsors also own chokepoints private ones must rent: the DSN, launch ranges, export
licences.

**Mixed** is the historically normal form and the most likely one: a state charter or
anchor-tenant offtake contract (propellant at $X/kg for N years), a launch/tug firm,
a resource start-up, a national agency's science payload riding along, and a
government co-investor (Luxembourg in Planetary Resources; the Tsar in the RAC; the
Colonial Office over the BSAC). Mixed sponsors have *two* patience clocks and the
outpost can play them against each other, which is exactly what Baranov did with
Yankee traders and what Rhodesian settlers did with Whitehall.

### 1.8 The sponsor as an actor in the sim

**What it is for.** The outpost exists to (a) demonstrate a production mass ratio φ
high enough to make a belt supply chain fundable, (b) hold a claim and a foothold on a
good body before rivals do, and (c) deliver volatiles and bulk mass to a Mars-side or
depot customer that is still mostly promised. It is not a mine for Earth. The sponsor
knows this; the prospectus does not say it.

**What it wants** (state variables it tracks):
- Tonnes delivered to the contracted location per year versus plan.
- φ: tonnes shipped per tonne of hardware and consumables sent.
- Burn rate versus committed runway; cost per delivered tonne.
- Schedule variance on milestones (first ship, first ton, first return cargo).
- Safety and reputation incidents (deaths, failures, political noise).
- Compliance: telemetry delivered, inventory reconciled, no unauthorised activity.
- Headcount, because every person is a liability with a return ticket.

**Levers**, cheapest to most drastic:
1. Manifest composition and pricing on the next resupply (the store).
2. Contact time (bandwidth allocation, a DSN-like scarce resource).
3. Software, licences, spares, medical supplies: the hard tail.
4. Pay: what accrues on Earth, bonuses per tonne, deferred shares (the 1821 Deed Poll).
5. Crew rotation: send it, delay it, or send fewer back than out.
6. The inspector or a new manager on the next ship (the 1818 naval officers).
7. Renegotiate or fail to renew the contract; sell the outpost to another sponsor.
8. Stop sending ships. Act 2.

**How patience degrades.** Model the sponsor with three stocks and a clock:
- *Runway* (money committed): drained by burn, refilled at milestones and renewal
  reviews, shocked by exogenous events (market crash, election, launch failure,
  competitor success).
- *Confidence*: moved by variance from plan, incidents, and φ trend; decays toward
  the current evidence.
- *Attention*: how much the sponsor is looking. Low attention is freedom (drift) but
  also invisibility at review time.
- *Review clock*: fund life, budget year, charter term. Reviews are where confidence
  and runway are compared and levers pulled.

The degradation path observed historically: enthusiasm → milestone anxiety →
austerity demands (automate, cut headcount, skip a rotation, defer the hospital) →
management replacement → renegotiation/sale → abandonment. Each step is a lever from
the list above, and each is a seed: a skipped rotation makes people who never
intended to stay into settlers; an austerity demand to automate makes the crew build
the very autonomy they will need in act 2; a sale introduces a new sponsor with a
different metric set mid-game.

## 2. Numbers the sim needs

| Parameter | Value / range | Unit | Source |
|---|---|---|---|
| World Pt mine production | 170-180 | t/yr | USGS 2025 |
| World Pd mine production | 190-208 | t/yr | USGS 2025 |
| World PGM total incl. Rh/Ru/Ir | ~400-670 | t/yr | USGS; market reports |
| PGM market value | 30-40 | $B/yr | market reports 2025-26 |
| Pt price, 2024 avg / 2026 forecast | 950 / 2,190 (≈30,000 / 70,000) | $/oz ($/kg) | USGS; Metals Focus |
| Pt demand anchor for elasticity model | 40,449 $/kg at 254,582 kg/yr | | Hein et al. |
| Pt price elasticity of demand | 0.5-0.6 (inelastic) | | Hein et al. |
| Terrestrial vs asteroid PGM grade | 4-10 vs 100-187 | ppm | Nachtrieb & Smith |
| Market-disruption ceiling per shipment | ≤20% of annual market | | Blair 2007 |
| Pt content, 100 m LL chondrite | 43 | t | Blair 2007 |
| PGM gold-rush profit multiple, then collapse | ×8 peak, → <0.5 initial | | Nachtrieb & Smith |
| Post-collapse margin | ~10% | | Nachtrieb & Smith |
| Propellant price, LEO / GEO or L1 / lunar surface | 3,000 / 1,000 / 500 | $/kg | Cislunar-1000 |
| Asteroid water delivered to LEO | 2,000-3,000 | $/kg | Colvin et al. 2020 |
| Cislunar propellant demand (20-yr) | 6,000-10,000 (300-500/yr) | t | Colvin et al. 2020 |
| Water mined : propellant : delivered to LEO (lunar) | 1,575 : 1,050 : 210 | t/yr | Cislunar-1000 |
| Production mass ratio threshold φ | ≳35 | product mass / capital mass | Metzger 2023 |
| φ, tent sublimation / strip mining | 400-550 / near threshold, crosses in 4-11 yr | | Metzger 2023 |
| Space-resources market 2018-2045 | 73-170 | B€ | ESA 2019 (Luxembourg study) |
| Launch to LEO: Shuttle / F9 price / F9 cost / Starship target / floor | 54,000 / 2,700-3,000 / ~630 / <100 / 30 | $/kg | multiple |
| Launch-demand elasticity threshold | 1,300-6,100 | $/kg (2022$) | Metzger review |
| Mass to orbit, 1980-2005 / 2005-19 / 2019- | 140 / 280 / 550 | t/yr | Metzger review |
| ISS annual cost (US) | 3.1 (1.3 ops + 1.8 transport) | $B/yr | NASA OIG / SpaceNews |
| ISS cost per crew-year | ~450 | $M | derived |
| USAP budget | 216-356 | $M/yr | NSF FY2022; FY2008 |
| Antarctic populations (summer/winter) | McMurdo 800-1,000/150-200; Pole 150/45; Palmer 44/20 | people | USAP |
| Antarctic cost per person-year | 0.3-1 | $M | derived |
| Mission cycle, launch to delivery | 2-5 | years | Blair 2007 |
| Water breakeven (conservative, $20k/kg) | 5.9 | years | Hein et al. |
| Throughput needed, water / platinum | 2.3×10⁻⁴ / ~0.35 | kg/s per kg hardware | Hein et al. |
| VC fund horizon / exit expectation | 10 / 7 | years | MIT TR 2019 |
| Charter terms: RAC / VOC / BSAC | 20 / 21 / 25+10 | years | multiple |
| HBC comms cadence | 1 round trip | per year | HBCA |
| VOC letter lag | 8-9 one-way | months | multiple |
| RAC voyage St Petersburg-Sitka | 14.5 | months | Fort Ross Conservancy |
| Belt one-way light time | 10-40 (conjunction blackout 2-4 wk) | min | geometry (01-map) |
| Physical enforcement lag, belt | 2-5 | years | phasing (01-map) |
| HBC factor base wage (1690) | ~50 | £/yr | Carlos & Nicholas |
| HBC officer profit share (1821 Deed Poll) | Chief Factor <1%, Chief Trader half | of profits/outfit | HBCA |
| VOC desertion baseline | 2.1% of contracts | | Wezel & Ruef |
| VOC punishment rate, strict vs 1742-55 vs post-1756 | 1.26 : 1 : 0.33 | relative | Wezel & Ruef |
| van Imhoff officer pay raise | ~20% | | Wezel & Ruef |
| New Archangel 1821 population | 639-693 (235 Russian men, 92 dependants, 76 Creole, 155 Aleut hunters, 83 dependants, 30 pupils) | people | Khlebnikov |
| RAC company store markup | 30% (10% on goods bought en route) | | Khlebnikov |
| RAC wages, apprentices/sailors | 60-150 + 36 lb flour/month | rubles/yr | Khlebnikov |
| RAC 1808 cost recovery | <50% of 2.3M rubles | | Wikipedia (RAC) |
| BSAC LegCo elected share, 1899 → 1920 | 4/10 → 13/19 | seats | Wikipedia (Company rule in Rhodesia) |
| BSAC referendum | 59.4% of 18,810 | | ibid. |
| Settler buyout of company land / minerals | 2 + 2 | £M (1923, 1933) | ibid. |
| Virginia starving time survival | 60 of 214 | | Wikipedia (Virginia Co.) |
| Pullman wage cut vs rent cut | ~25% vs 0% | | multiple |
| Longyearbyen "cork money" | 2.7 | M NOK/yr (2012) | Sysselmesteren |
| Company-town-to-municipality ratchet | 1948 → 1971 → 1989 → 2002 | years | Sysselmesteren |
| C-type water / carbon / nitrogen | 5-20 / 2-5 / 0.1-0.3 | wt% | meteorite literature; Bennu |
| Habitat N2 inventory | ~0.9 | kg per m³ at 0.78 bar | derived |
| DSN oversubscription now / 2030s | 40% / 50% | | Physics Today 2023 |
| Missed DSN track | ~100 | $k/hr ops cost | ibid. |

## 3. Implications for mechanics

1. **The sponsor is a second player with three stocks and a review clock** (act 1).
   Runway, Confidence, Attention; reviews at fixed intervals (budget year, fund
   milestone, charter term). Evidence: every chartered company's crises cluster at
   renewal dates (RAC 1818-21, BSAC 1914-23, EIC 1773, HBC 1857-69). Levers pulled at
   reviews, not continuously. Exogenous shocks (market crash, election, launch failure)
   hit Runway regardless of outpost performance (Planetary Resources, War of 1812).

2. **The outpost's headline metric is φ, not profit** (act 1). Score = tonnes shipped
   to the contracted location per tonne received. Evidence: Metzger's φ ≳ 35 threshold;
   Hein's throughput dominance. Profit is unobservable to the player because the price
   is set at a depot years away; φ is observable every turn. Sponsor Confidence tracks
   the φ trend.

3. **Every kilogram of imported capability is either sponsor-directed or self-directed,
   and the manifest is contested** (act 1). The resupply ship is the sponsor's store:
   it prices goods (RAC +30%), and the outpost requests against a budget. The act-1
   tension is allocating manifest mass between throughput hardware (raises φ, pleases
   sponsor) and capability hardware (life support closure, fabrication, medical), which
   is invisible to the sponsor until it saves the outpost in act 2. Persist: capability
   ladder from 03-industry.

4. **Automation pressure as a sponsor demand with a hidden payoff** (act 1 → 2). The
   sponsor periodically demands headcount reduction and autonomy investment (Blair's
   "requirement for automation"; ISS $450M/crew-year vs Antarctic $0.5M). Complying
   lowers cost and raises Confidence; it also raises the outpost's autonomy score, which
   is exactly what survives cut-off. Refusing keeps skills and population, which is
   what makes a polity. Neither is wrong.

5. **Light-lag asymmetry: telemetry is continuous, enforcement is years** (act 1).
   Model "what the sponsor knows" as a filtered view of the true state, with the outpost
   choosing what to report (journals on the annual ship). Discrepancies discovered by an
   inspector on the next ship cost Confidence heavily (Khlebnikov's tours; HBC 1871
   inspections). Choosing to under-report is a seed: it works until a ship arrives.

6. **Control regime is a one-way choice: hierarchy or social bonds** (act 1 → 2).
   Evidence: Wezel & Ruef. The sponsor either bans side-trade and monitors (works while
   people intend to go home) or permits side-trade and relies on cohesion (works when
   people are staying). Switching mid-stream produces the VOC's worst of both. Expose
   this as the sponsor's stance and the player's counter-stance; the mismatch is where
   desertion/defection probabilities come from.

7. **Deferred pay and the return ticket as loyalty collateral** (act 1 → 2). Wages
   accrue on Earth; a skipped rotation converts intended-returners into settlers and
   voids the collateral. Evidence: VOC forfeiture on desertion; RAC's move from shares
   to salary in 1818 as a control measure. Mechanic: each crew member has a "return
   intent"; sponsor austerity moves flip it; flipped people are the Voidborn seed.

8. **Population composition drives drift** (acts 1-2). Track employees vs dependants vs
   locally-born vs contractors. New Archangel was ~35% non-employee in year 22; BSAC
   settlers outvoted the company by year 30. When non-employees exceed ~40-50% the
   outpost demands a council; the sponsor concedes seats (Virginia 1619, BSAC 1899) or
   loses cohesion. Each concession is a ratchet: seats never go back.

9. **Sale of the outpost to a new sponsor is a mid-act event** (act 1). Evidence:
   RAC → Alaska Commercial; PR → ConsenSys; DSI → Bradford; Store Norske → Norwegian
   state. A new sponsor arrives with different metrics (state: safety and presence;
   corporate: φ and burn) and a different patience clock, and does not honour informal
   arrangements. Contract text persists; relationships do not.

10. **Cut-off is the sponsor's final lever, not a random event** (act 1 → 2). The
    path: austerity → skipped rotation → renegotiation → no ship. The player sees it
    coming for several turns and can prepare or prevent. Evidence: every abandoned
    outpost had years of warning (VOC 1780s-1799; RAC 1820s-1867). The last resupply's
    manifest is the most important decision in act 1.

11. **Complementary scarcity by body type is the belt's trade engine** (acts 2-3).
    S-type groups lack N/C/H; C-type groups lack metal; M-type groups have both metal and
    PGM but no volatiles; Ceres has ammonia. Nitrogen and the hard tail (semiconductors,
    pharma, bearings) are the scarce goods; water and propellant are the currency
    commodities. Trade is only possible at rendezvous, so trade decisions are made
    against 01-map's time-varying graph.

12. **Unit of account is commodity-at-location, cleared at rendezvous** (acts 2-3).
    Act 1: sponsor scrip (prices posted in the manifest). Act 2: a group-set standard
    ("Made Water": kg of water at a named body). Act 3: bills of exchange between
    groups, netted at a periodic fair (Champagne/Lyon). Confederation's first
    institution is the clearing rendezvous, not a parliament.

13. **PGMs are a trap the game should let the player fall into** (act 1). A sponsor
    (corporate) may push PGM return for the gold-rush window (Nachtrieb & Smith's 8x);
    the outpost is in the wrong place (belt, not NEA) and the price collapses on
    delivery if anyone else succeeds. Offer it as an advisor's temptation with the
    price-collapse mechanic behind it.

14. **State sponsors carry obligations and chokepoints** (act 1, act 3). A state
    sponsor demands a school, a clinic and safety reporting (1925 Mining Code; RAC
    charter), which raise cost and lower φ but raise cohesion and skills; it also owns
    the comms network and can throttle contact time (DSN). In act 3 the returning inner
    system is a state actor again, and its levers are the same: contact time,
    licences, the hard tail.

## 4. Open questions

- **Which sponsor type is act 1's default?** The evidence favours a *mixed* sponsor (a
  consortium with a state anchor tenant) as most realistic, but a single corporate
  sponsor gives the cleanest tension and a single state sponsor the longest runway.
  A design decision; possibly a scenario choice at game start.
- **The customer's location.** Is the contracted delivery point a cislunar depot (where
  the belt competes with lunar ice and mostly loses), a Mars-orbit depot (where the belt
  wins on chemistry and delta-v), or a belt-internal depot serving other sponsors'
  ships? This sets transit times, the price the sponsor sees, and whether act-3
  leverage (volatiles) is credible. Needs 01-map's delta-v graph to settle.
- **The price of a person-year in the belt.** I have bracketed it ($5-30M at
  Starship-era launch prices) from ISS and Antarctic analogues; 02-vessels' habitat mass
  per person and transit times should replace the bracket.
- **Nitrogen numbers.** Bennu's bulk N and NH3 content (Glavin et al. 2025) could not
  be fetched past a captcha; the 0.1-0.3 wt% figure is from CI/CM chondrite literature
  and should be verified against the sample papers before it becomes a constant.
- **Sonter's original NPV parameters** (Acta Astronautica 41, 1997) were paywalled; I
  used Hein's and Blair's summaries. If 03-industry or 02-vessels has the paper, the
  mass-payback-ratio figures belong in the numbers table.
- **How visible should the sponsor's internal state be?** Historically the outposts
  inferred London's or St Petersburg's mood from the manifest and the tone of the
  annual letter. Full visibility makes the sponsor a puzzle; hidden state makes it a
  character. The advisor ring could be the instrument that reads it.
- **Does the sponsor ever come back in act 3 as the same institution?** VOC no; HBC yes
  (as a retailer); RAC no; BSAC yes (as a mining company without a charter). Whether
  the act-3 inner system contains the act-1 sponsor's successor with a legal claim on
  the outpost is a plot decision with mechanical consequences (contract text persists).

## 5. Sources

- Hein, Matheson & Fries, "A techno-economic analysis of asteroid mining," Acta Astronautica 168 (2020); arXiv:1810.03836 — https://arxiv.org/abs/1810.03836 — NPV model with reuse/learning/multi-spacecraft; platinum supply-demand model; the "slim range" conclusion; throughput as key driver.
- Nachtrieb & Smith, "Will AstroForge Collapse the PGM Market?" arXiv:2607.06806 (2026) — https://arxiv.org/pdf/2607.06806 — system-dynamics transition model; 8x profit run-up then collapse; PGM production table; asteroid vs terrestrial grades.
- Blair, "The Role of Near-Earth Asteroids in Long-Term Platinum Supply" (2007) — https://nss.org/wp-content/uploads/2023/05/Role-Of-Near-Earth-Asteroids-In-Long-Term-Platinum-Supply.pdf — 20%-of-market rule; size-vs-Pt table; "both options lack economic justification"; 2-5 year mission cycle and automation rationale.
- USGS Mineral Commodity Summaries 2025, Platinum-Group Metals — https://pubs.usgs.gov/periodicals/mcs2025/mcs2025-platinum-group.pdf — production, prices, reserves.
- Sonter, "The technical and economic feasibility of mining the near-earth asteroids," Acta Astronautica 41 (1997) — https://www.sciencedirect.com/science/article/abs/pii/S0094576598000873 — original NPV framework (paywalled; used via Hein and Blair).
- MIT Technology Review, "How the asteroid-mining bubble burst" (2019) — https://www.technologyreview.com/2019/06/26/134510/asteroid-mining-bubble-burst-history/ — Planetary Resources/DSI funding, VC-horizon mismatch quotes, acquisitions.
- Sowers/ULA, "Cislunar-1000: Transportation supporting a self-sustaining Space Economy" (2016) — https://sciences.ucf.edu/class/wp-content/uploads/sites/23/2017/02/Cislunar-1000-Transporation-Space-2016.pdf — propellant price ladder ($3M/t LEO, $1M/t GEO, $0.5M/t Moon), plant sizing, revenue.
- Colvin, Crane & Lal, "Assessing the economics of asteroid-derived water for propellant," Acta Astronautica 176 (2020) — https://ui.adsabs.harvard.edu/abs/2020AcAau.176..298C/abstract — $2-3k/kg to LEO; 6,000-10,000 t 20-yr demand; water the only near-term economic asteroid resource.
- Metzger, "Economics of in-space industry and competitiveness of lunar-derived rocket propellant," Acta Astronautica (2023); arXiv:2303.09011 — https://arxiv.org/pdf/2303.09011 — φ ≳ 35 threshold, gear ratio on cost, launch-cost trajectory, launch-demand elasticity, Mars capital compounding argument.
- ESA Space Resources Strategy (2019) — https://sci.esa.int/documents/34161/35992/1567260390250-ESA_Space_Resources_Strategy.pdf — 73-170 B€ market 2018-2045; demand-stimulation as agency policy.
- Orbital Radar, launch cost trends (2026) — https://orbitalradar.com/space-economy/launch-cost-trends — Shuttle vs Falcon vs Starship $/kg.
- NextBigFuture, Falcon 9 true cost (2026) — https://www.nextbigfuture.com/2026/02/spacex-falcon-9-true-cost-to-launch-is-about-300-per-pound-which-is-25-of-selling-price-to-customers.html — ~$630/kg internal cost.
- SpacePolicyOnline, NASA IG on ISS cost — https://spacepolicyonline.com/news/nasa-ig-iss-cost-u-s-75-billion-so-far-estimates-of-future-costs-overly-optimistic/ — $75B through 2013; $3-4B/yr.
- SpaceNews, ISS transition cost savings — https://spacenews.com/nasa-outlines-cost-savings-from-iss-transition/ — $3.1B/yr split ops vs transport.
- Wikipedia, United States Antarctic Program — https://en.wikipedia.org/wiki/United_States_Antarctic_Program — budget, station populations, contractor and NSF representative model.
- Physics Today, "The Deep Space Network: Overburdened and underfunded" (2023) — https://pubs.aip.org/physicstoday/article/76/12/22/2923590/ — 40-50% oversubscription; $100k/hr missed track (via search snippet; page blocked).
- Carlos & Nicholas, "Agency Problems in Early Chartered Companies: The Case of the Hudson's Bay Company," J. Econ. Hist. 50(4) (1990) — https://ideas.repec.org/a/cup/jechis/v50y1990i04p853-875_03.html — contracts, bonds, journals, Orkney recruitment; control worked.
- Hudson's Bay Company Archives glossaries — https://www.gov.mb.ca/chc/archives/hbca/glossaries.html — Made Beaver, outfit year, Deed Poll profit shares, annual packet.
- Canadian Encyclopedia / Britannica, Red River Colony and Resistance — https://thecanadianencyclopedia.ca/en/timeline/red-river-colony ; https://www.britannica.com/topic/Red-River-Rebellion — Seven Oaks, 1869 transfer without consultation, Riel.
- Wezel & Ruef, "Agents with Principles: The Control of Labor in the Dutch East India Company, 1700 to 1796," Am. Sociol. Rev. 82(5) (2017) — https://dukespace.lib.duke.edu/server/api/core/bitstreams/1e0be719-61f7-4e77-8dd6-1f4cb5062180/content — desertion rates, monitoring vs social bonds by private-trade regime, van Imhoff reforms.
- Wikipedia, Company rule in the Dutch East Indies — https://en.wikipedia.org/wiki/Company_rule_in_the_Dutch_East_Indies — Batavia autonomy, bankruptcy and 1800 nationalisation.
- Banglapedia / Wikipedia, Regulating Act 1773 — https://en.banglapedia.org/index.php/Regulating_Act,_1773 — EIC private trade ban, parliamentary oversight after bailout.
- Khlebnikov, "Russian America in 1821" (Fort Ross Conservancy library, trans. Gibson 1976) — https://www.fortross.org/lib/48/russian-america-in-1821.pdf — 1818 reforms, New Archangel census and balance sheet, wages, store markups, statutory fur prices.
- Wikipedia, Russian-American Company — https://en.wikipedia.org/wiki/Russian-American_Company — charters, naval-officer rule, 1808 cost recovery, Astor deal, sale.
- Wikipedia, Company rule in Rhodesia — https://en.wikipedia.org/wiki/Company_rule_in_Rhodesia — LegCo composition over time, 1918 land ruling, 1922 referendum, buyouts.
- Wikipedia, Virginia Company of London — https://en.wikipedia.org/wiki/Virginia_Company_of_London — expectations vs shipments, starving time, assembly, 1624 revocation.
- Wikipedia, Pullman, Chicago; Teaching American History, "Big Trouble in a Company Town" — https://en.wikipedia.org/wiki/Pullman,_Chicago ; https://teachingamericanhistory.org/blog/big-trouble-in-a-company-town-the-pullman-strike/ — company-town controls, 1894 strike, court-ordered divestiture.
- Reymert, "Longyearbyen: From company town to modern town" (Sysselmesteren på Svalbard) — https://www.sysselmesteren.no/contentassets/bc51823074cc440f90894ba798f26a82/gamlelongyearbyen_eng__120114.pdf — 1925 Mining Code obligations, cork money, council ratchet 1948-2002, company school.
- Wikipedia, Royal Greenland Trading Department — https://en.wikipedia.org/wiki/Royal_Greenland_Trading_Department — state monopoly dates, 1782 isolation instruction, 1908-12 separation, Home Rule transfer.
- Tontine Coffee-House, "Medieval Trade Settlement and Credit"; Munro, "The Medieval Bill of Exchange" — https://tontinecoffeehouse.com/2024/04/15/medieval-trade-settlement-and-credit/ ; https://www.economics.utoronto.ca/munro5/BILLEXCH.htm — clearing without real-time communication; fairs as clearing houses.
- Universe Today, "A Habitat at Ceres Could be the Gateway to the Outer Solar System" — https://www.universetoday.com/articles/a-habitat-at-ceres-could-be-the-gateway-to-the-outer-solar-system — Ceres ammonium salts as nitrogen source (Janhunen).
- Glavin et al., "Abundant ammonia and nitrogen-rich soluble organic matter in samples from asteroid (101955) Bennu," Nature Astronomy (2025) — https://pmc.ncbi.nlm.nih.gov/articles/PMC11842271/ — Bennu NH3/N content (not fetched; captcha).
- New Space Economy, "Asteroid Mining Companies" (2025); asapdrew, "The State of Asteroid Mining (2026)" — https://newspaceeconomy.ca/2025/09/20/asteroid-mining-companies/ ; https://www.asapdrew.com/p/asteroid-mining-2026 — AstroForge, Karman+, TransAstra funding and plans.
