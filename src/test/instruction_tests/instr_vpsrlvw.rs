use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 16, 240], OperandSize::Dword)
}

#[test]
fn vpsrlvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 16, 40], OperandSize::Dword)
}

#[test]
fn vpsrlvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 149, 133, 16, 206], OperandSize::Qword)
}

#[test]
fn vpsrlvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RSI, 2001838842, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 173, 130, 16, 166, 250, 162, 81, 119], OperandSize::Qword)
}

#[test]
fn vpsrlvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 16, 197], OperandSize::Dword)
}

#[test]
fn vpsrlvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1578180302, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 16, 164, 219, 206, 30, 17, 94], OperandSize::Dword)
}

#[test]
fn vpsrlvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 141, 165, 16, 206], OperandSize::Qword)
}

#[test]
fn vpsrlvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1686930833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 141, 167, 16, 60, 221, 145, 133, 140, 100], OperandSize::Qword)
}

#[test]
fn vpsrlvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 16, 231], OperandSize::Dword)
}

#[test]
fn vpsrlvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2076772840, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 16, 44, 85, 232, 9, 201, 123], OperandSize::Dword)
}

#[test]
fn vpsrlvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 229, 205, 16, 211], OperandSize::Qword)
}

#[test]
fn vpsrlvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 63130114, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 189, 194, 16, 20, 197, 2, 74, 195, 3], OperandSize::Qword)
}

