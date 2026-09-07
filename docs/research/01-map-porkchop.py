import json, math, numpy as np, sys
MU = 1.32712440018e11  # km^3/s^2 sun
AU = 1.495978707e8
DAY = 86400.0
MU_E = 398600.4418; R_LEO = 6378.137+200
MU_CERES = 62.6; R_CERES = 470.0+100   # Ceres GM km3/s2, 100 km orbit
MU_VESTA = 17.29; R_VESTA = 263.0+100
sb = json.load(open("sbdb.json"))
EPOCH_SB = 2461200.5
def elems(name):
    if name=="Earth":
        # J2000 mean elements (ecliptic), epoch JD 2451545.0
        return dict(a=1.00000011*AU, e=0.01671022, i=math.radians(0.00005), om=math.radians(-11.26064), w=math.radians(102.94719+11.26064), M0=math.radians(100.46435-102.94719), epoch=2451545.0)
    if name=="Mars":
        return dict(a=1.52366231*AU, e=0.09341233, i=math.radians(1.85061), om=math.radians(49.57854), w=math.radians(336.04084-49.57854), M0=math.radians(355.45332-336.04084), epoch=2451545.0)
    e = sb[name]["elements"]
    return dict(a=float(e["a"])*AU, e=float(e["e"]), i=math.radians(float(e["i"])), om=math.radians(float(e["om"])), w=math.radians(float(e["w"])), M0=math.radians(float(e["ma"])), epoch=float(sb[name]["epoch"]))
def state(el, jd):
    a,e,i,om,w = el["a"],el["e"],el["i"],el["om"],el["w"]
    n = math.sqrt(MU/a**3)
    M = (el["M0"] + n*(jd-el["epoch"])*DAY) % (2*math.pi)
    E = M
    for _ in range(50):
        E = E - (E-e*math.sin(E)-M)/(1-e*math.cos(E))
    nu = 2*math.atan2(math.sqrt(1+e)*math.sin(E/2), math.sqrt(1-e)*math.cos(E/2))
    r = a*(1-e*math.cos(E)); p = a*(1-e*e); h = math.sqrt(MU*p)
    rp = np.array([r*math.cos(nu), r*math.sin(nu), 0.0])
    vp = np.array([-MU/h*math.sin(nu), MU/h*(e+math.cos(nu)), 0.0])
    co,so,cw,sw,ci,si = math.cos(om),math.sin(om),math.cos(w),math.sin(w),math.cos(i),math.sin(i)
    R = np.array([[co*cw-so*sw*ci, -co*sw-so*cw*ci, so*si],[so*cw+co*sw*ci, -so*sw+co*cw*ci, -co*si],[sw*si, cw*si, ci]])
    return R@rp, R@vp
def C(z):
    z=np.asarray(z,float); out=np.empty_like(z)
    p=z>1e-8; m=z<-1e-8; o=~(p|m)
    out[p]=(1-np.cos(np.sqrt(z[p])))/z[p]; out[m]=(np.cosh(np.sqrt(-z[m]))-1)/(-z[m]); out[o]=0.5
    return out
def S(z):
    z=np.asarray(z,float); out=np.empty_like(z)
    p=z>1e-8; m=z<-1e-8; o=~(p|m)
    sz=np.sqrt(z[p]); out[p]=(sz-np.sin(sz))/sz**3
    sm=np.sqrt(-z[m]); out[m]=(np.sinh(sm)-sm)/sm**3; out[o]=1/6
    return out
