import 'package:flutter/material.dart';
import 'pages/home_page.dart';
import 'pages/service_page.dart';
import 'pages/product_page.dart';
import 'pages/portfolio_page.dart';
import 'pages/about_page.dart';
import 'pages/contact_page.dart';
import 'widgets/navbar.dart';

void main() {
  runApp(const CobaltDevApp());
}

class CobaltDevApp extends StatelessWidget {
  const CobaltDevApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'CobaltDev',
      theme: ThemeData.dark(),
      initialRoute: '/',
      routes: {
        '/': (context) => const MainLayout(child: HomePage()),
        '/about': (context) => const MainLayout(child: AboutPage()),
        '/services': (context) => const MainLayout(child: ServicesPage()),
        '/products': (context) => const MainLayout(child: ProductsPage()),
        '/portfolio': (context) => const MainLayout(child: PortfolioPage()),
        '/contact': (context) => const MainLayout(child: ContactPage()),
      },
    );
  }
}

class MainLayout extends StatelessWidget {
  final Widget child;

  const MainLayout({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          const Navbar(),
          Expanded(child: child),
        ],
      ),
    );
  }
}