﻿namespace DrDotnet.Desktop
{
    public partial class App : Application
    {
        public App() {
            InitializeComponent();

            MainPage = new MainPage();
        }
    }
}