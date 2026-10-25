import { useState } from "react";

// ── Import your local video files ──
import realestate_demo from "../assets/videos/realestate_demo.mp4";
import adVideo1 from "../assets/videos/ad-video-1.mp4";
import adVideo2 from "../assets/videos/ad-video-2.mp4";

/* ── Types ── */
type ProjectType = "demo" | "live" | "video";

interface Project {
  title: string;
  description: string;
  tech: string[];
  link?: string;
  videoSrc?: string;
  type: ProjectType;
  highlight: string;
  number: string;
  icon: React.ReactNode;
  previewBg: string;
}

/* ── Data ── */
const projects: Project[] = [
  {
    title: "Tailor CRM System",
    description:
      "A management system for tailoring businesses to manage customers, orders, and measurements efficiently. Built with a clean dashboard UI and real-time order tracking.",
    tech: ["React", "Node.js", "MongoDB"],
    type: "demo",
    highlight: "Business Solution",
    number: "01",
    icon: (
      <svg viewBox="0 0 24 24" fill="none" className="w-6 h-6" stroke="currentColor" strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round">
        <path d="M9 3H5a2 2 0 00-2 2v4m6-6h10a2 2 0 012 2v4M9 3v18m0 0h10a2 2 0 002-2V9M9 21H5a2 2 0 01-2-2V9m0 0h18" />
      </svg>
    ),
    previewBg: "from-purple-900/60 via-violet-800/40 to-indigo-900/60",
  },
  {
    title: "WesternFlex Website",
    description:
      "Contributed to frontend development of the official company website with responsive UI and modern design. Live production site serving real users.",
    tech: ["React", "TypeScript"],
    link: "https://westernflex.com/",
    type: "live",
    highlight: "Real Client Work",
    number: "02",
    icon: (
      <svg viewBox="0 0 24 24" fill="none" className="w-6 h-6" stroke="currentColor" strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round">
        <circle cx="12" cy="12" r="10" />
        <path d="M2 12h20M12 2a15.3 15.3 0 010 20M12 2a15.3 15.3 0 000 20" />
      </svg>
    ),
    previewBg: "from-indigo-900/60 via-purple-800/40 to-violet-900/60",
  },
  {
    title: "Real Estate Website",
    description:
      "Developed a real estate website prototype during internship to demonstrate UI/UX and property listing features with advanced search and filtering.",
    tech: ["React", "Tailwind CSS"],
    videoSrc: realestate_demo,
    type: "demo",
    highlight: "Client Prototype",
    number: "03",
    icon: (
      <svg viewBox="0 0 24 24" fill="none" className="w-6 h-6" stroke="currentColor" strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round">
        <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
        <path d="M9 22V12h6v10" />
      </svg>
    ),
    previewBg: "from-violet-900/60 via-purple-800/40 to-fuchsia-900/60",
  },
  {
    title: "Brand Ad — Campaign 1",
    description:
      "Promotional ad video created for a client to boost brand visibility and engagement across social media platforms.",
    tech: ["Video Editing", "After Effects", "Creative Direction"],
    videoSrc: adVideo1,
    type: "video",
    highlight: "Ad Video",
    number: "04",
    icon: (
      <svg viewBox="0 0 24 24" fill="none" className="w-6 h-6" stroke="currentColor" strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round">
        <rect x="2" y="4" width="15" height="16" rx="2" />
        <path d="M22 8l-5 4 5 4V8z" />
      </svg>
    ),
    previewBg: "from-fuchsia-900/60 via-purple-800/40 to-violet-900/60",
  },
  {
    title: "Brand Ad — Campaign 2",
    description:
      "Second promotional ad reel crafted to enhance client brand identity and drive audience engagement.",
    tech: ["Video Editing", "Motion Graphics", "Creative Direction"],
    videoSrc: adVideo2,
    type: "video",
    highlight: "Ad Video",
    number: "05",
    icon: (
      <svg viewBox="0 0 24 24" fill="none" className="w-6 h-6" stroke="currentColor" strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round">
        <rect x="2" y="4" width="15" height="16" rx="2" />
        <path d="M22 8l-5 4 5 4V8z" />
      </svg>
    ),
    previewBg: "from-purple-900/60 via-fuchsia-800/40 to-indigo-900/60",
  },
];

