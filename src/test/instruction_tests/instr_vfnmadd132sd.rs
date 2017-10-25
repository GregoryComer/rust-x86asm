use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 157, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1543384039, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 157, 128, 231, 43, 254, 91], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 157, 214], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 558741422, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 157, 172, 193, 174, 183, 77, 33], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 251, 157, 250], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 157, 42], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 229, 221, 157, 209], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 131, 157, 4, 218], OperandSize::Qword)
}

