import 'package:flutter/material.dart';

class Navbar extends StatelessWidget {
  const Navbar({super.key});

  Widget navItem(BuildContext context, String title, String route) {
    return TextButton(
      onPressed: () => Navigator.pushNamed(context, route),
      child: Text(title, style: const TextStyle(fontSize: 16)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 20),
      color: Colors.black,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          const Text(
            "CobaltDev",
            style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
          ),
          Row(
            children: [
              navItem(context, "Home", "/"),
              navItem(context, "Services", "/services"),
              navItem(context, "Products", "/products"),
              navItem(context, "Portfolio", "/portfolio"),
              navItem(context, "About", "/about"),
              navItem(context, "Contact", "/contact"),
            ],
          ),
        ],
      ),
    );
  }
}
