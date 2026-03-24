import 'package:flutter/material.dart';
import '../widgets/animated_section.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          AnimatedSection(
            child: Container(
              height: 500,
              alignment: Alignment.center,
              child: const Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    "Welcome to CobaltDev",
                    style: TextStyle(fontSize: 40, fontWeight: FontWeight.bold),
                  ),
                  SizedBox(height: 20),
                  Text(
                    "We build modern apps, websites & scalable solutions",
                    style: TextStyle(fontSize: 18),
                  ),
                ],
              ),
            ),
          ),

          AnimatedSection(
            child: Container(
              padding: const EdgeInsets.all(40),
              child: const Column(
                children: [
                  Text("Our Services", style: TextStyle(fontSize: 28)),
                  SizedBox(height: 20),
                  Text("Web Development • Mobile Apps • UI/UX • Backend Systems"),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}