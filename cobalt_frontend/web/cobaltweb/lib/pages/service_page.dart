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
              delay: const Duration(milliseconds: 100),
              child: Wrap(
                spacing: 24,
                runSpacing: 24,
                children: [
                  _serviceCard(
                    context,
                    "Mobile & Desktop Apps",
                    "Pixel-perfect Flutter applications with clean architecture and smooth animations. Cross-platform (iOS, Android, Desktop, Web).",
                    "Flutter • Dart • Riverpod",
                    Icons.phone_android_rounded,
                  ),
                  _serviceCard(
                    context,
                    "Rust Backend Systems",
                    "High-performance, memory-safe backends using Axum, SQLx, and object_store. Built for speed and reliability.",
                    "Rust • Axum • SQLx",
                    Icons.memory_rounded,
                  ),
                  _serviceCard(
                    context,
                    "Private Cloud Infrastructure",
                    "Self-hosted cloud solutions running on your own hardware. Drag & drop file storage with full control.",
                    "Rust • object_store • Dioxus",
                    Icons.cloud_done_rounded,
                  ),
                  _serviceCard(
                    context,
                    "Performance & Systems",
                    "Low-level optimizations, FFI bridges, CLI tools, and experimental systems programming in Rust.",
                    "Rust • FFI • DSA",
                    Icons.speed_rounded,
                  ),
                ],
              ),
            ),

            const SizedBox(height: 100),

            // CTA Section
            AnimatedSection(
              delay: const Duration(milliseconds: 200),
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
                                    color:
                                        const Color(0xFF9333EA).withOpacity(0.35),
                                    blurRadius: 20,
                                    offset: const Offset(0, 4),
                                  ),
                                ],
                              ),
                              child: const Text(
                                "Start a Project",
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

  Widget _serviceCard(
    BuildContext context,
    String title,
    String desc,
    String tech,
    IconData icon,
  ) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? (sw - 48).clamp(0.0, double.infinity).toDouble() : 360.0,
      child: GlassCard(
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: const Color(0xFFA855F7).withOpacity(0.1),
                borderRadius: BorderRadius.circular(10),
                border: Border.all(
                  color: const Color(0xFFA855F7).withOpacity(0.2),
                ),
              ),
              child: Icon(icon, color: const Color(0xFFA855F7), size: 22),
            ),
            const SizedBox(height: 20),
            Text(
              title,
              style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
            ),
            const SizedBox(height: 12),
            Text(
              desc,
              style: const TextStyle(
                color: Colors.white70,
                height: 1.6,
                fontSize: 15,
              ),
            ),
            const SizedBox(height: 24),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: const Color(0xFF1E1A36),
                borderRadius: BorderRadius.circular(6),
                border: Border.all(color: Colors.white10),
              ),
              child: Text(
                tech,
                style: const TextStyle(
                  color: Color(0xFF94A3B8),
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
