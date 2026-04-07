import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class AboutPage extends StatelessWidget {
  const AboutPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 900;

    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 80),
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
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(40),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          "My Journey",
                          style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold),
                        ),
                        const SizedBox(height: 20),
                        const Text(
                          "I started with HTML & CSS in 2019. Quickly moved to JavaScript (hated it), then Java, Python, Dart/Flutter, and finally found my true love — Rust.",
                          style: TextStyle(fontSize: 18, height: 1.7, color: Colors.white70),
                        ),
                        const SizedBox(height: 20),
                        const Text(
                          "Now I build full-stack applications: beautiful Flutter frontends paired with high-performance, memory-safe Rust backends. From mobile apps to private cloud infrastructure running on my own hardware.",
                          style: TextStyle(fontSize: 18, height: 1.7, color: Colors.white70),
                        ),
                        const SizedBox(height: 30),

                        Row(
                          children: [
                            const Text("Currently obsessed with: ", style: TextStyle(color: Colors.white70)),
                            const SizedBox(width: 8),
                            const Text(
                              "Rust • Axum • Dioxus • Private Cloud • Unsafe Rust",
                              style: TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.w500),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              ),

              const SizedBox(height: 60),

              // Quick Stats
              AnimatedSection(
                child: Wrap(
                  spacing: 40,
                  runSpacing: 30,
                  children: [
                    _statCard("7", "Years Experience"),
                    _statCard("15+", "Projects Built"),
                    _statCard("3", "Open Source Contributions"),
                    _statCard("∞", "Lines of Rust Code"),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _statCard(String number, String label) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          children: [
            Text(
              number,
              style: const TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Color(0xFF10B981)),
            ),
            const SizedBox(height: 8),
            Text(label, style: const TextStyle(fontSize: 16, color: Colors.white70)),
          ],
        ),
      ),
    );
  }
}