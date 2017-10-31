use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 185, 244], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1280605166, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 185, 140, 72, 238, 123, 84, 76], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 185, 237], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 185, 24], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 221, 185, 215], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 185, 42], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 141, 177, 185, 244], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1026755538, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 181, 133, 185, 20, 69, 210, 11, 51, 61], OperandSize::Qword)
}

