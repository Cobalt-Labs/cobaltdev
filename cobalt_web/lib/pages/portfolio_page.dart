import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';

class PortfolioPage extends StatelessWidget {
  const PortfolioPage({super.key});

  final List<Map<String, String>> projects = const [
    {
        "title": "Rust Journal CLI",
        "desc": "CLI app using Rust + SQLx + SQLite"
      },
      {
        "title": "Flutter + Rust FFI App",
        "desc": "High-performance hybrid mobile app"
      },
      {
        "title": "Microservices with Axum",
        "desc": "Rust backend architecture"
      },
      {
        "title": "Algorithms in Rust",
        "desc": "DSA practice repo in Rust"
      },
  ];

  @override
  Widget build(BuildContext context) {
    return AnimatedSection(
      child: GridView.builder(
        padding: const EdgeInsets.all(40),
        gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: 3,
          crossAxisSpacing: 20,
          mainAxisSpacing: 20,
        ),
        itemCount: projects.length,
        itemBuilder: (context, index) {
          final project = projects[index];
          return Container(
            padding: const EdgeInsets.all(20),
            color: Colors.grey[900],
            child: Column(
              children: [
                Text(project["title"]!, style: const TextStyle(fontSize: 18)),
                const SizedBox(height: 10),
                Text(project["desc"]!),
              ],
            ),
          );
        },
      ),
    );
  }
}