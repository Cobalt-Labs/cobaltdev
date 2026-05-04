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
      backgroundColor: Colors.transparent,
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48, vertical: isMobile ? 40 : 80),
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
                        fontSize: isDesktop ? 48 : 36,
                        fontWeight: FontWeight.w600,
                        letterSpacing: -1,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "From simple interfaces to highly optimized Rust backends.",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
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
                      padding: EdgeInsets.all(isMobile ? 32 : 48),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text(
                            "My Journey",
                            style: TextStyle(fontSize: 24, fontWeight: FontWeight.w600, letterSpacing: -0.5),
                          ),
                          const SizedBox(height: 24),
                          const Text(
                            "I've been programming for over 7 years. What started as simple mobile applications turned into a deep passion for system-level programming and highly optimized backends.\n\nToday, I build seamless, native-feeling experiences using Flutter and power them with unyielding Rust backends.",
                            style: TextStyle(fontSize: 16, color: Colors.white70, height: 1.7),
                          ),
                          const SizedBox(height: 24),
                          const Text(
                            "Currently, I'm focused on Cobalt Cloud—a self-hosted platform running on raw Rust—and building cross-platform Dioxus frontend apps.",
                            style: TextStyle(fontSize: 16, color: Colors.white70, height: 1.7),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),

              const SizedBox(height: 80),

              // Quick Stats
              AnimatedSection(
                child: Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  alignment: WrapAlignment.center,
                  children: [
                    _statCard(context, "7+", "Years Coding"),
                    _statCard(context, "10+", "Production Apps"),
                    _statCard(context, "100%", "Rust Backend"),
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
      width: sw < 400 ? (sw - 48).clamp(0.0, double.infinity).toDouble() : 200.0,
      child: GlassCard(
        padding: EdgeInsets.all(sw < 400 ? 24 : 32),
        child: Column(
          children: [
            Text(
              number,
              style: const TextStyle(fontSize: 40, fontWeight: FontWeight.w600, color: Color(0xFFA855F7), letterSpacing: -1), // Purple
            ),
            const SizedBox(height: 8),
            Text(label, textAlign: TextAlign.center, style: const TextStyle(fontSize: 15, color: Colors.white70, fontWeight: FontWeight.w500)),
          ],
        ),
      ),
    );
  }
}