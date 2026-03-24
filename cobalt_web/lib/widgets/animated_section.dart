import 'package:flutter/material.dart';

class AnimatedSection extends StatefulWidget {
  final Widget child;

  const AnimatedSection({super.key, required this.child});

  @override
  State<AnimatedSection> createState() => _AnimatedSectionState();
}

class _AnimatedSectionState extends State<AnimatedSection>
    with SingleTickerProviderStateMixin {
  late AnimationController controller;
  late Animation<double> opacity;

  @override
  void initState() {
    super.initState();
    controller =
        AnimationController(vsync: this, duration: const Duration(milliseconds: 800));
    opacity = Tween(begin: 0.0, end: 1.0).animate(controller);
    controller.forward();
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: opacity,
      child: widget.child,
    );
  }
}