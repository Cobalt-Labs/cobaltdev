import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';

class AboutPage extends StatelessWidget {
  const AboutPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const AnimatedSection(
      child: Center(
        child: Text(
          "Services Page",
          style: TextStyle(fontSize: 30),
        ),
      ),
    );
  }
}