/* ── Video Modal ── */
const VideoModal = ({
  project,
  onClose,
}: {
  project: Project;
  onClose: () => void;
}) => (
  <div
    className="fixed inset-0 z-100 flex items-center justify-center p-4 bg-black/85 backdrop-blur-sm"
    onClick={onClose}
  >
    <div
      className="relative w-full max-w-3xl rounded-2xl overflow-hidden border border-purple-500/30 shadow-[0_0_80px_rgba(168,85,247,0.3)]"
      onClick={(e) => e.stopPropagation()}
    >
      {/* Modal header */}
      <div
        className="flex items-center justify-between px-5 py-3 border-b border-purple-500/20"
        style={{ background: "rgba(9,7,20,0.98)" }}
      >
        <div className="flex items-center gap-3">
          <span className="inline-block w-2 h-2 rounded-full bg-purple-500 shadow-[0_0_6px_#a855f7]" />
          <span className="text-sm font-semibold text-white">{project.title}</span>
          <span className="text-[0.68rem] font-bold tracking-wider uppercase px-2 py-0.5 rounded-full border border-purple-500/30 bg-purple-500/10 text-purple-400">
            {project.highlight}
          </span>
        </div>
        <button
          onClick={onClose}
          className="w-8 h-8 rounded-lg flex items-center justify-center text-slate-400
            hover:text-white hover:bg-white/10 transition-all duration-200 text-sm font-bold"
        >
          ✕
        </button>
      </div>

      {/* Video player */}
      <div className="aspect-video w-full bg-black">
        {project.videoSrc ? (
          <video
            className="w-full h-full object-contain"
            src={project.videoSrc}
            controls
            autoPlay
            playsInline
          />
        ) : (
          /* No video yet — coming soon */
          <div
            className="w-full h-full flex flex-col items-center justify-center gap-4"
            style={{ background: "linear-gradient(135deg,#0F0B1E,#150D2E)" }}
          >
            <div className="w-16 h-16 rounded-2xl bg-purple-500/15 border border-purple-500/30 flex items-center justify-center">
              <svg viewBox="0 0 24 24" fill="none" className="w-8 h-8 text-purple-400" stroke="currentColor" strokeWidth={1.5}>
                <circle cx="12" cy="12" r="10" />
                <path d="M12 8v4l3 3" strokeLinecap="round" />
              </svg>
            </div>
            <p className="text-white font-semibold text-lg">Demo Coming Soon</p>
            <p className="text-slate-500 text-sm text-center max-w-xs">
              This demo video is being prepared. Check back soon!
            </p>
          </div>
        )}
      </div>
    </div>
  </div>
);

