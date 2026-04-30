import 'dart:ui';
import 'package:flutter/material.dart';

class GlassCard extends StatefulWidget {
  final Widget child;
  final double? height;
  final double? width;
  final EdgeInsets? padding;
  final VoidCallback? onTap;

  const GlassCard({
    super.key,
    required this.child,
    this.height,
    this.width,
    this.padding,
    this.onTap,
  });

  @override
  State<GlassCard> createState() => _GlassCardState();
}

class _GlassCardState extends State<GlassCard> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: widget.onTap != null
          ? SystemMouseCursors.click
          : MouseCursor.defer,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              curve: Curves.easeOut,
              height: widget.height,
              width: widget.width,
              padding: widget.padding ?? const EdgeInsets.all(24),
              transform: Matrix4.identity()..translate(0.0, _hovered ? -4.0 : 0.0),
              decoration: BoxDecoration(
                color: _hovered && widget.onTap != null
                    ? Colors.white.withOpacity(0.04)
                    : Colors.white.withOpacity(0.025),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: _hovered && widget.onTap != null
                      ? const Color(0xFFA855F7).withOpacity(0.35)
                      : const Color(0xFFA855F7).withOpacity(0.15),
                  width: 1,
                ),
                boxShadow: _hovered && widget.onTap != null
                    ? [
                        BoxShadow(
                          color: const Color(0xFFA855F7).withOpacity(0.1),
                          blurRadius: 50,
                          spreadRadius: 2,
                        )
                      ]
                    : [],
              ),
              child: widget.child,
            ),
          ),
        ),
      ),
    );
  }
}