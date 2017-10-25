use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 16, 206], OperandSize::Dword)
}

#[test]
fn vpsrlvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1187652869, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 16, 28, 117, 5, 37, 202, 70], OperandSize::Dword)
}

#[test]
fn vpsrlvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 221, 135, 16, 220], OperandSize::Qword)
}

#[test]
fn vpsrlvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1009068965, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 237, 141, 16, 12, 181, 165, 43, 37, 60], OperandSize::Qword)
}

#[test]
fn vpsrlvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 16, 227], OperandSize::Dword)
}

#[test]
fn vpsrlvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 205262341, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 16, 20, 133, 5, 14, 60, 12], OperandSize::Dword)
}

#[test]
fn vpsrlvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 205, 164, 16, 225], OperandSize::Qword)
}

#[test]
fn vpsrlvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 141, 162, 16, 10], OperandSize::Qword)
}

#[test]
fn vpsrlvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 16, 206], OperandSize::Dword)
}

#[test]
fn vpsrlvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 588535826, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 201, 16, 172, 209, 18, 88, 20, 35], OperandSize::Dword)
}

#[test]
fn vpsrlvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 181, 202, 16, 215], OperandSize::Qword)
}

#[test]
fn vpsrlvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RSI, 421563820, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 205, 206, 16, 166, 172, 141, 32, 25], OperandSize::Qword)
}

