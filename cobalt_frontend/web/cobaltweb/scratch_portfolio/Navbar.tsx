import { useEffect, useState } from "react";

const navItems = ["Home", "Services", "Projects", "Contact"];

const Navbar = () => {
  const [menuOpen, setMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    // Trigger slide-down animation
    const t = setTimeout(() => setMounted(true), 50);
    return () => clearTimeout(t);
  }, []);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 20);
    window.addEventListener("scroll", onScroll);
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <nav
      className={`fixed top-0 left-0 w-full z-50 transition-all duration-500
        ${mounted ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-4"}
        ${scrolled
          ? "border-b border-purple-500/20 shadow-[0_4px_32px_rgba(0,0,0,0.4)]"
          : "border-b border-purple-500/10"
        }
      `}
      style={{
        background: scrolled ? "rgba(7,7,15,0.88)" : "rgba(11,9,28,0.62)",
        backdropFilter: "blur(16px)",
        WebkitBackdropFilter: "blur(16px)",
      }}
    >
      {/* ── Top bar ── */}
      <div className="w-full flex justify-between items-center px-8 sm:px-12 md:px-16 lg:px-20 xl:px-28 py-4">

        {/* Logo */}
        <span
          className="text-2xl font-black tracking-tight cursor-pointer select-none
            bg-linear-to-r from-purple-400 via-purple-500 to-indigo-400
            bg-clip-text text-transparent
            drop-shadow-[0_0_10px_rgba(168,85,247,0.4)]
            hover:drop-shadow-[0_0_18px_rgba(168,85,247,0.75)]
            transition-all duration-300"
        >
          Nabila.dev
        </span>

        {/* Desktop nav links */}
        <ul className="hidden md:flex items-center gap-8">
          {navItems.map((item) => (
            <li
              key={item}
              onClick={() => {
                const section = document.getElementById(item.toLowerCase());
                section?.scrollIntoView({ behavior: "smooth" });
          }}
              className="relative list-none text-[0.95rem] font-medium text-slate-400
                cursor-pointer transition-colors duration-200
               hover:text-purple-200
                after:content-[''] after:absolute after:left-0 after:-bottom-0.5
                after:h-0.5 after:w-0 after:rounded-full
                after:bg-linear-to-r after:from-purple-500 after:to-indigo-400
                after:shadow-[0_0_6px_rgba(168,85,247,0.6)]
                after:transition-[width] after:duration-300
                hover:after:w-full"
              >
              {item}
          </li>
          ))}
        </ul>

        {/* Desktop CTA */}
        <a
          href="#contact"
          className="hidden md:inline-flex items-center gap-2
            px-5 py-2.5 rounded-xl text-[0.9rem] font-semibold text-white
            bg-linear-to-r from-purple-600 to-violet-700
            shadow-[0_0_16px_rgba(147,51,234,0.35)]
            hover:shadow-[0_0_28px_rgba(168,85,247,0.6)]
            hover:-translate-y-0.5 hover:scale-[1.03]
            transition-all duration-200"
        >
          Hire Me ✦
        </a>

        {/* Hamburger (mobile) */}
        <button
          className="md:hidden flex flex-col justify-center gap-1.5  p-1 bg-transparent border-0 cursor-pointer"
          onClick={() => setMenuOpen(!menuOpen)}
          aria-label="Toggle menu"
        >
          {/* Line 1 */}
          <span
            className={`block h-0.5 bg-purple-500 rounded-full transition-all duration-300 origin-center
              ${menuOpen ? "w-5.5 translate-y-2 rotate-45" : "w-5.5"}`}
          />
          {/* Line 2 */}
          <span
            className={`block h-0.5 bg-purple-500 rounded-full transition-all duration-300
              ${menuOpen ? "w-0 opacity-0" : "w-5.5 opacity-100"}`}
          />
          {/* Line 3 */}
          <span
            className={`block h-0.5  bg-purple-500 rounded-full transition-all duration-300 origin-center
              ${menuOpen ? "w-5.5 -translate-y-2 -rotate-45" : "w-5.5"}`}
          />
        </button>
      </div>

      {/* ── Mobile dropdown ── */}
      <div
        className={`md:hidden overflow-hidden transition-all duration-300 ease-in-out
          ${menuOpen ? "max-h-100 opacity-100" : "max-h-0 opacity-0"}`}
        style={{
          background: "rgba(9,7,20,0.97)",
          borderTop: menuOpen ? "1px solid rgba(139,92,246,0.15)" : "none",
        }}
      >
        {/* Glowing divider */}
        <div className="mx-6 mt-3 mb-4 h-px bg-linear-to-r from-transparent via-purple-500/40 to-transparent" />

        <div className="px-6 pb-5 flex flex-col gap-1">
          {navItems.map((item) => (
            <div
              key={item}
              className="flex items-center gap-3 px-4 py-3 rounded-xl
                text-[0.95rem] font-medium text-slate-400 cursor-pointer
                hover:bg-purple-500/10 hover:text-purple-200
                transition-all duration-200"
              onClick={() => setMenuOpen(false)}
            >
              {/* Dot accent */}
              <span className="inline-block w-1.5 h-1.5 rounded-full shrink-0 bg-linear-to-br from-purple-500 to-indigo-400" />
              {item}
            </div>
          ))}

          {/* Mobile CTA */}
          <div className="mt-3 pt-4 border-t border-purple-500/10">
            <button
              className="w-full py-3 rounded-xl text-[0.93rem] font-semibold text-white
                bg-linear-to-br from-purple-600 to-violet-700
                shadow-[0_0_20px_rgba(147,51,234,0.35)]
                hover:shadow-[0_0_30px_rgba(168,85,247,0.5)]
                transition-all duration-200"
              onClick={() => setMenuOpen(false)}
            >
              Hire Me ✦
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;