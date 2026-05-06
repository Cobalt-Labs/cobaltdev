import 'package:flutter/material.dart';

/// A premium card widget with hover effects.
/// [BackdropFilter] removed from here to eliminate GPU overdraw/scroll lag.
/// Uses a subtle border + background color approach instead.
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
      cursor: widget.onTap != null ? SystemMouseCursors.click : MouseCursor.defer,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 220),
          curve: Curves.easeOut,
          height: widget.height,
          width: widget.width,
          padding: widget.padding ?? const EdgeInsets.all(24),
          transform: Matrix4.identity()
            ..translate(0.0, _hovered && widget.onTap != null ? -4.0 : 0.0),
          decoration: BoxDecoration(
            // Simulated glass: dark translucent bg + border — no BackdropFilter needed
            color: _hovered && widget.onTap != null
                ? const Color(0xFF16122A)
                : const Color(0xFF100E22),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(
              color: _hovered && widget.onTap != null
                  ? const Color(0xFFA855F7).withOpacity(0.45)
                  : const Color(0xFFA855F7).withOpacity(0.18),
              width: 1,
            ),
            boxShadow: _hovered && widget.onTap != null
                ? [
                    BoxShadow(
                      color: const Color(0xFFA855F7).withOpacity(0.12),
                      blurRadius: 40,
                      spreadRadius: 1,
                    ),
                  ]
                : [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.3),
                      blurRadius: 16,
                      offset: const Offset(0, 4),
                    ),
                  ],
          ),
          child: widget.child,
        ),
      ),
    );
  }
}