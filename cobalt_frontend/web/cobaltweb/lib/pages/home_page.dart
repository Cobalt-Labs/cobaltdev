import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Future<void> _openLink(String url) async {
    final uri = Uri.parse(url);
    if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
      throw 'Could not launch $url';
    }
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1100;
    final isTablet = width > 700 && width <= 1100;
    final isMobile = width <= 700;

    return Scaffold(
      backgroundColor: const Color(0xFF18181B), // zinc-900
      body: SingleChildScrollView(
        child: Column(
          children: [
            // HERO SECTION
            SizedBox(
              height: MediaQuery.of(context).size.height * 0.9,
              child: Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 1200),
                  child: Padding(
                    padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48),
                    child: AnimatedSection(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            "Ibrahim Haji",
                            style: TextStyle(
                              fontSize: isDesktop ? 64 : isTablet ? 48 : 40,
                              fontWeight: FontWeight.w600,
                              letterSpacing: -1,
                              height: 1.1,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 12),
                          Text(
                            "Flutter + Rust Developer",
                            style: TextStyle(
                              fontSize: isDesktop ? 28 : 22,
                              color: const Color(0xFF6366F1), // Indigo
                              fontWeight: FontWeight.w500,
                              letterSpacing: -0.5,
                            ),
                          ),
                          const SizedBox(height: 24),

                          const Text(
                            "Building production-grade mobile apps, high-performance backends, and private cloud infrastructure with Flutter & Rust.",
                            style: TextStyle(
                              fontSize: 18,
                              color: Colors.white70,
                              height: 1.6,
                            ),
                          ),

                          const SizedBox(height: 48),

                          Wrap(
                            spacing: 16,
                            runSpacing: 16,
                            children: [
                              ElevatedButton(
                                onPressed: () => Navigator.pushReplacementNamed(context, '/portfolio'),
                                style: ElevatedButton.styleFrom(
                                  backgroundColor: const Color(0xFF4F46E5), // Indigo 600
                                  foregroundColor: Colors.white,
                                  padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
                                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                                  elevation: 0, // Flat
                                ),
                                child: const Text("View My Work", style: TextStyle(fontSize: 16, fontWeight: FontWeight.w500)),
                              ),
                              OutlinedButton(
                                onPressed: () => Navigator.pushReplacementNamed(context, '/contact'),
                                style: OutlinedButton.styleFrom(
                                  foregroundColor: Colors.white,
                                  side: const BorderSide(color: Color(0xFF3F3F46)), // zinc-700
                                  padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
                                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                                ),
                                child: const Text("Let's Talk", style: TextStyle(fontSize: 16, fontWeight: FontWeight.w500)),
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),

            // FEATURED WORK
            Container(
              padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48, vertical: isMobile ? 60 : 100),
              color: const Color(0xFF18181B), // uniform background
              child: AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      "Featured Projects",
                      style: TextStyle(fontSize: 32, fontWeight: FontWeight.w600, letterSpacing: -0.5),
                    ),
                    const SizedBox(height: 8),
                    const Text(
                      "Real stuff I've built with passion",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                    const SizedBox(height: 48),

                    Wrap(
                      spacing: 24,
                      runSpacing: 24,
                      children: [
                        _projectCard(context,
                          "Secure Journal",
                          "CLI + Dioxus + Axum + SQLx",
                          "A private journaling app with end-to-end encryption and Rust backend.",
                          "https://github.com/Cobalt-Labs/cobalt_journal",
                        ),
                        _projectCard(context,
                          "Cobalt Cloud",
                          "Rust Backend + Dioxus Frontend",
                          "Self-hosted private cloud running on my laptop HDD.",
                          "https://github.com/Cobalt-Labs/cobaltdev/tree/main/cobalt_cloud",
                        ),
                        _projectCard(context,
                          "Encrypt Notepad",
                          "Hybrid Mobile + Desktop",
                          "Production apps using Flutter frontend + Rust core via FFI.",
                          "https://github.com/ibrahim-3595/Encrypt-Notepad",
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),

            // FOOTER
            Container(
              padding: const EdgeInsets.all(60),
              color: const Color(0xFF09090B), // zinc-950
              child: const Center(
                child: Text(
                  "© 2026 CobaltDev",
                  style: TextStyle(color: Colors.white54, fontSize: 14),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _projectCard(BuildContext context, String title, String subtitle, String desc, String url) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 48 : 360,
      child: GlassCard(
        onTap: () => _openLink(url),
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(title, style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600)),
            const SizedBox(height: 4),
            Text(subtitle, style: const TextStyle(fontSize: 14, color: Color(0xFF6366F1), fontWeight: FontWeight.w500)),
            const SizedBox(height: 16),
            Text(desc, style: const TextStyle(color: Colors.white70, height: 1.5, fontSize: 15)),
            const SizedBox(height: 24),
            const Text("View Project →", style: TextStyle(color: Color(0xFF6366F1), fontSize: 14, fontWeight: FontWeight.w500)),
          ],
        ),
      ),
    );
  }
}