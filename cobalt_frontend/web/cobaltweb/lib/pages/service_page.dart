import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ServicesPage extends StatelessWidget {
  const ServicesPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1000;
    final isMobile = width < 700;

    return Scaffold(
      backgroundColor: const Color(0xFF18181B), // zinc-900
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(
            horizontal: isMobile ? 24 : 48,
            vertical: isMobile ? 40 : 80,
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "What I Build",
                      style: TextStyle(
                        fontSize: isDesktop ? 48 : 36,
                        fontWeight: FontWeight.w600,
                        letterSpacing: -1,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "From beautiful mobile apps to high-performance Rust backends.",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              // Services Grid
              AnimatedSection(
                child: Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  children: [
                    _serviceCard(
                      context,
                      "Mobile & Desktop Apps",
                      "Pixel-perfect Flutter applications with clean architecture and smooth animations. Cross-platform (iOS, Android, Desktop, Web).",
                      "Flutter • Dart • Riverpod",
                    ),
                    _serviceCard(
                      context,
                      "Rust Backend Systems",
                      "High-performance, memory-safe backends using Axum, SQLx, and object_store. Built for speed and reliability.",
                      "Rust • Axum • SQLx",
                    ),
                    _serviceCard(
                      context,
                      "Private Cloud Infrastructure",
                      "Self-hosted cloud solutions running on your own hardware. Drag & drop file storage with full control.",
                      "Rust • object_store • Dioxus",
                    ),
                    _serviceCard(
                      context,
                      "Performance & Systems",
                      "Low-level optimizations, FFI bridges, CLI tools, and experimental systems programming in Rust.",
                      "Rust • FFI • DSA",
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 100),

              // CTA Section
              AnimatedSection(
                child: Center(
                  child: SizedBox(
                    width: isMobile ? double.infinity : null,
                    child: GlassCard(
                      padding: EdgeInsets.all(isMobile ? 32 : 48),
                      child: Column(
                        children: [
                          const Text(
                            "Ready to build something great?",
                            style: TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.w600,
                              letterSpacing: -0.5,
                            ),
                            textAlign: TextAlign.center,
                          ),
                          const SizedBox(height: 16),
                          const Text(
                            "Let's turn your idea into a production-ready product.",
                            style: TextStyle(
                              fontSize: 16,
                              color: Colors.white70,
                            ),
                            textAlign: TextAlign.center,
                          ),
                          const SizedBox(height: 32),
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
                              "Start a Project",
                              style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500),
                            ),
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
      ),
    );
  }

  Widget _serviceCard(BuildContext context, String title, String desc, String tech) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 48 : 360,
      child: GlassCard(
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
            ),
            const SizedBox(height: 12),
            Text(
              desc,
              style: const TextStyle(color: Colors.white70, height: 1.6, fontSize: 15),
            ),
            const SizedBox(height: 24),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: const Color(0xFF27272A), // zinc-800
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text(
                tech,
                style: const TextStyle(
                  color: Colors.white70,
                  fontSize: 13,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
