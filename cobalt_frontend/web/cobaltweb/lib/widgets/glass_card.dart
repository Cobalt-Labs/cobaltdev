import 'package:flutter/material.dart';

/// A clean, flat card — no blur, no glass, no glow.
/// Think Vercel / Linear dashboard card.
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
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 180),
          height: widget.height,
          width: widget.width,
          padding: widget.padding ?? const EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: _hovered && widget.onTap != null
                ? const Color(0xFF252530)
                : const Color(0xFF1C1C24),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: _hovered && widget.onTap != null
                  ? const Color(0xFF6366F1).withOpacity(0.4)
                  : const Color(0xFF2E2E3A),
              width: 1,
            ),
          ),
          child: widget.child,
        ),
      ),
    );
  }
}