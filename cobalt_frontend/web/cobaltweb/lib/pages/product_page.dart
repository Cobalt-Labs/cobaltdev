import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ProductsPage extends StatelessWidget {
  const ProductsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 900;
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

            AnimatedSection(
              delay: const Duration(milliseconds: 100),
              child: Wrap(
                spacing: 24,
                runSpacing: 24,
                children: [
                  _productCard(
                    context,
                    "Cobalt Web",
                    "Flutter App / Web",
                    "Implemented BLoC and Freezed for better state management. Deployed and live.",
                    "Live",
                    onTap: () => _launch("https://cobaltdev.vercel.app"),
                  ),
                  _productCard(
                    context,
                    "Cobalt Cloud",
                    "Rust + Dioxus Cloud Storage",
                    "Self-hosted cloud with Dioxus frontend and Axum Rust backend.",
                    "Available",
                    // Navigate to portfolio — no /cloud route in the router
                    onTap: () =>
                        Navigator.pushReplacementNamed(context, '/portfolio'),
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
                    onTap: () =>
                        _launch("https://github.com/ibrahim-3595/Encrypt-Notepad"),
                  ),
                  _productCard(
                    context,
                    "Rust DSA Library",
                    "Algorithms & Data Structures",
                    "Clean, well-documented implementations for learning and production use.",
                    "Open Source",
                    onTap: () => _launch("https://github.com/ibrahim-3595"),
                  ),
                ],
              ),
            ),

            const SizedBox(height: 100),

            AnimatedSection(
              delay: const Duration(milliseconds: 200),
              child: Center(
                child: GlassCard(
                  padding: EdgeInsets.all(isMobile ? 32 : 48),
                  child: Column(
                    children: const [
                      Text(
                        "More products coming soon...",
                        style: TextStyle(fontSize: 16, color: Colors.white70),
                        textAlign: TextAlign.center,
                      ),
                      SizedBox(height: 8),
                      Text(
                        "Follow the GitHub for updates.",
                        style: TextStyle(
                          fontSize: 14,
                          color: Color(0xFF64748B),
                        ),
                        textAlign: TextAlign.center,
                      ),
                    ],
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

  Future<void> _launch(String url) async {
    final uri = Uri.parse(url);
    try {
      await launchUrl(uri, mode: LaunchMode.externalApplication);
    } catch (e) {
      debugPrint('Could not launch $url: $e');
    }
  }

  Widget _productCard(
    BuildContext context,
    String title,
    String subtitle,
    String desc,
    String status, {
    VoidCallback? onTap,
  }) {
    final sw = MediaQuery.of(context).size.width;

    // Status badge styling
    final Color statusColor;
    final Color statusBg;
    switch (status) {
      case "Live":
        statusColor = const Color(0xFF4ADE80);
        statusBg = const Color(0xFF4ADE80).withValues(alpha: 0.1);
        break;
      case "Available":
        statusColor = const Color(0xFFA855F7);
        statusBg = const Color(0xFFA855F7).withValues(alpha: 0.1);
        break;
      default: // Open Source
        statusColor = const Color(0xFF94A3B8);
        statusBg = Colors.white.withValues(alpha: 0.04);
    }

    return SizedBox(
      width: sw < 450
          ? (sw - 48).clamp(0.0, double.infinity).toDouble()
          : 360.0,
      child: GlassCard(
        onTap: onTap,
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.w600,
              ),
            ),
            const SizedBox(height: 6),
            Text(
              subtitle,
              style: const TextStyle(
                color: Color(0xFFA855F7),
                fontWeight: FontWeight.w500,
                fontSize: 14,
              ),
            ),
            const SizedBox(height: 16),
            Text(
              desc,
              style: const TextStyle(
                color: Colors.white70,
                height: 1.6,
                fontSize: 15,
              ),
            ),
            const SizedBox(height: 24),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 6,
                  ),
                  decoration: BoxDecoration(
                    color: statusBg,
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(
                      color: statusColor.withValues(alpha: 0.3),
                    ),
                  ),
                  child: Text(
                    status,
                    style: TextStyle(
                      color: statusColor,
                      fontSize: 13,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
                if (onTap != null)
                  Icon(
                    Icons.arrow_forward_rounded,
                    size: 18,
                    color: const Color(0xFFA855F7).withValues(alpha: 0.7),
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}