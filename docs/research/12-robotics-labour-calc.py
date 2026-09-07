"""Salotti 2020 time-budget with automation fractions.
Hours are per Martian year (16,487 h). Capacity = 0.3125 * 16487 * n = 5152 n.
Demand_i = r_i * n / n^a_i = r_i * n^(1-a_i).
Automation: fraction A of activity's hours moved to robots; robot hours = A*demand*k
(k = robot hours per human hour replaced). Robot units R = robot_hours / H_robot.
Maintenance hours (human) = R * m. Solve smallest n with demand_h + maint < capacity.
"""
import math
CAP=5152.0; HMY=16487.0
# (name, r, a, premium?)  premium = human hand/judgment irreplaceable on decades horizon
ACT=[
 ("agriculture",940,0.3),("air mgmt",380,0.7),("water mgmt",750,0.7),
 ("living organisms",940,0.5),("organic wastes",940,0.5),("agronomy",940,0.4),
 ("eco other/innov",1880,0.5),
 ("PV production",1880,0.6),("electricity mgmt",1880,0.6),("thermal mgmt",380,0.6),
 ("methane prod",940,0.6),("energy other/innov",1880,0.6),
 ("mining",3802,0.6),("metal production",5640,0.6),("metallic objects",5640,0.6),
 ("ceramics/glass",1880,0.6),("chemical industry",5640,0.6),("clothes",1880,0.6),
 ("industry other/innov",5640,0.6),
 ("concrete",380,0.7),("construction",940,0.7),("equipping buildings",940,0.5),
 ("building maintenance",1880,0.4),("building other/innov",1880,0.5),
 ("raising babies",1500,0.3),("education",1500,0.6),("health care",1500,0.1),
 ("meal prep",560,0.6),("social org",940,0.2),("sport/entertain",940,1.0),
 ("social other/innov",1880,0.3),
]
def demand(n): return sum(r*n**(1-a) for _,r,a in ACT)
def nmin(f):
    for n in range(2,5000):
        if f(n)<CAP*n: return n
    return None
print("Salotti baseline (no robots): n_min =",nmin(demand))
n=110
print("shares at n=110 (h/Martian yr, % of demand):")
D=demand(n)
for name,r,a in ACT:
    d=r*n**(1-a); print(f"  {name:22s} {d:8.0f} {100*d/D:5.1f}%  per-cap {d/n:6.0f}")
print(f"  total demand {D:.0f}  capacity {CAP*n:.0f}")

# Automation fractions per activity per scenario: (A, k)
# scenarios: T1a T2a T3a T4a (sponsor alive, spares flow) ; T2d T3d T4d (sponsor dead, belt-repairable fleet only)
S=["T1a","T2a","T3a","T4a","T2d","T3d","T4d"]
A={ # fraction automatable
 "agriculture":       [.5,.6,.7,.8, .3,.4,.6],
 "air mgmt":          [.8,.8,.8,.9, .5,.6,.7],
 "water mgmt":        [.8,.8,.8,.9, .5,.6,.7],
 "living organisms":  [.2,.2,.3,.4, .1,.1,.2],
 "organic wastes":    [.6,.6,.7,.8, .4,.5,.6],
 "agronomy":          [.1,.1,.2,.3, .0,.1,.1],
 "eco other/innov":   [.2,.2,.3,.3, .1,.1,.2],
 "PV production":     [.7,.7,.8,.9, .3,.5,.6],
 "electricity mgmt":  [.8,.8,.9,.9, .6,.7,.8],
 "thermal mgmt":      [.8,.8,.9,.9, .6,.7,.8],
 "methane prod":      [.8,.8,.9,.9, .5,.6,.7],
 "energy other/innov":[.2,.2,.3,.3, .1,.1,.2],
 "mining":            [.8,.85,.9,.9, .5,.6,.7],
 "metal production":  [.7,.8,.85,.9, .4,.5,.6],
 "metallic objects":  [.4,.5,.6,.7, .1,.2,.4],
 "ceramics/glass":    [.5,.6,.7,.8, .2,.3,.5],
 "chemical industry": [.7,.8,.85,.9, .4,.5,.6],
 "clothes":           [.5,.6,.7,.8, .2,.3,.5],
 "industry other/innov":[.2,.2,.3,.3,.1,.1,.2],
 "concrete":          [.7,.8,.8,.9, .4,.5,.6],
 "construction":      [.3,.4,.5,.6, .1,.2,.3],
 "equipping buildings":[.1,.2,.3,.4,.0,.1,.2],
 "building maintenance":[.1,.1,.2,.3,.0,.0,.1],
 "building other/innov":[.1,.1,.2,.2,.0,.0,.1],
 "raising babies":    [0,0,0,0,0,0,0],
 "education":         [.3,.3,.4,.4, .1,.2,.2],
 "health care":       [.1,.1,.15,.2, .0,.05,.1],
 "meal prep":         [.3,.3,.4,.5, .1,.2,.3],
 "social org":        [.2,.2,.2,.2, .1,.1,.1],
 "sport/entertain":   [0,0,0,0,0,0,0],
 "social other/innov":[.1,.1,.1,.1, .0,.0,.0],
}
K=1.0            # robot hours per human hour replaced (blend of faster haulage, slower dexterous)
HROB=6000.0      # productive robot hours per Martian year (~36% duty; mining fleets ~85% avail of 2 shifts)
MAINT={"a":400.0,"d":1500.0}  # human maintenance hours per robot-unit per Martian year
def scenario(i):
    dead = S[i].endswith("d")
    m=MAINT["d" if dead else "a"]
    def f(n):
        hum=0; rob=0
        for name,r,a in ACT:
            d=r*n**(1-a); frac=A[name][i]
            hum+=d*(1-frac); rob+=d*frac*K
        R=rob/HROB
        return hum + R*m
    return f
