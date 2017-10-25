use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 122, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1237122511, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 122, 162, 207, 253, 188, 73], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 125, 139, 122, 240], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 143, 122, 34], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 122, 217], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 122, 4, 82], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 125, 169, 122, 246], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 125, 174, 122, 36, 214], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 159, 122, 211], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1923377715, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 122, 156, 248, 51, 106, 164, 114], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 125, 159, 122, 205], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 122, 18], OperandSize::Qword)
}

