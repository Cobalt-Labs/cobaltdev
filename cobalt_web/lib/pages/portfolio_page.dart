import 'package:flutter/material.dart';
import '../widgets/glass_card.dart';

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
    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(40),
        child: Wrap(
          spacing: 20,
          runSpacing: 20,
          children: projects.map((project) {
            return SizedBox(
              width: 300,
              child: GlassCard(
                child: Column(
                  children: [
                    Text(project["title"]!, style: TextStyle(fontSize: 18)),
                    SizedBox(height: 10),
                    Text(project["desc"]!),
                  ],
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}