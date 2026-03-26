import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ServicesPage extends StatelessWidget {
  const ServicesPage({super.key});

  int _getGridCount(double width) {
    if (width > 1000) return 4;
    if (width > 600) return 2;
    return 1;
  }

  double _getResponsiveFont(
    BuildContext context,
    double desktop,
    double mobile,
  ) {
    final width = MediaQuery.of(context).size.width;
    return width > 1000 ? desktop : mobile;
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final gridCount = _getGridCount(width);
    final isDesktop = width > 1000;

    return Scaffold(
      body: SingleChildScrollView(
        child: Align(
          alignment: Alignment.topCenter,
          child: Container(
            width: isDesktop ? 1200 : double.infinity,
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 40),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                /// 🔥 HERO
                AnimatedSection(
                  child: Column(
                    children: [
                      Text(
                        "What I Can Build For You",
                        textAlign: TextAlign.center,
                        style: TextStyle(
                          fontSize: _getResponsiveFont(context, 42, 28),
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 20),
                      const Text(
                        "From mobile apps to scalable Rust backends — I build systems that actually perform.",
                        textAlign: TextAlign.center,
                      ),
                    ],
                  ),
                ),

                const SizedBox(height: 40),

                /// 🚀 SERVICES GRID
                AnimatedSection(
                  child: GridView.builder(
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    itemCount: 4,
                    gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                      crossAxisCount: gridCount,
                      crossAxisSpacing: 20,
                      mainAxisSpacing: 20,
                      // childAspectRatio: width < 600 ? 1.3 : 1.2,
                      childAspectRatio: width < 600 ? 0.9 : 1.1,
                    ),
                    itemBuilder: (context, index) {
                      final services = [
                        _service(
                          "📱",
                          "Mobile Apps",
                          "Flutter apps with clean UI & performance",
                        ),
                        _service(
                          "🌐",
                          "Web Apps",
                          "Responsive Flutter web apps",
                        ),
                        _service(
                          "🦀",
                          "Rust Backend",
                          "Axum APIs, SQLx, scalable systems",
                        ),
                        _service(
                          "⚡",
                          "Performance Systems",
                          "High-speed backend logic in Rust",
                        ),
                      ];
                      return services[index];
                    },
                  ),
                ),

                const SizedBox(height: 60),

                /// 🧠 PROCESS
                AnimatedSection(
                  child: Column(
                    children: [
                      const Text("How I Work", style: TextStyle(fontSize: 32)),
                      const SizedBox(height: 20),
                      GridView(
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: gridCount > 3 ? 3 : gridCount,
                          crossAxisSpacing: 20,
                          mainAxisSpacing: 20,
                          childAspectRatio: 1.2,
                        ),
                        children: [
                          _step("1", "Plan", "Understand your idea"),
                          _step("2", "Build", "Develop fast & clean"),
                          _step("3", "Scale", "Make it production ready"),
                        ],
                      ),
                    ],
                  ),
                ),

                const SizedBox(height: 60),

                /// 🛠️ TECH STACK
                AnimatedSection(
                  child: Column(
                    children: const [
                      Text("Tech Stack", style: TextStyle(fontSize: 32)),
                      SizedBox(height: 20),
                      Text(
                        "Flutter • Rust • Axum • SQLx • Docker • Firebase",
                        textAlign: TextAlign.center,
                      ),
                    ],
                  ),
                ),

                const SizedBox(height: 60),

                /// 💼 CTA
                AnimatedSection(
                  child: Column(
                    children: [
                      const Text(
                        "Let’s Build Something Great",
                        style: TextStyle(fontSize: 28),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 20),
                      ElevatedButton(
                        onPressed: () {
                          Navigator.pushReplacementNamed(context, "/contact");
                        },
                        child: const Text("Start a Project"),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _service(String icon, String title, String desc) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(icon, style: const TextStyle(fontSize: 36)),
            const SizedBox(height: 8),

            Flexible(
              child: Text(
                title,
                style: const TextStyle(fontSize: 16),
                textAlign: TextAlign.center,
                overflow: TextOverflow.ellipsis,
              ),
            ),

            const SizedBox(height: 6),

            Flexible(
              child: Text(
                desc,
                textAlign: TextAlign.center,
                overflow: TextOverflow.ellipsis,
                maxLines: 3,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _step(String num, String title, String desc) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(num, style: const TextStyle(fontSize: 24)),
            const SizedBox(height: 8),

            Flexible(
              child: Text(
                title,
                textAlign: TextAlign.center,
                overflow: TextOverflow.ellipsis,
              ),
            ),

            const SizedBox(height: 6),

            Flexible(
              child: Text(
                desc,
                textAlign: TextAlign.center,
                overflow: TextOverflow.ellipsis,
                maxLines: 2,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
