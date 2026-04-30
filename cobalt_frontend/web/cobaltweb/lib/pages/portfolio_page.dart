import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class PortfolioPage extends StatelessWidget {
  const PortfolioPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1100;
    final isMobile = width < 700;

    return Scaffold(
      backgroundColor: Colors.transparent,
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(
            horizontal: isMobile ? 24 : 48,
            vertical: isMobile ? 40 : 80,
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "My Portfolio",
                      style: TextStyle(
                        fontSize: isDesktop ? 48 : 36,
                        fontWeight: FontWeight.w600,
                        letterSpacing: -1,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "A collection of apps, backends, and systems I've built over the last 7 years.",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              // Apps Section
              _sectionTitle("Mobile & Desktop Apps"),
              const SizedBox(height: 24),
              _projectGrid(context, _appProjects()),

              const SizedBox(height: 80),

              // Backend Section
              _sectionTitle("Rust Backend Systems"),
              const SizedBox(height: 24),
              _projectGrid(context, _backendProjects()),

              const SizedBox(height: 80),

              // Systems & Experiments
              _sectionTitle("Systems & Experiments"),
              const SizedBox(height: 24),
              _projectGrid(context, _systemProjects()),

              const SizedBox(height: 100),

              // CTA
              Center(
                child: SizedBox(
                  width: isMobile ? double.infinity : null,
                  child: GlassCard(
                    padding: EdgeInsets.all(isMobile ? 32 : 48),
                    child: Column(
                      children: [
                        const Text(
                          "Want to see more or discuss a project?",
                          style: TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.w600,
                            letterSpacing: -0.5,
                          ),
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 24),
                        ElevatedButton(
                          onPressed: () => Navigator.pushReplacementNamed(context, '/contact'),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF4F46E5), // Indigo 600
                            foregroundColor: Colors.white,
                            padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 18),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                            elevation: 0,
                          ),
                          child: const Text(
                            "Let's Build Something Together",
                            style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _sectionTitle(String title) {
    return Text(
      title,
      style: const TextStyle(fontSize: 24, fontWeight: FontWeight.w600, letterSpacing: -0.5),
    );
  }

  Widget _projectGrid(BuildContext context, List<Map<String, dynamic>> projects) {
    return Wrap(
      spacing: 24,
      runSpacing: 24,
      children: projects.map((project) => _projectCard(context, project)).toList(),
    );
  }

  Widget _projectCard(BuildContext context, Map<String, dynamic> project) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 48 : 360,
      child: GlassCard(
        onTap: project['url'] != null ? () => _openLink(project['url']) : null,
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              project['title'],
              style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
            ),
            const SizedBox(height: 6),
            Text(
              project['tech'],
              style: const TextStyle(color: Color(0xFFA855F7), fontWeight: FontWeight.w500, fontSize: 14),
            ),
            const SizedBox(height: 16),
            Text(
              project['desc'],
              style: const TextStyle(color: Colors.white70, height: 1.6, fontSize: 15),
              maxLines: 4,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 24),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                if (project['url'] != null)
                  TextButton.icon(
                    onPressed: () => _openLink(project['url']),
                    icon: const Icon(Icons.arrow_forward, size: 16),
                    label: const Text("View Project"),
                    style: TextButton.styleFrom(
                      foregroundColor: const Color(0xFFA855F7),
                      textStyle: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
                      padding: EdgeInsets.zero,
                      minimumSize: Size.zero,
                    ),
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _openLink(String url) async {
    final uri = Uri.parse(url);
    if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
      throw 'Could not launch $url';
    }
  }

  // PROJECT DATA
  List<Map<String, dynamic>> _appProjects() {
    return [
      {
        "title": "Flutter + Rust Hybrid Apps",
        "tech": "Flutter • Rust • FFI",
        "desc": "Multiple production apps using Flutter for beautiful UI and Rust for high-performance core logic via FFI.",
        "url": "https://github.com/ibrahim-3595",
      },
      {
        "title": "Secure Journal App",
        "tech": "Flutter • Rust • SQLx",
        "desc": "Private journaling app with end-to-end encryption, Rust backend, and clean cross-platform UI.",
        "url": "https://github.com/ibrahim-3595/Secure-Journal",
      },
    ];
  }

  List<Map<String, dynamic>> _backendProjects() {
    return [
      {
        "title": "Cobalt Cloud",
        "tech": "Rust • Axum • Dioxus",
        "desc": "Self-hosted private cloud infrastructure with Dioxus frontend and Rust backend in cobalt_backend.",
        "url": "https://github.com/ibrahim-3595/cobaltdev/tree/main/cobalt_cloud",
      },
      {
        "title": "Axum Microservices",
        "tech": "Rust • Axum • SQLx",
        "desc": "Scalable backend APIs and microservices built with Axum framework and SQLx for database operations.",
        "url": "https://github.com/ibrahim-3595",
      },
    ];
  }

  List<Map<String, dynamic>> _systemProjects() {
    return [
      {
        "title": "Algorithms in Rust",
        "tech": "Rust • DSA",
        "desc": "Collection of data structures and algorithms implemented in Rust for learning and performance testing.",
        "url": "https://github.com/ibrahim-3595",
      },
      {
        "title": "Rust CLI Tools",
        "tech": "Rust • CLI • SQLx",
        "desc": "Command-line tools and utilities built with pure Rust for maximum performance and reliability.",
        "url": "https://github.com/ibrahim-3595",
      },
    ];
  }
}
