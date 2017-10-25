use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vreducesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 154, 87, 243, 120], OperandSize::Dword)
}

fn vreducesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 142, 87, 36, 255, 37], OperandSize::Dword)
}

fn vreducesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM29)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 131, 197, 154, 87, 237, 56], OperandSize::Qword)
}

fn vreducesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1192516051, Some(OperandSize::Qword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 221, 129, 87, 172, 126, 211, 89, 20, 71, 89], OperandSize::Qword)
}

