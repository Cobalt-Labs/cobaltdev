import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class AboutPage extends StatelessWidget {
  const AboutPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 900;
    final isMobile = width < 700;

    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(horizontal: isMobile ? 20 : 40, vertical: isMobile ? 40 : 80),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "About Me",
                      style: TextStyle(
                        fontSize: isDesktop ? 58 : 42,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 24),
                    const Text(
                      "7 years of grinding. From hating JavaScript to falling in love with Rust.",
                      style: TextStyle(fontSize: 22, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              AnimatedSection(
                child: Center(
                  child: ConstrainedBox(
                    constraints: const BoxConstraints(maxWidth: 800),
                    child: GlassCard(
                      child: Padding(
                        padding: EdgeInsets.all(isMobile ? 30 : 40),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const Text(
                              "My Journey",
                              style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Color(0xFF10B981)),
                            ),
                            const SizedBox(height: 20),
                            const Text(
                              "I've been programming for over 7 years. What started as simple mobile applications turned into a deep passion for system-level programming and highly optimized backends.\n\nToday, I build seamless, native-feeling experiences using Flutter and power them with unyielding Rust backends.",
                              style: TextStyle(fontSize: 18, color: Colors.white70, height: 1.8),
                            ),
                            const SizedBox(height: 24),
                            const Text(
                              "Currently, I'm focused on Cobalt Cloud—a self-hosted platform running on raw Rust—and building cross-platform Dioxus frontend apps.",
                              style: TextStyle(fontSize: 18, color: Colors.white70, height: 1.8),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),

              const SizedBox(height: 60),

              // Quick Stats
              AnimatedSection(
                child: Wrap(
                  spacing: 30,
                  runSpacing: 30,
                  alignment: WrapAlignment.center,
                  children: [
                    _statCard(context, "7+", "Years Coding"),
                    _statCard(context, "10+", "Production Apps"),
                    _statCard(context, "100%", "Rust-pilled"),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _statCard(BuildContext context, String number, String label) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 400 ? sw - 40 : 200,
      child: GlassCard(
        child: Padding(
          padding: EdgeInsets.all(sw < 400 ? 16 : 24),
          child: Column(
            children: [
              Text(
                number,
                style: const TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Color(0xFF10B981)),
              ),
              const SizedBox(height: 8),
              Text(label, textAlign: TextAlign.center, style: const TextStyle(fontSize: 16, color: Colors.white70)),
            ],
          ),
        ),
      ),
    );
  }
}