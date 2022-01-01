using System;
using System.Collections.Generic;
namespace DrDotnet
{
    public class AnalysesDiscovery : IAnalysesDiscovery
    {
        private ILogger _logger;

        public AnalysesDiscovery(ILogger logger)
        {
            _logger = logger;
        }

        public List<AnalysisData> GetAnalyses()
        {
            var analysis = new List<AnalysisData>();

            // Todo: Read analyses files

            analysis.Add(new AnalysisData { ProcessName = "MyProcess.exe", Date = DateTime.Now, SizeBytes = 24757458, Status = AnalysisStatus.InProgress, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-4), SizeBytes = 123453, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-8), SizeBytes = 456563, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-75), SizeBytes = 43453, Status = AnalysisStatus.Failed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-521), SizeBytes = 2347502, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-521), SizeBytes = 2347502, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-521), SizeBytes = 2347502, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-521), SizeBytes = 2347502, Status = AnalysisStatus.Completed, Type = new AnalysisType() });
            analysis.Add(new AnalysisData { ProcessName = "AnotherProcess.exe", Date = DateTime.Now.AddHours(-521), SizeBytes = 2347502, Status = AnalysisStatus.Completed, Type = new AnalysisType() });

            return analysis;
        }
    }
}