/* ── Preview Pane ── */
const PreviewPane = ({
  project,
  onPlay,
}: {
  project: Project;
  onPlay: () => void;
}) => (
  <div
    className={`relative w-full h-full min-h-190px lg:min-h-0 rounded-xl overflow-hidden
      bg-linear-to-br ${project.previewBg} border border-purple-500/20
      flex items-center justify-center group/preview
      ${project.type !== "live" ? "cursor-pointer" : ""}`}
    onClick={project.type !== "live" ? onPlay : undefined}
  >
    {/* Grid overlay */}
    <div
      className="absolute inset-0 opacity-20"
      style={{
        backgroundImage: `linear-gradient(rgba(168,85,247,0.3) 1px,transparent 1px),
          linear-gradient(90deg,rgba(168,85,247,0.3) 1px,transparent 1px)`,
        backgroundSize: "32px 32px",
      }}
    />

    {/* Number watermark */}
    <span
      className="absolute bottom-2 right-3 font-black text-purple-500/20 select-none pointer-events-none"
      style={{ fontSize: "4.5rem", lineHeight: 1 }}
    >
      {project.number}
    </span>

    {/* Center content */}
    <div className="relative z-10 flex flex-col items-center gap-3">
      <div
        className="w-14 h-14 rounded-2xl bg-white/5 border border-purple-500/30 flex items-center justify-center text-purple-300
          group-hover/preview:bg-purple-500/20 group-hover/preview:border-purple-500/60
          group-hover/preview:shadow-[0_0_24px_rgba(168,85,247,0.4)]
          transition-all duration-300"
      >
        {project.icon}
      </div>

      {project.type !== "live" && (
        <div
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-black/30 border border-white/10
            group-hover/preview:bg-purple-500/20 group-hover/preview:border-purple-500/40
            transition-all duration-300"
        >
          <svg viewBox="0 0 16 16" fill="currentColor" className="w-3 h-3 text-purple-400">
            <path d="M3 2.5l10 5.5-10 5.5V2.5z" />
          </svg>
          <span className="text-[0.72rem] font-semibold text-purple-300">
            {project.type === "video" ? "Watch Video" : "Watch Demo"}
          </span>
        </div>
      )}

      {project.type === "live" && (
        <a
          href={project.link}
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-black/30 border border-white/10
            hover:bg-purple-500/20 hover:border-purple-500/40 transition-all duration-300"
          onClick={(e) => e.stopPropagation()}
        >
          <span className="w-2 h-2 rounded-full bg-green-400 shadow-[0_0_6px_rgba(74,222,128,0.8)]" />
          <span className="text-[0.72rem] font-semibold text-purple-300">Live Site</span>
        </a>
      )}
    </div>
  </div>
);

