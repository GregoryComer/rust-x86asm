use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 63, 218], OperandSize::Dword)
}

#[test]
fn vpmaxuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 235490698, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 63, 60, 125, 138, 77, 9, 14], OperandSize::Dword)
}

#[test]
fn vpmaxuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1838205084, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 156, 63, 148, 131, 156, 200, 144, 109], OperandSize::Dword)
}

#[test]
fn vpmaxuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 221, 143, 63, 196], OperandSize::Qword)
}

#[test]
fn vpmaxuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1135608413, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 245, 141, 63, 60, 125, 93, 2, 176, 67], OperandSize::Qword)
}

#[test]
fn vpmaxuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 162241115, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 181, 151, 63, 188, 206, 91, 154, 171, 9], OperandSize::Qword)
}

#[test]
fn vpmaxuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 63, 195], OperandSize::Dword)
}

#[test]
fn vpmaxuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 597848518, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 63, 164, 255, 198, 113, 162, 35], OperandSize::Dword)
}

#[test]
fn vpmaxuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1110093118, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 186, 63, 140, 154, 62, 173, 42, 66], OperandSize::Dword)
}

#[test]
fn vpmaxuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 245, 172, 63, 230], OperandSize::Qword)
}

#[test]
fn vpmaxuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 930093685, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 229, 173, 63, 60, 253, 117, 26, 112, 55], OperandSize::Qword)
}

#[test]
fn vpmaxuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 213, 182, 63, 43], OperandSize::Qword)
}

#[test]
fn vpmaxuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 63, 251], OperandSize::Dword)
}

#[test]
fn vpmaxuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 203, 63, 50], OperandSize::Dword)
}

#[test]
fn vpmaxuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 655192542, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 63, 188, 248, 222, 113, 13, 39], OperandSize::Dword)
}

#[test]
fn vpmaxuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 157, 195, 63, 206], OperandSize::Qword)
}

#[test]
fn vpmaxuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RSI, 1715479685, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 213, 198, 63, 182, 133, 36, 64, 102], OperandSize::Qword)
}

#[test]
fn vpmaxuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1484305088, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 221, 212, 63, 36, 93, 192, 178, 120, 88], OperandSize::Qword)
}

