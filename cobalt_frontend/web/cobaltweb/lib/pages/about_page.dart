import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';

class AboutPage extends StatelessWidget {
  const AboutPage({super.key});

  @override
  Widget build(BuildContext context) {
    return AnimatedSection(
      child: Padding(
        padding: const EdgeInsets.all(40),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: const [
            Text("About Me", style: TextStyle(fontSize: 36)),
            SizedBox(height: 20),
            Text(
              "I started my journey 7 years ago with HTML and CSS, explored JavaScript, Java, Python, and eventually found my core stack in Flutter and Rust.",
            ),
            SizedBox(height: 10),
            Text(
              "I build full-stack applications, from frontend UI to backend systems, and even low-level systems using Rust.",
            ),
            SizedBox(height: 10),
            Text(
              "Currently working with Flutter, Rust (Axum, SQLx), and exploring Dioxus, AI systems, and distributed architectures.",
            ),
          ],
        ),
      ),
    );
  }
}