using System;
namespace DrDotnet
{
    public class AnalysisData
    {
        public Guid Id { get; set; }

        public AnalysisStatus Status { get; set; }

        public AnalysisType Type { get; set; }

        public DateTime Date { get; set; }

        public string ProcessName { get; set; }

        public ulong SizeBytes { get; set; }
    }
}