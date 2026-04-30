import { useEffect, useRef, useState } from "react";
import profile from "../assets/hero.png";

const TITLES = [
  "Frontend Developer",
  "Creative Problem Solver",
  "UI/UX Enthusiast",
  "React Specialist",
];

const Hero = () => {
  const [titleIndex, setTitleIndex] = useState(0);
  const [displayed, setDisplayed] = useState("");
  const [isDeleting, setIsDeleting] = useState(false);
  const [mounted, setMounted] = useState(false);
  const typingRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const t = setTimeout(() => setMounted(true), 100);
    return () => clearTimeout(t);
  }, []);

  useEffect(() => {
    const current = TITLES[titleIndex];
    const speed = isDeleting ? 40 : 80;

    typingRef.current = setTimeout(() => {
      if (!isDeleting && displayed.length < current.length) {
        setDisplayed(current.slice(0, displayed.length + 1));
      } else if (!isDeleting && displayed.length === current.length) {
        setTimeout(() => setIsDeleting(true), 1800);
      } else if (isDeleting && displayed.length > 0) {
        setDisplayed(current.slice(0, displayed.length - 1));
      } else if (isDeleting && displayed.length === 0) {
        setIsDeleting(false);
        setTitleIndex((i) => (i + 1) % TITLES.length);
      }
    }, speed);

    return () => {
      if (typingRef.current) clearTimeout(typingRef.current);
    };
  }, [displayed, isDeleting, titleIndex]);

  return (
    <>
      {/* Keyframe animations — only what Tailwind can't do inline */}
      <style>{`
        @keyframes floatA    { 0%,100%{transform:translate(0,0) scale(1)}   50%{transform:translate(3%,4%) scale(1.05)} }
        @keyframes floatB    { 0%,100%{transform:translate(0,0) scale(1)}   50%{transform:translate(-3%,-4%) scale(1.06)} }
        @keyframes floatImg  { 0%,100%{transform:translateY(0)}              50%{transform:translateY(-12px)} }
        @keyframes floatBadge{ 0%,100%{transform:translateY(0)}              50%{transform:translateY(-5px)} }
        @keyframes glowPulse { 0%,100%{opacity:.65;transform:scale(1)}       50%{opacity:1;transform:scale(1.1)} }
        @keyframes blink     { 0%,100%{opacity:1}                            50%{opacity:0} }
        @keyframes dotPulse  { 0%,100%{box-shadow:0 0 6px #a855f7}           50%{box-shadow:0 0 14px #a855f7;opacity:.6} }
        @keyframes shimmer   { 0%{transform:translateX(-100%)} 100%{transform:translateX(100%)} }

        .animate-floatA    { animation: floatA    8s  ease-in-out infinite; }
        .animate-floatB    { animation: floatB    10s ease-in-out infinite; }
        .animate-floatImg  { animation: floatImg  6s  ease-in-out infinite; }
        .animate-floatBadgeA { animation: floatBadge 4s ease-in-out infinite; }
        .animate-floatBadgeB { animation: floatBadge 5s ease-in-out infinite reverse; }
        .animate-glowPulse { animation: glowPulse 3s  ease-in-out infinite; }
        .animate-blink     { animation: blink     1s  step-end   infinite; }
        .animate-dotPulse  { animation: dotPulse  2s             infinite; }

        /* Shimmer button */
        .btn-shimmer::after {
          content:'';
          position:absolute;
          inset:0;
          background:linear-gradient(105deg,transparent 30%,rgba(255,255,255,0.15) 50%,transparent 70%);
          opacity:0;
          transition:opacity 0.3s;
        }
        .btn-shimmer:hover::after { opacity:1; }

        /* Gradient text — Tailwind can't do bg-clip:text reliably without arbitrary values */
        .gradient-name {
          background: linear-gradient(135deg, #c084fc 0%, #a855f7 35%, #7c3aed 65%, #818cf8 100%);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
          background-clip: text;
          filter: drop-shadow(0 0 20px rgba(168,85,247,0.5));
        }
        .gradient-stat {
          background: linear-gradient(135deg,#c084fc,#818cf8);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
          background-clip: text;
        }

        /* Image mask blend */
        .img-mask {
          -webkit-mask-image: linear-gradient(to bottom, black 50%, transparent 100%);
          mask-image: linear-gradient(to bottom, black 50%, transparent 100%);
        }
      `}</style>

      {/* ─────────────────────────────────────────────
          SECTION — full-viewport, full-width
      ───────────────────────────────────────────── */}
      <section
        id="home"
        className="pt-14 relative w-full min-h-screen flex items-center overflow-hidden"
        style={{ background: "linear-gradient(135deg,#07070F 0%,#0F0B1E 40%,#150D2E 70%,#1A0B33 100%)" }}
      >

        {/* ── Ambient blobs ── */}
        <div
          className="animate-floatA pointer-events-none absolute -top-[10%] -left-[5%] w-[50vw] h-[50vw] rounded-full"
          style={{ background:"radial-gradient(circle,rgba(139,92,246,0.18) 0%,transparent 70%)", filter:"blur(70px)" }}
        />
        <div
          className="animate-floatB pointer-events-none absolute -bottom-[15%] -right-[5%] w-[55vw] h-[55vw] rounded-full"
          style={{ background:"radial-gradient(circle,rgba(168,85,247,0.22) 0%,rgba(99,102,241,0.1) 40%,transparent 70%)", filter:"blur(90px)" }}
        />
        {/* Glow behind image (right side) */}
        <div
          className="pointer-events-none absolute top-1/2 right-[8%] -translate-y-1/2 w-[40vw] h-[40vw]max-w-130 max-w-130 rounded-full"
          style={{ background:"radial-gradient(circle,rgba(168,85,247,0.28) 0%,rgba(109,40,217,0.1) 45%,transparent 70%)", filter:"blur(65px)" }}
        />
        {/* Noise grain */}
        <div
          className="pointer-events-none absolute inset-0 opacity-[0.03]"
          style={{ backgroundImage:`url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E")`, backgroundSize:"180px" }}
        />

        {/* ─────────────────────────────────────────
            INNER — full width, two-col on desktop
        ───────────────────────────────────────── */}
        <div className="relative z-10 w-full flex flex-col-reverse lg:flex-row items-center justify-between gap-10 lg:gap-14 px-4 sm:px-10 md:px-16 lg:px-20 xl:px-28 2xl:px-36 py-20 lg:py-0 lg:min-h-screen">

          {/* ── LEFT: Text ── */}
          <div className="w-full lg:w-1/2 flex flex-col items-center lg:items-start text-center lg:text-left">

            {/* Available badge */}
            <div
              className="inline-flex items-center gap-2 mb-4 px-4 py-1.5 rounded-full border text-[0.68rem] font-bold tracking-[0.12em] uppercase backdrop-blur-sm"
              style={{
                borderColor:"rgba(139,92,246,0.4)",
                background:"rgba(139,92,246,0.08)",
                color:"#c084fc",
                opacity: mounted ? 1 : 0,
                transform: mounted ? "translateY(0)" : "translateY(14px)",
                transition:"opacity 0.7s ease 0.1s, transform 0.7s ease 0.1s",
              }}
            >
              <span className="animate-dotPulse w-1.75 h-1.75 rounded-full bg-purple-500 shrink-0" />
              Available for work
            </div>

            {/* Name heading */}
            <h1
              className="font-black leading-[1.05] mb-2"
              style={{
                fontSize:"clamp(2.25rem, 5vw, 3.8rem)",
                opacity: mounted ? 1 : 0,
                transform: mounted ? "translateY(0)" : "translateY(18px)",
                transition:"opacity 0.8s ease 0.2s, transform 0.8s ease 0.2s",
              }}
            >
              <span className="text-white">Hi, I'm </span>
              <br />
              <span className="gradient-name">Nabila Tamboli</span>
            </h1>

            {/* Typing subtitle */}
            <h2
              className="font-semibold mb-4 min-h-[1.8rem]"
              style={{
                fontSize:"clamp(1rem, 2vw, 1.25rem)",
                color:"#94a3b8",
                opacity: mounted ? 1 : 0,
                transform: mounted ? "translateY(0)" : "translateY(14px)",
                transition:"opacity 0.8s ease 0.35s, transform 0.8s ease 0.35s",
              }}
            >
              <span style={{ color:"#c084fc" }}>{displayed}</span>
              <span
                className="animate-blink inline-block w-0.5 h-[1em] bg-purple-500 ml-0.5 rounded-sm align-middle"
              />
            </h2>

            {/* Description */}
            <p
              className="mb-6 leading-relaxed max-w-110"
              style={{
                color:"#64748b",
                fontSize:"clamp(0.875rem, 1.2vw, 0.95rem)",
                opacity: mounted ? 1 : 0,
                transform: mounted ? "translateY(0)" : "translateY(14px)",
                transition:"opacity 0.8s ease 0.5s, transform 0.8s ease 0.5s",
              }}
            >
              I help businesses build modern websites, improve user experience, and create digital solutions that drive results.
            </p>

            {/* Buttons */}
            <div
              className="flex flex-wrap gap-4 justify-center lg:justify-start mb-6"
              style={{
                opacity: mounted ? 1 : 0,
                transform: mounted ? "translateY(0)" : "translateY(14px)",
                transition:"opacity 0.8s ease 0.62s, transform 0.8s ease 0.62s",
              }}
            >
              {/* Primary */}
              <a
                href="#projects"
                className="btn-shimmer relative overflow-hidden px-7 py-3 rounded-xl font-semibold text-white text-[0.95rem] border-none cursor-pointer transition-all duration-200 hover:-translate-y-0.5 hover:scale-[1.03]"
                style={{
                  background:"linear-gradient(135deg,#9333ea,#7c3aed)",
                  boxShadow:"0 0 22px rgba(147,51,234,0.42), 0 4px 20px rgba(0,0,0,0.35)",
                }}
              >
                View Projects
              </a>

              {/* Outline */}
              <a
                href="#contact" 
                className="px-7 py-3 rounded-xl font-semibold text-[0.95rem] cursor-pointer transition-all duration-200 hover:-translate-y-0.5 backdrop-blur-sm"
                style={{
                  color:"#c084fc",
                  background:"rgba(139,92,246,0.07)",
                  border:"1px solid rgba(168,85,247,0.4)",
                }}
              >
                Contact Me
              </a>
            </div>

            {/* Stats */}
            <div
              className="flex gap-8 justify-center lg:justify-start"
              style={{
                opacity: mounted ? 1 : 0,
                transition:"opacity 0.8s ease 0.8s",
              }}
            >
              {[
                { num: "100%", label: "Project Completion Rate" },
                { num:"10+", label:"Projects"   },
                { num:"5+", label:"Clients"    },
              ].map(({ num, label }) => (
                <div key={label} className="flex flex-col">
                  <span className="gradient-stat font-black text-2xl">{num}</span>
                  <span className="text-slate-500 text-xs mt-0.5 tracking-wide">{label}</span>
                </div>
              ))}
            </div>
          </div>

          {/* ── RIGHT: Image ── */}
          <div
            className="w-full lg:w-1/2 flex justify-center lg:justify-end items-center"
            style={{
              opacity: mounted ? 1 : 0,
              transform: mounted ? "translateY(0)" : "translateY(28px)",
              transition:"opacity 0.9s ease 0.3s, transform 0.9s ease 0.3s",
            }}
          >
            <div className="animate-floatImg relative">

              {/* Outer gradient ring */}
              <div
                className="absolute -inset-3 rounded-[28px]"
                style={{ background:"linear-gradient(135deg,rgba(168,85,247,0.45),rgba(99,102,241,0.18),transparent 60%)", filter:"blur(1px)" }}
              />

              {/* Image box */}
              <div
                className="relative overflow-hidden rounded-[22px] border border-purple-500/30 group"
                style={{
                  width:"clamp(240px, 38vw, 420px)",
                  height:"clamp(240px, 38vw, 420px)",
                  boxShadow:"0 0 0 1px rgba(139,92,246,0.12), 0 32px 64px rgba(0,0,0,0.55)",
                }}
              >
                {/* Glow */}
                <div
                  className="animate-glowPulse absolute inset-0 rounded-[22px]"
                  style={{ background:"radial-gradient(ellipse at center,rgba(147,51,234,0.48) 0%,transparent 70%)", filter:"blur(35px)" }}
                />

                {/* Photo */}
                <img
                  src={profile}
                  alt="Nabila Tamboli – Developer"
                  className="img-mask w-full h-full object-cover mix-blend-luminosity brightness-105 saturate-[1.15] transition-all duration-500 group-hover:scale-105 group-hover:brightness-110 group-hover:saturate-[1.3]"
                />

                {/* Bottom fade overlay blending into bg */}
                <div
                  className="absolute inset-x-0 bottom-0 h-1/2 pointer-events-none"
                  style={{ background:"linear-gradient(to top,rgba(21,13,46,0.97) 0%,transparent 100%)" }}
                />
              </div>

              {/* Badge: top-right */}
              <div
                className="animate-floatBadgeA absolute -top-4 -right-4 px-2 py-2 rounded-xl text-[0.90rem] font-bold backdrop-blur-md"
                style={{
                  background:"rgba(10,8,24,0.9)",
                  border:"1px solid rgba(168,85,247,0.45)",
                  color:"#c084fc",
                  boxShadow:"0 0 16px rgba(168,85,247,0.25)",
                }}
              >
                ✦ Open to work
              </div>         

            </div>
          </div>

        </div>
      </section>
    </>
  );
};

export default Hero;