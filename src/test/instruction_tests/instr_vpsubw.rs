use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 249, 227], OperandSize::Dword)
}

#[test]
fn vpsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1182214466, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 249, 20, 205, 66, 41, 119, 70], OperandSize::Dword)
}

#[test]
fn vpsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 249, 198], OperandSize::Qword)
}

#[test]
fn vpsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 84081297, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 249, 164, 79, 145, 250, 2, 5], OperandSize::Qword)
}

#[test]
fn vpsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 249, 224], OperandSize::Dword)
}

#[test]
fn vpsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1161239728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 249, 52, 189, 176, 28, 55, 69], OperandSize::Dword)
}

#[test]
fn vpsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 249, 216], OperandSize::Qword)
}

#[test]
fn vpsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 168734079, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 172, 154, 127, 173, 14, 10], OperandSize::Qword)
}

#[test]
fn vpsubw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 249, 250], OperandSize::Dword)
}

#[test]
fn vpsubw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 249, 3], OperandSize::Dword)
}

#[test]
fn vpsubw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 101, 134, 249, 221], OperandSize::Qword)
}

#[test]
fn vpsubw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2034886941, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 29, 133, 249, 188, 80, 29, 233, 73, 121], OperandSize::Qword)
}

#[test]
fn vpsubw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 249, 243], OperandSize::Dword)
}

#[test]
fn vpsubw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 171, 249, 35], OperandSize::Dword)
}

#[test]
fn vpsubw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 13, 174, 249, 193], OperandSize::Qword)
}

#[test]
fn vpsubw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 107579141, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 175, 249, 20, 213, 5, 135, 105, 6], OperandSize::Qword)
}

#[test]
fn vpsubw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 201, 249, 196], OperandSize::Dword)
}

#[test]
fn vpsubw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 249, 42], OperandSize::Dword)
}

#[test]
fn vpsubw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 13, 198, 249, 241], OperandSize::Qword)
}

#[test]
fn vpsubw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1922771062, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 204, 249, 140, 155, 118, 40, 155, 114], OperandSize::Qword)
}

