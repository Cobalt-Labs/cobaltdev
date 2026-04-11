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
                      "Products & Tools",
                      style: TextStyle(
                        fontSize: isDesktop ? 58 : 42,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 16),
                    const Text(
                      "Some of the tools and products I've built or am actively developing.",
                      style: TextStyle(fontSize: 20, color: Colors.white70),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 80),

              Wrap(
                spacing: 30,
                runSpacing: 40,
                children: [
                  _productCard(
                    context,
                    "Cobalt Web",
                    "Flutter app/web hosted describing my products and services",
                    "Implemented BLoC and Fressed for better state managemen.t",
                    "⚡",
                    "Available - https://cobaltdev.vercel.app",
                  ),
                  InkWell(
                    onTap: () => Navigator.pushReplacementNamed(context, '/cloud'),
                    borderRadius: BorderRadius.circular(20),
                    child: _productCard(
                      context,
                      "Cobalt Cloud",
                      "Rust + Dioxus Cloud Storage",
                      "Self-hosted cloud with Dioxus frontend and Axum Rust backend. Click to learn more!",
                      "☁️",
                      "Available",
                      onTap: () => Navigator.pushReplacementNamed(context, '/cloud'),
                    ),
                  ),
                  _productCard(
                    context,
                    "Secure Journal",
                    "Private encrypted journaling app",
                    "CLI + Dioxus frontend with Axum + SQLx backend. Your thoughts stay yours.",
                    "📖",
                    "Available",
                  ),
                  _productCard(
                    context,
                    "Encrypt Notepad",
                    "Rust + Flutter via FFI/flutter_rust_bridge",
                    "Structured UI and performant backend with memory safety.",
                    "⚡",
                    "Open Source",
                  ),
                  //
                  _productCard(
                    context,
                    "Rust DSA Library",
                    "Algorithms & Data Structures in Rust",
                    "Clean, well-documented implementations for learning and production use.",
                    "⚡",
                    "Open Source",
                  ),
                ],
              ),

              const SizedBox(height: 120),

              Center(
                child: GlassCard(
                  child: Padding(
                    padding: EdgeInsets.all(isMobile ? 30 : 60),
                    child: Text(
                      "More products coming soon...",
                      style: TextStyle(fontSize: 22, color: Colors.white70),
                      textAlign: TextAlign.center,
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

  Widget _productCard(BuildContext context, String title, String subtitle, String desc, String emoji, String status, {VoidCallback? onTap}) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 40 : 380,
      child: GlassCard(
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.all(32),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(emoji, style: const TextStyle(fontSize: 52)),
              const SizedBox(height: 24),
              Text(title, style: const TextStyle(fontSize: 26, fontWeight: FontWeight.bold)),
              const SizedBox(height: 8),
              Text(subtitle, style: const TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.w500)),
              const SizedBox(height: 16),
              Text(desc, style: const TextStyle(color: Colors.white70, height: 1.6)),
              const SizedBox(height: 24),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
                decoration: BoxDecoration(
                  color: status == "Available" 
                      ? const Color(0xFF10B981).withOpacity(0.15)
                      : Colors.white.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(30),
                ),
                child: Text(
                  status,
                  style: TextStyle(
                    color: status == "Available" ? const Color(0xFF10B981) : Colors.white70,
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