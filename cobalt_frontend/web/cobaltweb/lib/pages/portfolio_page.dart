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

    // No inner Scaffold — MainLayout already provides one
    return SingleChildScrollView(
      physics: const ClampingScrollPhysics(),
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

            AnimatedSection(
              delay: const Duration(milliseconds: 80),
              child: _sectionTitle("Mobile & Desktop Apps"),
            ),
            const SizedBox(height: 24),
            AnimatedSection(
              delay: const Duration(milliseconds: 120),
              child: _projectGrid(context, _appProjects()),
            ),

            const SizedBox(height: 80),

            AnimatedSection(
              delay: const Duration(milliseconds: 160),
              child: _sectionTitle("Rust Backend Systems"),
            ),
            const SizedBox(height: 24),
            AnimatedSection(
              delay: const Duration(milliseconds: 200),
              child: _projectGrid(context, _backendProjects()),
            ),

            const SizedBox(height: 80),

            AnimatedSection(
              delay: const Duration(milliseconds: 240),
              child: _sectionTitle("Systems & Experiments"),
            ),
            const SizedBox(height: 24),
            AnimatedSection(
              delay: const Duration(milliseconds: 280),
              child: _projectGrid(context, _systemProjects()),
            ),

            const SizedBox(height: 100),

            // CTA
            AnimatedSection(
              delay: const Duration(milliseconds: 300),
              child: Center(
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
                        MouseRegion(
                          cursor: SystemMouseCursors.click,
                          child: GestureDetector(
                            onTap: () => Navigator.pushReplacementNamed(
                              context,
                              '/contact',
                            ),
                            child: Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 40,
                                vertical: 18,
                              ),
                              decoration: BoxDecoration(
                                gradient: const LinearGradient(
                                  colors: [Color(0xFF9333EA), Color(0xFF6D28D9)],
                                ),
                                borderRadius: BorderRadius.circular(12),
                                boxShadow: [
                                  BoxShadow(
                                    color: const Color(0xFF9333EA)
                                        .withValues(alpha: 0.35),
                                    blurRadius: 20,
                                    offset: const Offset(0, 4),
                                  ),
                                ],
                              ),
                              child: const Text(
                                "Let's Build Something Together",
                                style: TextStyle(
                                  fontSize: 15,
                                  fontWeight: FontWeight.w600,
                                  color: Colors.white,
                                ),
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),

            const SizedBox(height: 60),
          ],
        ),
      ),
    );
  }

  Widget _sectionTitle(String title) {
    return Row(
      children: [
        Container(
          width: 3,
          height: 20,
          decoration: BoxDecoration(
            color: const Color(0xFFA855F7),
            borderRadius: BorderRadius.circular(2),
          ),
        ),
        const SizedBox(width: 12),
        Text(
          title,
          style: const TextStyle(
            fontSize: 22,
            fontWeight: FontWeight.w700,
            letterSpacing: -0.5,
            color: Colors.white,
          ),
        ),
      ],
    );
  }

  Widget _projectGrid(
    BuildContext context,
    List<Map<String, dynamic>> projects,
  ) {
    return Wrap(
      spacing: 24,
      runSpacing: 24,
      children: projects.map((p) => _projectCard(context, p)).toList(),
    );
  }

  Widget _projectCard(BuildContext context, Map<String, dynamic> project) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? (sw - 48).clamp(0.0, double.infinity).toDouble() : 360.0,
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
              style: const TextStyle(
                color: Color(0xFFA855F7),
                fontWeight: FontWeight.w500,
                fontSize: 14,
              ),
            ),
            const SizedBox(height: 16),
            Text(
              project['desc'],
              style: const TextStyle(
                color: Colors.white70,
                height: 1.6,
                fontSize: 15,
              ),
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
                    icon: const Icon(Icons.arrow_forward_rounded, size: 16),
                    label: const Text("View Project"),
                    style: TextButton.styleFrom(
                      foregroundColor: const Color(0xFFA855F7),
                      textStyle: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w500,
                      ),
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
      debugPrint('Could not launch $url');
    }
  }

  List<Map<String, dynamic>> _appProjects() => [
        {
          "title": "Flutter + Rust Hybrid Apps",
          "tech": "Flutter • Rust • FFI",
          "desc":
              "Multiple production apps using Flutter for beautiful UI and Rust for high-performance core logic via FFI.",
          "url": "https://github.com/ibrahim-3595",
        },
        {
          "title": "Secure Journal App",
          "tech": "Flutter • Rust • SQLx",
          "desc":
              "Private journaling app with end-to-end encryption, Rust backend, and clean cross-platform UI.",
          "url": "https://github.com/ibrahim-3595/Secure-Journal",
        },
      ];

  List<Map<String, dynamic>> _backendProjects() => [
        {
          "title": "Cobalt Cloud",
          "tech": "Rust • Axum • Dioxus",
          "desc":
              "Self-hosted private cloud infrastructure with Dioxus frontend and Rust backend in cobalt_backend.",
          "url":
              "https://github.com/Cobalt-Labs/cobaltdev/tree/main/cobalt_cloud",
        },
        {
          "title": "Axum Microservices",
          "tech": "Rust • Axum • SQLx",
          "desc":
              "Scalable backend APIs and microservices built with Axum framework and SQLx for database operations.",
          "url": "https://github.com/ibrahim-3595",
        },
      ];

  List<Map<String, dynamic>> _systemProjects() => [
        {
          "title": "Algorithms in Rust",
          "tech": "Rust • DSA",
          "desc":
              "Collection of data structures and algorithms implemented in Rust for learning and performance testing.",
          "url": "https://github.com/ibrahim-3595",
        },
        {
          "title": "Rust CLI Tools",
          "tech": "Rust • CLI • SQLx",
          "desc":
              "Command-line tools and utilities built with pure Rust for maximum performance and reliability.",
          "url": "https://github.com/ibrahim-3595",
        },
      ];
}
