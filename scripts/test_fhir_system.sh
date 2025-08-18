#!/bin/bash

echo "🧪 Testing FHIR to PDF System"
echo "=============================="

# Change to the rust_ssi directory
cd rust_ssi

echo "📦 Building the project..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    
    echo "🚀 Running the FHIR processor..."
    cargo run
    
    if [ $? -eq 0 ]; then
        echo "✅ FHIR processing completed successfully!"
        
        # Check if output file was created
        if [ -f "patient_visit_summary.txt" ]; then
            echo "📄 Generated document found:"
            echo "----------------------------------------"
            head -20 patient_visit_summary.txt
            echo "----------------------------------------"
            echo "📁 Full document saved to: patient_visit_summary.txt"
        else
            echo "❌ Output file not found"
        fi
    else
        echo "❌ FHIR processing failed"
        exit 1
    fi
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "🎉 Test completed successfully!"
echo "📚 Next steps:"
echo "   - Add MongoDB integration"
echo "   - Implement proper PDF generation"
echo "   - Add web API endpoints"
echo "   - Integrate with Hedera blockchain"
