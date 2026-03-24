import 'package:flutter/material.dart';
import '../widgets/glass_card.dart';
import '../widgets/animated_section.dart';
import 'package:url_launcher/url_launcher.dart';

Future<void> _openGithub() async {
  final url = Uri.parse("https://github.com/ibrahim-3595");

  if (await canLaunchUrl(url)) {
    await launchUrl(
      url,
      mode: LaunchMode.platformDefault, // ✅ better for web
    );
  } else {
    throw "Could not launch $url";
  }
}

class PortfolioPage extends StatelessWidget {
  const PortfolioPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1000;

    return Scaffold(
      body: SingleChildScrollView(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1200),
            child: Padding(
              padding: const EdgeInsets.all(40),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  /// 🔥 HEADER
                  AnimatedSection(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          "My Work",
                          style: TextStyle(
                            fontSize: isDesktop ? 42 : 28,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 10),
                        const Text(
                          "A collection of apps, systems and backend projects I’ve built.",
                        ),
                      ],
                    ),
                  ),

                  const SizedBox(height: 40),

                  /// 📱 APPS
                  _sectionTitle("📱 Applications"),
                  _projectGrid(_appProjects()),

                  const SizedBox(height: 40),

                  /// 🦀 BACKEND
                  _sectionTitle("🦀 Backend Systems"),
                  _projectGrid(_backendProjects()),

                  const SizedBox(height: 40),

                  /// ⚡ SYSTEMS
                  _sectionTitle("⚡ Systems & Experiments"),
                  _projectGrid(_systemProjects()),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  /// 🔥 SECTION TITLE
  Widget _sectionTitle(String title) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 20),
      child: Text(
        title,
        style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold),
      ),
    );
  }

  /// 🔥 GRID
  Widget _projectGrid(List<Map<String, String>> projects) {
    return Wrap(
      spacing: 20,
      runSpacing: 20,
      children: projects.map((p) => _projectCard(p)).toList(),
    );
  }

  /// 🔥 PROJECT CARD
  Widget _projectCard(Map<String, String> project) {
    return SizedBox(
      width: 300,
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          transform: Matrix4.identity()..scale(1.02),
          child: GlassCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                /// 🖼️ IMAGE
                Container(
                  height: 160,
                  width: double.infinity,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(10),
                    image: const DecorationImage(
                      image: NetworkImage("https://picsum.photos/400"),
                      fit: BoxFit.cover,
                    ),
                  ),
                ),

                const SizedBox(height: 12),

                /// 📌 TITLE
                Text(
                  project["title"]!,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),

                const SizedBox(height: 6),

                /// 📝 DESC
                Text(project["desc"]!),

                const SizedBox(height: 10),

                /// 🛠️ TECH
                Text(
                  project["tech"]!,
                  style: const TextStyle(fontSize: 12, color: Colors.grey),
                ),

                const SizedBox(height: 10),

                /// 🔗 ACTION
                TextButton.icon(
                  onPressed: _openGithub,
                  icon: const Icon(Icons.open_in_new),
                  label: const Text("View on GitHub"),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  /// 📱 APP PROJECTS
  List<Map<String, String>> _appProjects() {
    return [
      {
        "title": "Flutter + Rust App",
        "desc": "Hybrid app using Flutter UI and Rust backend",
        "tech": "Flutter • Rust • FFI",
      },
      {
        "title": "Portfolio Website",
        "desc": "Animated Flutter web portfolio",
        "tech": "Flutter Web • UI/UX",
      },
    ];
  }

  /// 🦀 BACKEND PROJECTS
  List<Map<String, String>> _backendProjects() {
    return [
      {
        "title": "Auth System",
        "desc": "JWT auth with Axum + SQLx",
        "tech": "Rust • Axum • SQLite",
      },
      {
        "title": "Microservices API",
        "desc": "Scalable backend architecture",
        "tech": "Rust • Docker • APIs",
      },
    ];
  }

  /// ⚡ SYSTEM PROJECTS
  List<Map<String, String>> _systemProjects() {
    return [
      {
        "title": "Rust CLI Journal",
        "desc": "Command-line journaling app",
        "tech": "Rust • CLI • SQLite",
      },
      {
        "title": "Algorithms in Rust",
        "desc": "DSA implementations and experiments",
        "tech": "Rust • Data Structures",
      },
    ];
  }
}
