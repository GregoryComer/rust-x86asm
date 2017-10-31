use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 233, 198], OperandSize::Dword)
}

#[test]
fn vpsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 590106862, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 233, 150, 238, 80, 44, 35], OperandSize::Dword)
}

#[test]
fn vpsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 228], OperandSize::Qword)
}

#[test]
fn vpsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 1247128567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 233, 131, 247, 171, 85, 74], OperandSize::Qword)
}

#[test]
fn vpsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 233, 197], OperandSize::Dword)
}

#[test]
fn vpsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDI, 137437221, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 233, 175, 37, 32, 49, 8], OperandSize::Dword)
}

#[test]
fn vpsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 233, 215], OperandSize::Qword)
}

#[test]
fn vpsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 339953055, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 233, 164, 218, 159, 69, 67, 20], OperandSize::Qword)
}

#[test]
fn vpsubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 233, 247], OperandSize::Dword)
}

#[test]
fn vpsubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 935526504, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 233, 52, 85, 104, 0, 195, 55], OperandSize::Dword)
}

#[test]
fn vpsubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 93, 134, 233, 228], OperandSize::Qword)
}

#[test]
fn vpsubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1082175934, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 53, 140, 233, 140, 211, 190, 177, 128, 64], OperandSize::Qword)
}

#[test]
fn vpsubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 233, 205], OperandSize::Dword)
}

#[test]
fn vpsubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 967839254, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 233, 172, 178, 22, 14, 176, 57], OperandSize::Dword)
}

#[test]
fn vpsubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 21, 171, 233, 226], OperandSize::Qword)
}

#[test]
fn vpsubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 658555931, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 109, 172, 233, 188, 145, 27, 196, 64, 39], OperandSize::Qword)
}