def lambert(r1, r2, tofs):
    """Universal-variable Lambert (Curtis Alg 5.2), prograde, 0-rev, vectorized over tofs (s). Returns v1,v2 arrays (N,3)."""
    n1=np.linalg.norm(r1); n2=np.linalg.norm(r2)
    cz=np.cross(r1,r2)[2]
    dth=math.acos(np.clip(np.dot(r1,r2)/(n1*n2),-1,1))
    if cz<0: dth=2*math.pi-dth
    A=math.sin(dth)*math.sqrt(n1*n2/(1-math.cos(dth)))
    N=len(tofs)
    def F(z,t):
        y=n1+n2+A*(z*S(z)-1)/np.sqrt(C(z))
        return (y/C(z))**1.5*S(z)+A*np.sqrt(y)-math.sqrt(MU)*t, y
    lo=np.full(N,-4*math.pi**2); hi=np.full(N,4*math.pi**2*0.999)  # 0-rev: z < (2pi)^2
    # ensure y>0 at lo: shift lo up until y>0
    for _ in range(60):
        _,y=F(lo,tofs); bad=y<0; 
        if not bad.any(): break
        lo[bad]+=0.5
    for _ in range(80):
        mid=(lo+hi)/2; f,_=F(mid,tofs)
        neg=f<0; lo=np.where(neg,mid,lo); hi=np.where(neg,hi,mid)
    z=(lo+hi)/2; f,y=F(z,tofs)
    fg=1-y/n1; g=A*np.sqrt(y/MU); gd=1-y/n2
    v1=(r2[None,:]-fg[:,None]*r1[None,:])/g[:,None]
    v2=(gd[:,None]*r2[None,:]-r1[None,:])/g[:,None]
    ok=np.abs(f)<1e-3*math.sqrt(MU)*tofs  # converged
    return v1,v2,ok
def leo_dv(vinf): return math.sqrt(vinf**2+2*MU_E/R_LEO)-math.sqrt(MU_E/R_LEO)
def capture_dv(vinf, mu, r): return math.sqrt(vinf**2+2*mu/r)-math.sqrt(mu/r)
def porkchop(A_, B_, jd0, ndays, dstep, tof_min, tof_max, tstep, dep_mode="vinf", arr_mode="vinf"):
    ea, eb = elems(A_), elems(B_)
    deps=np.arange(jd0, jd0+ndays, dstep); tofs=np.arange(tof_min, tof_max, tstep)*DAY
    best=[]; besttof=[]; dvd=[]; dva=[]
    for jd in deps:
        r1,vA=state(ea,jd)
        R2=[]; V2=[]
        for t in tofs:
            r2,vB=state(eb,jd+t/DAY); R2.append(r2); V2.append(vB)
        R2=np.array(R2); V2=np.array(V2)
        # need per-arrival r2: lambert is vectorized over tof but r2 differs per tof -> loop in chunks
        v1s=np.empty((len(tofs),3)); v2s=np.empty((len(tofs),3)); oks=np.empty(len(tofs),bool)
        for k,t in enumerate(tofs):
            v1,v2,ok=lambert(r1,R2[k],np.array([t])); v1s[k]=v1[0]; v2s[k]=v2[0]; oks[k]=ok[0]
        vinf1=np.linalg.norm(v1s-vA[None,:],axis=1); vinf2=np.linalg.norm(v2s-V2,axis=1)
        d1 = np.array([leo_dv(v) for v in vinf1]) if dep_mode=="leo" else vinf1
        if arr_mode=="ceres": d2=np.array([capture_dv(v,MU_CERES,R_CERES) for v in vinf2])
        elif arr_mode=="vesta": d2=np.array([capture_dv(v,MU_VESTA,R_VESTA) for v in vinf2])
        else: d2=vinf2
        tot=d1+d2; tot[~oks]=np.inf
        k=int(np.argmin(tot)); best.append(tot[k]); besttof.append(tofs[k]/DAY); dvd.append(d1[k]); dva.append(d2[k])
    return deps, np.array(best), np.array(besttof), np.array(dvd), np.array(dva)
def synodic(A_,B_):
    Ta=math.sqrt(elems(A_)["a"]**3/MU)*2*math.pi/DAY/365.25; Tb=math.sqrt(elems(B_)["a"]**3/MU)*2*math.pi/DAY/365.25
    return abs(1/(1/Ta-1/Tb)), Ta, Tb
def hohmann(A_,B_):
    ra=elems(A_)["a"]; rb=elems(B_)["a"]; 
    va=math.sqrt(MU/ra); vb=math.sqrt(MU/rb); at=(ra+rb)/2
    dv1=abs(math.sqrt(MU*(2/ra-1/at))-va); dv2=abs(vb-math.sqrt(MU*(2/rb-1/at)))
    tof=math.pi*math.sqrt(at**3/MU)/DAY
    return dv1,dv2,tof
