use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 157, 208], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 42649213, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 157, 20, 245, 125, 198, 138, 2], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 157, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 837444063, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 157, 44, 253, 223, 97, 234, 49], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 249, 157, 203], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 1463473414, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 157, 171, 6, 213, 58, 87], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 229, 251, 157, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 173, 138, 157, 28, 192], OperandSize::Qword)
}

