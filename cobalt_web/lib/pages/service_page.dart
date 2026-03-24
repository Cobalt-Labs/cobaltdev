import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ServicesPage extends StatelessWidget {
  const ServicesPage({super.key});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1000;

    return Scaffold(
      body: SingleChildScrollView(
        child: Column(
          children: [

            /// 🔥 HERO
            AnimatedSection(
              child: Padding(
                padding: const EdgeInsets.all(40),
                child: Column(
                  children: [
                    Text(
                      "What I Can Build For You",
                      style: TextStyle(
                        fontSize: isDesktop ? 42 : 28,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    SizedBox(height: 20),
                    Text(
                      "From mobile apps to scalable Rust backends — I build systems that actually perform.",
                      textAlign: TextAlign.center,
                    ),
                  ],
                ),
              ),
            ),

            /// 🚀 SERVICES GRID
            AnimatedSection(
              child: Padding(
                padding: const EdgeInsets.all(40),
                child: Wrap(
                  spacing: 20,
                  runSpacing: 20,
                  children: [
                    _service("📱", "Mobile Apps",
                        "Flutter apps with clean UI & performance"),
                    _service("🌐", "Web Apps",
                        "Responsive Flutter web apps"),
                    _service("🦀", "Rust Backend",
                        "Axum APIs, SQLx, scalable systems"),
                    _service("⚡", "Performance Systems",
                        "High-speed backend logic in Rust"),
                  ],
                ),
              ),
            ),

            /// 🧠 PROCESS
            AnimatedSection(
              child: Padding(
                padding: const EdgeInsets.all(40),
                child: Column(
                  children: [
                    Text("How I Work",
                        style: TextStyle(fontSize: 32)),
                    SizedBox(height: 20),
                    Wrap(
                      spacing: 20,
                      runSpacing: 20,
                      children: [
                        _step("1", "Plan", "Understand your idea"),
                        _step("2", "Build", "Develop fast & clean"),
                        _step("3", "Scale", "Make it production ready"),
                      ],
                    )
                  ],
                ),
              ),
            ),

            /// 🛠️ TECH STACK
            AnimatedSection(
              child: Padding(
                padding: const EdgeInsets.all(40),
                child: Column(
                  children: [
                    Text("Tech Stack",
                        style: TextStyle(fontSize: 32)),
                    SizedBox(height: 20),
                    Text(
                      "Flutter • Rust • Axum • SQLx • Docker • Firebase",
                      textAlign: TextAlign.center,
                    ),
                  ],
                ),
              ),
            ),

            /// 💼 CTA
            AnimatedSection(
              child: Padding(
                padding: const EdgeInsets.all(40),
                child: Column(
                  children: [
                    Text("Let’s Build Something Great",
                        style: TextStyle(fontSize: 28)),
                    SizedBox(height: 20),
                    ElevatedButton(
                      onPressed: () {
                        Navigator.pushNamed(context, "/contact");
                      },
                      child: Text("Start a Project"),
                    )
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  /// 🔧 SERVICE CARD
  Widget _service(String icon, String title, String desc) {
    return SizedBox(
      width: 250,
      child: GlassCard(
        child: Column(
          children: [
            Text(icon, style: TextStyle(fontSize: 40)),
            SizedBox(height: 10),
            Text(title, style: TextStyle(fontSize: 18)),
            SizedBox(height: 10),
            Text(desc, textAlign: TextAlign.center),
          ],
        ),
      ),
    );
  }

  /// 🔧 PROCESS STEP
  Widget _step(String num, String title, String desc) {
    return SizedBox(
      width: 200,
      child: GlassCard(
        child: Column(
          children: [
            Text(num, style: TextStyle(fontSize: 28)),
            SizedBox(height: 10),
            Text(title),
            SizedBox(height: 6),
            Text(desc, textAlign: TextAlign.center),
          ],
        ),
      ),
    );
  }
}