def edelbaum(A_,B_):
    ea,eb=elems(A_),elems(B_); va=math.sqrt(MU/ea["a"]); vb=math.sqrt(MU/eb["a"]); di=abs(ea["i"]-eb["i"])
    return math.sqrt(va*va+vb*vb-2*va*vb*math.cos(math.pi/2*di))
if __name__=="__main__":
    JD0=2461041.5  # 2026-01-01 approx
    pairs=[("Earth","Ceres","leo","ceres",4000,8,150,900,8),("Earth","Vesta","leo","vesta",4000,8,150,900,8),
           ("Earth","Psyche","leo","vinf",4000,8,150,1000,8),
           ("Ceres","Vesta","vinf","vinf",4000,10,100,1500,10),("Vesta","Ceres","vinf","vinf",4000,10,100,1500,10),
           ("Ceres","Pallas","vinf","vinf",4000,10,100,1500,10),("Ceres","Hygiea","vinf","vinf",6000,10,100,1800,10),
           ("Ceres","Psyche","vinf","vinf",6000,10,100,1800,10),("Vesta","Psyche","vinf","vinf",4000,10,100,1500,10),
           ("Ceres","Thisbe","vinf","vinf",6000,10,100,1800,10),("Ceres","Themis","vinf","vinf",6000,10,100,1800,10),
           ("Vesta","Hebe","vinf","vinf",4000,10,100,1500,10),("Vesta","Flora","vinf","vinf",4000,10,100,1500,10),
           ("Earth","2000 FJ10","leo","vinf",6000,5,60,600,5),("Earth","Bennu","leo","vinf",4000,5,60,600,5),
           ("Earth","2008 EV5","leo","vinf",4000,5,60,600,5),("Earth","Ryugu","leo","vinf",4000,5,60,600,5),
           ("Earth","Mars","leo","vinf",2000,5,100,500,5),("Earth","1996 FG3","leo","vinf",4000,5,60,600,5),
           ("Ceres","Earth","vinf","vinf",4000,8,150,900,8)]
    res={}
    for A_,B_,dm,am,nd,ds,t0,t1,ts in pairs:
        syn,Ta,Tb=synodic(A_,B_); h=hohmann(A_,B_); ed=edelbaum(A_,B_)
        deps,best,btof,d1,d2=porkchop(A_,B_,JD0,nd,ds,t0,t1,ts,dm,am)
        fin=np.isfinite(best)
        b=best[fin]; 
        # stats over one full synodic cycle (or whole span if shorter)
        pct=np.percentile(b,[0,10,25,50,75,90,100])
        # fraction of departure dates within +20% and +50% of global min
        f20=np.mean(b<=pct[0]*1.2); f50=np.mean(b<=pct[0]*1.5)
        k=int(np.argmin(best)); 
        res[f"{A_}->{B_}"]=dict(synodic_yr=syn,T_A=Ta,T_B=Tb,hohmann_dv=h[0]+h[1],hohmann_tof_d=h[2],edelbaum=ed,
            best=pct[0],p10=pct[1],p25=pct[2],p50=pct[3],p75=pct[4],p90=pct[5],worst=pct[6],frac_within20=f20,frac_within50=f50,
            best_tof_d=btof[k],best_dep_jd=deps[k],best_d1=d1[k],best_d2=d2[k],
            series=[(float(x),float(y),float(z)) for x,y,z in zip(deps,best,btof)])
        print(f"{A_:8s}->{B_:10s} syn={syn:5.2f}yr Hoh={h[0]+h[1]:5.2f} ({h[2]:.0f}d) Edel={ed:5.2f} | best={pct[0]:5.2f} p25={pct[2]:5.2f} med={pct[3]:5.2f} p75={pct[4]:5.2f} worst={pct[6]:5.2f} | within20%={f20:.2f} within50%={f50:.2f} | tof@best={btof[k]:.0f}d dep={d1[k]:.2f} arr={d2[k]:.2f}", flush=True)
    json.dump(res,open("porkchop_results.json","w"))
