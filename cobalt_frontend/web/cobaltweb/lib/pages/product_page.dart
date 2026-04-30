import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ProductsPage extends StatelessWidget {
  const ProductsPage({super.key});

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
                      "Products & Tools",
                      style: TextStyle(
                        fontSize: isDesktop ? 48 : 36,
                        fontWeight: FontWeight.w600,
                        letterSpacing: -1,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "Some of the tools and products I've built or am actively developing.",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              Wrap(
                spacing: 24,
                runSpacing: 24,
                children: [
                  _productCard(
                    context,
                    "Cobalt Web",
                    "Flutter App / Web",
                    "Implemented BLoC and Freezed for better state management.",
                    "Available - https://cobaltdev.vercel.app",
                  ),
                  InkWell(
                    onTap: () => Navigator.pushReplacementNamed(context, '/cloud'),
                    borderRadius: BorderRadius.circular(12),
                    child: _productCard(
                      context,
                      "Cobalt Cloud",
                      "Rust + Dioxus Cloud Storage",
                      "Self-hosted cloud with Dioxus frontend and Axum Rust backend. Click to learn more!",
                      "Available",
                      onTap: () => Navigator.pushReplacementNamed(context, '/cloud'),
                    ),
                  ),
                  _productCard(
                    context,
                    "Secure Journal",
                    "Encrypted journaling app",
                    "CLI + Dioxus frontend with Axum + SQLx backend. Your thoughts stay yours.",
                    "Available",
                  ),
                  _productCard(
                    context,
                    "Encrypt Notepad",
                    "Rust + Flutter via FFI",
                    "Structured UI and performant backend with memory safety.",
                    "Open Source",
                  ),
                  _productCard(
                    context,
                    "Rust DSA Library",
                    "Algorithms & Data Structures",
                    "Clean, well-documented implementations for learning and production use.",
                    "Open Source",
                  ),
                ],
              ),

              const SizedBox(height: 100),

              Center(
                child: GlassCard(
                  padding: EdgeInsets.all(isMobile ? 32 : 48),
                  child: const Text(
                    "More products coming soon...",
                    style: TextStyle(fontSize: 16, color: Colors.white70),
                    textAlign: TextAlign.center,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _productCard(BuildContext context, String title, String subtitle, String desc, String status, {VoidCallback? onTap}) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 48 : 360,
      child: GlassCard(
        onTap: onTap,
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(title, style: const TextStyle(fontSize: 20, fontWeight: FontWeight.w600)),
            const SizedBox(height: 6),
            Text(subtitle, style: const TextStyle(color: Color(0xFFA855F7), fontWeight: FontWeight.w500, fontSize: 14)),
            const SizedBox(height: 16),
            Text(desc, style: const TextStyle(color: Colors.white70, height: 1.6, fontSize: 15)),
            const SizedBox(height: 24),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: status.contains("Available") 
                    ? const Color(0xFFA855F7).withOpacity(0.1)
                    : const Color(0xFF27272A), // zinc-800
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text(
                status,
                style: TextStyle(
                  color: status.contains("Available") ? const Color(0xFFA855F7) : Colors.white70,
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