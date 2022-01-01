using System.Collections.Generic;

namespace DrDotnet
{
    public interface IAnalysesDiscovery
    {
        List<AnalysisData> GetAnalyses();
    }
}