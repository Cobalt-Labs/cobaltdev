import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class PortfolioPage extends StatelessWidget {
  const PortfolioPage({super.key});

  Future<void> _openGithub() async {
    final url = Uri.parse("https://github.com/ibrahim-3595");
    if (await canLaunchUrl(url)) {
      await launchUrl(url, mode: LaunchMode.externalApplication);
    }
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1100;

    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 80),
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
                        fontSize: isDesktop ? 62 : 48,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 16),
                    const Text(
                      "A collection of apps, backends, and systems I've built over the last 7 years.",
                      style: TextStyle(fontSize: 22, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              // Apps Section
              _sectionTitle("📱 Mobile & Desktop Apps"),
              const SizedBox(height: 30),
              _projectGrid(_appProjects()),

              const SizedBox(height: 100),

              // Backend Section
              _sectionTitle("🦀 Rust Backend Systems"),
              const SizedBox(height: 30),
              _projectGrid(_backendProjects()),

              const SizedBox(height: 100),

              // Systems & Experiments
              _sectionTitle("⚡ Systems & Experiments"),
              const SizedBox(height: 30),
              _projectGrid(_systemProjects()),

              const SizedBox(height: 120),

              // CTA
              Center(
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(40),
                    child: Column(
                      children: [
                        const Text(
                          "Want to see more or discuss a project?",
                          style: TextStyle(fontSize: 24, fontWeight: FontWeight.w500),
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 20),
                        ElevatedButton(
                          onPressed: () => Navigator.pushReplacementNamed(context, '/contact'),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF10B981),
                            foregroundColor: Colors.black,
                            padding: const EdgeInsets.symmetric(horizontal: 50, vertical: 18),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(50)),
                          ),
                          child: const Text("Let's Build Something Together", style: TextStyle(fontSize: 18)),
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
      style: const TextStyle(
        fontSize: 34,
        fontWeight: FontWeight.bold,
      ),
    );
  }

  Widget _projectGrid(List<Map<String, dynamic>> projects) {
    return Wrap(
      spacing: 30,
      runSpacing: 40,
      children: projects.map((project) => _projectCard(project)).toList(),
    );
  }

  Widget _projectCard(Map<String, dynamic> project) {
    return SizedBox(
      width: 380,
      child: GlassCard(
        onTap: project['url'] != null ? () => _openLink(project['url']) : null,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Project Image / Icon
            Container(
              height: 200,
              width: double.infinity,
              decoration: BoxDecoration(
                color: const Color(0xFF1F1F1F),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Center(
                child: Text(
                  project['icon'] ?? "🛠️",
                  style: const TextStyle(fontSize: 80),
                ),
              ),
            ),

            const SizedBox(height: 24),

            // Title
            Text(
              project['title'],
              style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),

            const SizedBox(height: 8),

            // Subtitle / Tech
            Text(
              project['tech'],
              style: const TextStyle(
                color: Color(0xFF10B981),
                fontWeight: FontWeight.w500,
              ),
            ),

            const SizedBox(height: 16),

            // Description
            Text(
              project['desc'],
              style: const TextStyle(color: Colors.white70, height: 1.6),
              maxLines: 4,
              overflow: TextOverflow.ellipsis,
            ),

            const SizedBox(height: 24),

            // Action
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton.icon(
                  onPressed: project['url'] != null ? () => _openLink(project['url']) : null,
                  icon: const Icon(Icons.open_in_new, size: 18),
                  label: const Text("View Project"),
                  style: TextButton.styleFrom(
                    foregroundColor: const Color(0xFF10B981),
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

  // ─────────────────────────────────────────────────────────────
  // PROJECT DATA
  List<Map<String, dynamic>> _appProjects() {
    return [
      {
        "title": "Flutter + Rust Hybrid Apps",
        "tech": "Flutter • Rust • FFI",
        "desc": "Multiple production apps using Flutter for beautiful UI and Rust for high-performance core logic via FFI.",
        "icon": "📱",
        "url": "https://github.com/ibrahim-3595",
      },
      {
        "title": "Secure Journal App",
        "tech": "Flutter • Rust • SQLx",
        "desc": "Private journaling app with end-to-end encryption, Rust backend, and clean cross-platform UI.",
        "icon": "📖",
        "url": "https://github.com/ibrahim-3595/Secure-Journal",
      },
    ];
  }

  List<Map<String, dynamic>> _backendProjects() {
    return [
      {
        "title": "Cobalt Cloud",
        "tech": "Rust • Axum • object_store",
        "desc": "Self-hosted private cloud infrastructure running on my laptop HDD. Drag & drop from Dioxus frontend.",
        "icon": "☁️",
        "url": "#",
      },
      {
        "title": "Axum Microservices",
        "tech": "Rust • Axum • SQLx",
        "desc": "Scalable backend APIs and microservices built with Axum framework and SQLx for database operations.",
        "icon": "🦀",
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
        "icon": "⚡",
        "url": "https://github.com/ibrahim-3595",
      },
      {
        "title": "Rust CLI Tools",
        "tech": "Rust • CLI • SQLx",
        "desc": "Command-line tools and utilities built with pure Rust for maximum performance and reliability.",
        "icon": "🖥️",
        "url": "https://github.com/ibrahim-3595",
      },
    ];
  }
}