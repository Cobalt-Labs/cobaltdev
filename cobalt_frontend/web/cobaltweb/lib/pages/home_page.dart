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

    return Scaffold(
      body: SingleChildScrollView(
        child: Column(
          children: [
            // 🔥 HERO SECTION
            SizedBox(
              height: MediaQuery.of(context).size.height,
              child: Stack(
                fit: StackFit.expand,
                children: [
                  // Background gradient
                  Container(
                    decoration: const BoxDecoration(
                      gradient: LinearGradient(
                        begin: Alignment.topLeft,
                        end: Alignment.bottomRight,
                        colors: [
                          Color(0xFF0A0A0A),
                          Color(0xFF111111),
                        ],
                      ),
                    ),
                  ),

                  Center(
                    child: ConstrainedBox(
                      constraints: const BoxConstraints(maxWidth: 1200),
                      child: Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 40),
                        child: AnimatedSection(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                "Ibrahim Haji",
                                style: TextStyle(
                                  fontSize: isDesktop ? 68 : isTablet ? 52 : 42,
                                  fontWeight: FontWeight.bold,
                                  height: 1.1,
                                ),
                              ),
                              const SizedBox(height: 12),
                              Text(
                                "Flutter + Rust Developer",
                                style: TextStyle(
                                  fontSize: isDesktop ? 32 : 24,
                                  color: const Color(0xFF10B981),
                                  fontWeight: FontWeight.w500,
                                ),
                              ),
                              const SizedBox(height: 24),

                              const Text(
                                "Building production-grade mobile apps, high-performance backends, and private cloud infrastructure with Flutter & Rust.",
                                style: TextStyle(
                                  fontSize: 20,
                                  color: Colors.white70,
                                  height: 1.6,
                                ),
                              ),

                              const SizedBox(height: 50),

                              Row(
                                children: [
                                  ElevatedButton(
                                    onPressed: () => Navigator.pushReplacementNamed(context, '/portfolio'),
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor: const Color(0xFF10B981),
                                      foregroundColor: Colors.black,
                                      padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 18),
                                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(50)),
                                    ),
                                    child: const Text("View My Work", style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                                  ),
                                  const SizedBox(width: 20),
                                  OutlinedButton(
                                    onPressed: () => Navigator.pushReplacementNamed(context, '/contact'),
                                    style: OutlinedButton.styleFrom(
                                      side: const BorderSide(color: Colors.white54),
                                      padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 18),
                                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(50)),
                                    ),
                                    child: const Text("Let's Talk", style: TextStyle(fontSize: 18)),
                                  ),
                                ],
                              ),

                              const SizedBox(height: 80),

                              // Trust signals
                              Wrap(
                                spacing: 40,
                                runSpacing: 20,
                                children: const [
                                  Text("7+ Years Experience", style: TextStyle(color: Colors.white70)),
                                  Text("Flutter • Rust • Axum", style: TextStyle(color: Colors.white70)),
                                  Text("Private Cloud Infra", style: TextStyle(color: Colors.white70)),
                                ],
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),

            // 🔥 FEATURED WORK
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 100),
              child: AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      "Featured Projects",
                      style: TextStyle(fontSize: 42, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "Real stuff I've built with passion",
                      style: TextStyle(fontSize: 20, color: Colors.white70),
                    ),
                    const SizedBox(height: 60),

                    Wrap(
                      spacing: 30,
                      runSpacing: 30,
                      children: [
                        _projectCard(
                          "Secure Journal",
                          "CLI + Dioxus + Axum + SQLx",
                          "A private journaling app with end-to-end encryption and Rust backend",
                          "https://github.com/ibrahim-3595/Secure-Journal",
                        ),
                        _projectCard(
                          "Cobalt Cloud",
                          "Rust Backend + Dioxus Frontend",
                          "Self-hosted private cloud running on my laptop HDD",
                          "#", // we'll link later
                        ),
                        _projectCard(
                          "Flutter + Rust FFI Apps",
                          "Hybrid Mobile + Desktop",
                          "Production apps using Flutter frontend + Rust core via FFI",
                          "https://github.com/ibrahim-3595",
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),

            // 🔥 WHY ME SECTION
            Container(
              color: const Color(0xFF111111),
              padding: const EdgeInsets.symmetric(vertical: 100, horizontal: 40),
              child: AnimatedSection(
                child: Column(
                  children: [
                    const Text(
                      "Why Work With Me?",
                      style: TextStyle(fontSize: 42, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 60),
                    Wrap(
                      spacing: 40,
                      runSpacing: 40,
                      alignment: WrapAlignment.center,
                      children: [
                        _whyCard("⚡", "Blazing Fast", "Rust performance + Flutter smoothness"),
                        _whyCard("🔒", "Production Grade", "Clean architecture & error handling"),
                        _whyCard("🛠️", "Full-Stack", "From UI to backend to infra"),
                        _whyCard("🌍", "Cross Platform", "Mobile, Desktop, Web, Cloud"),
                      ],
                    ),
                  ],
                ),
              ),
            ),

            // FOOTER
            Container(
              padding: const EdgeInsets.all(60),
              color: Colors.black,
              child: const Center(
                child: Text(
                  "© 2026 CobaltDev • Built with Flutter & Rust",
                  style: TextStyle(color: Colors.white54),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _projectCard(String title, String subtitle, String desc, String url) {
    return SizedBox(
      width: 380,
      child: GlassCard(
        onTap: () => _openLink(url),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              height: 180,
              decoration: BoxDecoration(
                color: const Color(0xFF1F1F1F),
                borderRadius: BorderRadius.circular(16),
              ),
              child: const Center(
                child: Icon(Icons.code, size: 60, color: Color(0xFF10B981)),
              ),
            ),
            const SizedBox(height: 20),
            Text(title, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
            const SizedBox(height: 6),
            Text(subtitle, style: const TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.w500)),
            const SizedBox(height: 12),
            Text(desc, style: const TextStyle(color: Colors.white70, height: 1.5)),
            const SizedBox(height: 20),
            const Text("View Project →", style: TextStyle(color: Color(0xFF10B981))),
          ],
        ),
      ),
    );
  }

  Widget _whyCard(String emoji, String title, String desc) {
    return SizedBox(
      width: 280,
      child: GlassCard(
        child: Column(
          children: [
            Text(emoji, style: const TextStyle(fontSize: 48)),
            const SizedBox(height: 16),
            Text(title, style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
            const SizedBox(height: 10),
            Text(desc, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70)),
          ],
        ),
      ),
    );
  }
}