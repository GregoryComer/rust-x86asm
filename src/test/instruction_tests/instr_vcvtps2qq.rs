use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 123, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 682630865, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 123, 28, 245, 209, 30, 176, 40], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 125, 142, 123, 202], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 692496156, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 140, 123, 36, 93, 28, 167, 70, 41], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 123, 226], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 146469055, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 123, 44, 253, 191, 240, 186, 8], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 125, 169, 123, 210], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 170, 123, 10], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 219, 123, 199], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 123, 52, 90], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 125, 187, 123, 212], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 125, 204, 123, 22], OperandSize::Qword)
}

