use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 117, 205], OperandSize::Dword)
}

#[test]
fn vpermi2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 1568825986, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 117, 178, 130, 98, 130, 93], OperandSize::Dword)
}

#[test]
fn vpermi2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 165, 137, 117, 205], OperandSize::Qword)
}

#[test]
fn vpermi2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 138, 117, 57], OperandSize::Qword)
}

#[test]
fn vpermi2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 117, 249], OperandSize::Dword)
}

#[test]
fn vpermi2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 117, 36, 137], OperandSize::Dword)
}

#[test]
fn vpermi2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 237, 163, 117, 241], OperandSize::Qword)
}

#[test]
fn vpermi2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 162, 117, 44, 207], OperandSize::Qword)
}

#[test]
fn vpermi2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 117, 205], OperandSize::Dword)
}

#[test]
fn vpermi2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1600615115, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 117, 52, 197, 203, 114, 103, 95], OperandSize::Dword)
}

#[test]
fn vpermi2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 181, 201, 117, 235], OperandSize::Qword)
}

#[test]
fn vpermi2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1623634470, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 197, 117, 28, 245, 38, 178, 198, 96], OperandSize::Qword)
}