/* ── Main ── */
const Projects = () => {
  const [modal, setModal] = useState<Project | null>(null);

  return (
    <>
      {modal && <VideoModal project={modal} onClose={() => setModal(null)} />}

      <section
        id="projects"
        className="relative w-full py-18 overflow-hidden"
        style={{ background: "linear-gradient(135deg,#07070F 0%,#0F0B1E 40%,#150D2E 70%,#1A0B33 100%)" }}
      >
        {/* Blobs */}
        <div className="pointer-events-none absolute -top-[20%] -left-[10%] w-[45vw] h-[45vw] rounded-full"
          style={{ background: "radial-gradient(circle,rgba(139,92,246,0.14) 0%,transparent 70%)", filter: "blur(80px)" }} />
        <div className="pointer-events-none absolute -bottom-[20%] -right-[10%] w-[50vw] h-[50vw] rounded-full"
          style={{ background: "radial-gradient(circle,rgba(168,85,247,0.16) 0%,rgba(99,102,241,0.08) 40%,transparent 70%)", filter: "blur(90px)" }} />
        <div className="pointer-events-none absolute inset-0 opacity-[0.03]"
          style={{ backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E")`, backgroundSize: "180px" }} />

        <div className="relative z-10 w-full px-8 sm:px-12 md:px-16 lg:px-20 xl:px-28">

          {/* Heading */}
          <div className="text-center mb-16">
            <div className="inline-flex items-center gap-2 mb-5 px-4 py-1.5 rounded-full border border-purple-500/30 bg-purple-500/8 backdrop-blur-sm text-[0.7rem] font-bold tracking-[0.12em] uppercase text-purple-400">
              <span className="inline-block w-1.5 h-1.5 rounded-full bg-purple-500 shadow-[0_0_6px_#a855f7]" />
              Portfolio
            </div>
            <h2 className="text-4xl md:text-5xl font-black leading-tight mb-4 text-white">
              My{" "}
              <span className="bg-linear-to-r from-purple-400 via-purple-500 to-indigo-400 bg-clip-text text-transparent drop-shadow-[0_0_20px_rgba(168,85,247,0.5)]">
                Work
              </span>
            </h2>
            <p className="text-slate-500 text-base max-w-xl mx-auto leading-relaxed">
              A selection of my work — from web apps and client sites to creative ad videos.
            </p>
            <div className="mt-6 mx-auto w-20 h-0.5 rounded-full bg-linear-to-r from-purple-500 to-indigo-400 shadow-[0_0_8px_rgba(168,85,247,0.6)]" />
          </div>

          {/* Showcase rows */}
          <div className="flex flex-col gap-5">
            {projects.map((project, index) => {
              const isEven = index % 2 === 0;
              return (
                <div
                  key={index}
                  className="group relative flex flex-col lg:flex-row rounded-2xl border border-purple-500/15
                    bg-white/2.5 backdrop-blur-sm overflow-hidden
                    hover:border-purple-500/35 hover:bg-white/4
                    hover:shadow-[0_0_50px_rgba(168,85,247,0.1)]
                    transition-all duration-500"
                >
                  {/* Accent bar */}
                  <div
                    className="absolute inset-x-0 top-0 h-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                    style={{ background: "linear-gradient(90deg,transparent,rgba(168,85,247,0.7),transparent)" }}
                  />

                  {/* Preview pane */}
                  <div className={`w-full lg:w-[36%] shrink-0 p-4 ${isEven ? "lg:order-1" : "lg:order-2"}`}>
                    <PreviewPane project={project} onPlay={() => setModal(project)} />
                  </div>

                  {/* Text */}
                  <div className={`flex flex-col justify-center flex-1 px-6 py-6 lg:px-8 lg:py-8 ${isEven ? "lg:order-2" : "lg:order-1"}`}>

                    {/* Number + badge */}
                    <div className="flex items-center gap-3 mb-4">
                      <span className="text-[0.72rem] font-black text-purple-500/45 tracking-widest">
                        {project.number}
                      </span>
                      <div className="h-px flex-1 bg-purple-500/12" />
                      <span className="text-[0.68rem] font-bold tracking-wider uppercase px-3 py-1 rounded-full border border-purple-500/25 bg-purple-500/10 text-purple-400">
                        {project.highlight}
                      </span>
                    </div>

                    {/* Title */}
                    <h3 className="text-2xl lg:text-3xl font-black text-white mb-3 leading-tight
                      group-hover:text-purple-200 transition-colors duration-300">
                      {project.title}
                    </h3>

                    {/* Description */}
                    <p className="text-slate-500 text-[0.92rem] leading-relaxed mb-5
                      group-hover:text-slate-400 transition-colors duration-300">
                      {project.description}
                    </p>

                    {/* Tech tags */}
                    <div className="flex flex-wrap gap-2 mb-6">
                      {project.tech.map((tech, i) => (
                        <span
                          key={i}
                          className="text-[0.72rem] font-medium px-3 py-1 rounded-lg
                            bg-white/5 border border-white/10 text-slate-400
                            group-hover:border-purple-500/25 group-hover:text-slate-300
                            transition-all duration-300"
                        >
                          {tech}
                        </span>
                      ))}
                    </div>

                    {/* CTA */}
                    {project.type === "live" ? (
                      <a
                        href={project.link}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 w-fit px-5 py-2.5 rounded-xl
                          text-[0.88rem] font-semibold text-white
                          bg-linear-to-br from-purple-600 to-violet-700
                          shadow-[0_0_16px_rgba(147,51,234,0.3)]
                          hover:shadow-[0_0_28px_rgba(168,85,247,0.55)]
                          hover:-translate-y-0.5 transition-all duration-200"
                      >
                        Visit Website
                        <svg viewBox="0 0 16 16" fill="none" className="w-3.5 h-3.5" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round">
                          <path d="M3 8h10M9 4l4 4-4 4" />
                        </svg>
                      </a>
                    ) : (
                      <button
                        onClick={() => setModal(project)}
                        className="inline-flex items-center gap-2 w-fit px-5 py-2.5 rounded-xl
                          text-[0.88rem] font-semibold text-purple-300
                          border border-purple-500/35 bg-purple-500/8
                          hover:bg-purple-500/18 hover:border-purple-500/60
                          hover:text-white hover:shadow-[0_0_20px_rgba(168,85,247,0.25)]
                          hover:-translate-y-0.5 transition-all duration-200"
                      >
                        <svg viewBox="0 0 16 16" fill="currentColor" className="w-3.5 h-3.5 text-purple-400">
                          <path d="M3 2.5l10 5.5-10 5.5V2.5z" />
                        </svg>
                        {project.type === "video" ? "Watch Video" : "Watch Demo"}
                      </button>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

        </div>
      </section>
    </>
  );
};

export default Projects;