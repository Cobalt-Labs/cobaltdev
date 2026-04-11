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
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(
            horizontal: isMobile ? 20 : 40,
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
                        fontSize: isDesktop ? 58 : 42,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 16),
                    const Text(
                      "From beautiful mobile apps to high-performance Rust backends and private cloud infrastructure.",
                      style: TextStyle(fontSize: 20, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              // Services Grid
              AnimatedSection(
                child: Wrap(
                  spacing: 30,
                  runSpacing: 30,
                  children: [
                    _serviceCard(
                      context,
                      "📱",
                      "Mobile & Desktop Apps",
                      "Pixel-perfect Flutter applications with clean architecture and smooth animations. Cross-platform (iOS, Android, Desktop, Web).",
                      "Flutter • Dart • Riverpod",
                    ),
                    _serviceCard(
                      context,
                      "🦀",
                      "Rust Backend Systems",
                      "High-performance, memory-safe backends using Axum, SQLx, and object_store. Built for speed and reliability.",
                      "Rust • Axum • SQLx • Tokio",
                    ),
                    _serviceCard(
                      context,
                      "☁️",
                      "Private Cloud Infrastructure",
                      "Self-hosted cloud solutions running on your own hardware. Drag & drop file storage with full control.",
                      "Rust • object_store • Dioxus",
                    ),
                    _serviceCard(
                      context,
                      "⚡",
                      "Performance & Systems",
                      "Low-level optimizations, FFI bridges, CLI tools, and experimental systems programming in Rust.",
                      "Rust • FFI • DSA • Unsafe",
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 120),

              // CTA Section
              AnimatedSection(
                child: Center(
                  child: SizedBox(
                    width: isMobile ? double.infinity : null,
                    child: GlassCard(
                      child: Padding(
                        padding: EdgeInsets.all(isMobile ? 30 : 50),
                        child: Column(
                          children: [
                            const Text(
                              "Ready to build something great?",
                              style: TextStyle(
                                fontSize: 32,
                                fontWeight: FontWeight.bold,
                              ),
                              textAlign: TextAlign.center,
                            ),
                            const SizedBox(height: 16),
                            const Text(
                              "Let's turn your idea into a production-ready product.",
                              style: TextStyle(
                                fontSize: 18,
                                color: Colors.white70,
                              ),
                              textAlign: TextAlign.center,
                            ),
                            const SizedBox(height: 40),
                            ElevatedButton(
                              onPressed: () => Navigator.pushReplacementNamed(
                                context,
                                '/contact',
                              ),
                              style: ElevatedButton.styleFrom(
                                backgroundColor: const Color(0xFF10B981),
                                foregroundColor: Colors.black,
                                padding: const EdgeInsets.symmetric(
                                  horizontal: 60,
                                  vertical: 20,
                                ),
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(50),
                                ),
                              ),
                              child: const Text(
                                "Start a Project",
                                style: TextStyle(
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                            ),
                          ],
                        ),
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

  Widget _serviceCard(
    BuildContext context,
    String emoji,
    String title,
    String desc,
    String tech,
  ) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 40 : 380,
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(32),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(emoji, style: const TextStyle(fontSize: 48)),
              const SizedBox(height: 24),
              Text(
                title,
                style: const TextStyle(
                  fontSize: 26,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 16),
              Text(
                desc,
                style: const TextStyle(color: Colors.white70, height: 1.7),
              ),
              const SizedBox(height: 24),
              Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 8,
                ),
                decoration: BoxDecoration(
                  color: const Color(0xFF10B981).withOpacity(0.1),
                  borderRadius: BorderRadius.circular(30),
                ),
                child: Text(
                  tech,
                  style: const TextStyle(
                    color: Color(0xFF10B981),
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