print("\nscenario  n_min  (robots at n_min, human h, robot h, maint h) and at n=110: robots, human-hours, share automated")
for i,s in enumerate(S):
    f=scenario(i); nm=nmin(f)
    def parts(n):
        hum=rob=0
        for name,r,a in ACT:
            d=r*n**(1-a); frac=A[name][i]; hum+=d*(1-frac); rob+=d*frac*K
        R=rob/HROB; m=MAINT["d" if s.endswith("d") else "a"]
        return hum,rob,R,R*m
    h,rb,R,mt=parts(nm); h2,rb2,R2,mt2=parts(110)
    print(f"{s:5s} n_min={nm:4d}  R={R:5.1f} hum={h:7.0f} rob={rb:7.0f} maint={mt:6.0f} | n=110: R={R2:5.1f} hum={h2:7.0f} rob={rb2:7.0f} maint={mt2:6.0f} auto={rb2/(h2+rb2):.0%} people/robot={110/R2:.1f}")
# sensitivity: maintenance per robot
print("\nsensitivity (T2a): maint h/robot -> n_min")
for m in [200,400,800,1500,3000]:
    MAINT["a"]=m; print(f"  m={m:5d}  n_min={nmin(scenario(1))}")
MAINT["a"]=400
print("\nsensitivity: 'bits fully automated' only (planning/inventory/reckoning ~ 10% of every activity's hours), no hands:")
def bits(n): return sum(r*n**(1-a)*0.9 for _,r,a in ACT)
print("  n_min =",nmin(bits))
print("hands fully automated but premium activities kept human (health, babies, social, maintenance, agronomy, living organisms):")
prem={"raising babies","health care","social org","sport/entertain","social other/innov","building maintenance","agronomy","living organisms","equipping buildings"}
def hands(n): return sum(r*n**(1-a)*(1.0 if nm in prem else 0.0) for nm,r,a in ACT)
print("  n_min =",nmin(hands), " (premium demand per capita at n=110:", round(hands(110)/110),"h vs capacity 5152)")

print("\nGrouped table at n=110, hours per Martian year: demand | T2a human/robot | T3d human/robot")
G={"Farm & biology (agri, organisms, wastes, agronomy, eco-other)":["agriculture","living organisms","organic wastes","agronomy","eco other/innov"],
   "Air & water loops":["air mgmt","water mgmt"],
   "Power (PV, electricity, thermal, methane, other)":["PV production","electricity mgmt","thermal mgmt","methane prod","energy other/innov"],
   "Mining":["mining"],"Metal production":["metal production"],"Metallic objects (fabrication)":["metallic objects"],
   "Chemical industry":["chemical industry"],"Glass/ceramics + clothes":["ceramics/glass","clothes"],
   "Industry other/innovation":["industry other/innov"],
   "Construction & concrete & equipping":["concrete","construction","equipping buildings"],
   "Building maintenance + other":["building maintenance","building other/innov"],
   "Raising children + education":["raising babies","education"],"Health care":["health care"],
   "Meals, social org, culture, other":["meal prep","social org","sport/entertain","social other/innov"]}
n=110
tot=[0,0,0,0,0]
for g,names in G.items():
    d=h1=r1=h2=r2=0
    for nm in names:
        r,a=[(rr,aa) for x,rr,aa in ACT if x==nm][0]
        dd=r*n**(1-a); d+=dd
        h1+=dd*(1-A[nm][1]); r1+=dd*A[nm][1]
        h2+=dd*(1-A[nm][5]); r2+=dd*A[nm][5]
    tot=[t+v for t,v in zip(tot,[d,h1,r1,h2,r2])]
    print(f"{g:62s} {d:7.0f} | {h1:7.0f} {r1:7.0f} | {h2:7.0f} {r2:7.0f}")
print("TOTAL",[round(t) for t in tot], " capacity", CAP*n)
