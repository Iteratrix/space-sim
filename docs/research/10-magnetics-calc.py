import math
mu0=4e-7*math.pi; mp=1.67e-27; e=1.6e-19
# ---- solar wind ----
for r in (1,2.2,2.7,3.3):
    n=6e6/r**2; v=450e3; p=n*mp*v**2
    Bmp=math.sqrt(2*mu0*p)
    rg=mp*v/(e*Bmp)
    print(f"r={r} AU n={n/1e6:.2f}/cc p_dyn={p*1e9:.2f} nPa B_mp={Bmp*1e9:.1f} nT r_gi={rg/1e3:.0f} km  g_sun={5.93e-3/r**2*1e3:.2f} mm/s2 v_orb={29.8/math.sqrt(r):.1f} km/s")
# ---- magsail (Zubrin-class loop) ----
def magsail(R,I,r,Cd=3.6):
    n=6e6/r**2; v=450e3; p=n*mp*v**2
    m=I*math.pi*R**2
    Bmp=math.sqrt(2*mu0*p)
    L=(mu0*m/(2*math.pi*Bmp))**(1/3)
    F=Cd*0.5*p*math.pi*L**2
    return m,L,F
for R,I in ((32e3,50e3),(10e3,50e3),(50e3,58e3)):
    Je=2e9; A=I/Je; length=2*math.pi*R; mass_tape=length*A*8000
    Lind=mu0*R*(math.log(8*R/0.003)-2); E=0.5*Lind*I**2
    print(f"\nloop R={R/1e3:.0f} km I={I/1e3:.0f} kA tape {mass_tape/1e3:.1f} t (Je 2e9) + shade {0.0288*length/1e3:.1f} t; L={Lind:.2f} H E={E/1e9:.2f} GJ")
    for r in (1,2.2,2.7,3.3):
        m,L,F=magsail(R,I,r)
        print(f"  r={r}: m={m:.2e} A m2 standoff={L/1e3:.0f} km F={F:.1f} N (Freeland/3.1 -> {F/3.1:.1f}); on 1000 t: {F/1e6*1e3:.4f} mm/s2 -> 1 km/s in {1e3/(F/1e6)/86400:.0f} d; per yr {F/1e6*3.156e7/1e3:.2f} km/s")
# ---- plasma magnet from a MAARSS-class coil (R_c=4 m, B_c=1 T) ----
print("\nplasma magnet R_c=4 m B_c=1 T:")
for fo in (1.5,2.0):
    for r in (1,2.7):
        n=6e6/r**2; v=450e3; p=n*mp*v**2; Bmp=math.sqrt(2*mu0*p)
        L=4*(1.0/Bmp)**(1/fo); F=3.6*0.5*p*math.pi*L**2
        print(f"  fo={fo} r={r}: L={L/1e3:.0f} km F={F:.2f} N")
# ---- e-sail, solar sail ----
for r in (2.2,2.7,3.3):
    es=500e-9*r**-1.1*2000e3
    P=2*1361/r**2/3e8
    print(f"r={r}: e-sail 2000 km {es:.2f} N ; solar pressure {P*1e6:.2f} uPa -> 1 km2 foil {P*1e6:.2f} N (13.5 t of 5um Al)")
# ---- radial thrust orbit effect ----
a=0.02e-3; r=2.7*1.496e11; g=1.327e20/r**2; beta=a/g
print(f"\nbeta(0.02 mm/s2 @2.7AU)={beta:.3f}; e_induced~{beta/(1-beta):.3f}; T_orb={2*math.pi*math.sqrt(r**3/1.327e20)/3.156e7:.2f} yr; dv per orbit={a*2*math.pi*math.sqrt(r**3/1.327e20)/1e3:.2f} km/s")
# ---- stored energy of shield coils ----
for B,d,l,nc in ((1,8,20,6),(4,8,20,6),(1.5,16,20,6)):
    V=math.pi*(d/2)**2*l; E=B**2/(2*mu0)*V*nc
    print(f"B={B} T d={d} m l={l} m x{nc}: E={E/1e9:.1f} GJ = {E/4.184e12:.2f} kt TNT")
# ---- REBCO tape composition per 100 t ----
# 4 mm x 0.1 mm tape: 50um Hastelloy, 2x20um Cu, 2x~1.5um Ag, 1um REBCO, 0.2um buffers
t={'Hastelloy(Ni57 Cr16 Mo16 W4 Fe5)':50e-6*8.9,'Cu':40e-6*8.96,'Ag':3e-6*10.5,'YBCO':1e-6*6.3}
tot=sum(t.values()); print("\nREBCO tape mass fractions:",{k:round(v/tot,3) for k,v in t.items()})
# ---- dose model ----
# free-space solar-min barrel baseline ~650 mSv/yr (MAARSS 1977 min); solar-cycle avg ~ x0.75
def fpass(g):  # water, synthesis from 02
    pts=[(0,1),(20,0.65),(50,0.48),(100,0.32),(300,0.10),(1000,0.02)]
    for (g1,f1),(g2,f2) in zip(pts,pts[1:]):
        if g<=g2: return f1+(f2-f1)*(g-g1)/(g2-g1)
    return 0.02
fact={0:1.0,8:0.62,20:0.27,40:0.13}  # MAARSS Table 8.2 mid-band (barrel), relative to 10 g/cm2 case
print("\ndose mSv/yr (solar-min / cycle-avg), 6x10 m bore, endcaps at ~50 g/cm2 PE add ~30-50:")
for BL,fa in fact.items():
    for g in (20,50,100,300):
        comb=fpass(g)*(fa**0.85)  # sub-multiplicative: overlap at low rigidity
        dmin=650*comb+40*(0.5 if BL else 1); davg=0.75*dmin
        print(f"  BL={BL:>2} T m, water {g:>3} g/cm2: {dmin:5.0f} / {davg:5.0f}  ; 600 mSv in {600/davg:.1f} yr; 40-yr life {40*davg/1e3:.1f} Sv -> ~{40*davg/1e3*5:.0f}% excess cancer mortality")
# water mass for 6x10 cylinder
A=2*math.pi*3*10+2*math.pi*9; print(f"\nhull area 6x10 m cylinder {A:.0f} m2: water at 50/100/300/1000 g/cm2 = {[round(A*g*10/1e3) for g in (50,100,300,1000)]} t")
